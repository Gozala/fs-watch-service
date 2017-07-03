extern crate notify;
extern crate futures;
mod watcher;
mod service;

use watcher::watch;
use std::time::Duration;
use notify::RecursiveMode;
use futures::{Stream, Future};
use futures::future::ok;
use std::path::{PathBuf};

fn main() {
    let inbox = watch(vec![PathBuf::from("/Users/Gozala/Projects/watch-stream")],
                      RecursiveMode::Recursive,
                      Duration::from_secs(1));

    println!("watching");


    inbox.for_each(|item| ok(println!("{:?}", item))).wait().unwrap();
}