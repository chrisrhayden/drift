use std::error::Error;
use std::sync::mpsc;
use std::thread;

use listener::tcp_listener;

// PlaySong(Box<Vec<String>>),
pub enum Event {
    PlaySong(String),
    Stop,
    Pause,
}

pub struct Events {
    receiver: mpsc::Receiver<Event>,
}

impl Events {
    pub fn new() -> Self {
        Events::make_event_threads()
    }

    pub fn make_event_threads() -> Events {
        let (tx, rx) = mpsc::channel();

        {
            let tx = tx.clone();

            thread::spawn(|| {
                if let Err(e) = tcp_listener(tx) {
                    eprintln!("{}", e);
                }
            });
        };

        Events { receiver: rx }
    }

    pub fn next(&self) -> Result<Event, Box<dyn Error>> {
        match self.receiver.recv() {
            Ok(val) => Ok(val),
            // TODO: graceful shutdown here i think
            // this is bad and i feel bad
            Err(e) => Err(Box::from(format!("{}", e))),
        }
    }
}
