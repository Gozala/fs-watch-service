extern crate notify;
extern crate futures;
extern crate tokio_service;


use watcher;
use futures::{future, Future, BoxFuture};
use std::path::{PathBuf};
use self::tokio_service::Service;

pub struct Watcher;

pub struct WacthSettings {
  paths: Vec<PathBuf>,
  mode: watcher::Mode,
  delay: watcher::Delay
}

impl Service for Watcher {
  type Request = WacthSettings;
  type Response = watcher::EventStream;
  type Error = ();
  type Future = BoxFuture<Self::Response, Self::Error>;
  fn call (&self, request:Self::Request) -> Self::Future {
    let response = watcher::watch(request.paths, request.mode, request.delay);
    future::ok(response).boxed()
  } 
}
