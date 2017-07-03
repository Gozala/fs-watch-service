extern crate notify;
extern crate futures;

use notify::{Watcher, RecursiveMode};
use std::sync::mpsc::channel as std_channel;
use futures::sync::mpsc::{Receiver, channel};
use futures::{Sink, Future};
use std::thread;
use std::path::{PathBuf};
use std::time;

pub type Event = notify::DebouncedEvent;
pub type Mode = notify::RecursiveMode;
pub type Delay = time::Duration;
pub type EventStream = Receiver<Event>;

pub fn watch(paths: Vec<PathBuf>,
              mode:Mode,
              delay:Delay) -> EventStream {

  let (mut sink, stream) = channel(1);

  thread::spawn(move || {
    let (mailbox, events) = std_channel();
    let mut watcher = notify::watcher(mailbox, delay).unwrap();
    
    for path in &paths {
      let recursive_mode = match mode {
        RecursiveMode::Recursive => RecursiveMode::Recursive,
        RecursiveMode::NonRecursive => RecursiveMode::NonRecursive
      };
      
      watcher.watch(path, mode).unwrap();
    }

    // println!("watching at {:?} in ${:?} mode and delay of :{:?}", path, recursive_mode, delay);
      // let path = PathBuf::from(Path::new(path));
  
    loop {
      match events.recv() {
        Ok(event) => {
          // println!("received event ${:?}", event);
          sink = sink.send(event).wait().unwrap();
        },
        Err(_) => {
          // println!("failed to receive event");
          sink.close().unwrap();
        }
      }
    }
  });

  
  stream
}