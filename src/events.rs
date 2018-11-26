use std::thread;
use std::sync::mpsc;
use std::error::Error;

use listener::socket_listener;

#[derive(Debug)]
pub enum Event {
    PlaySong(String),
    Stop,
    Pause,
    PauseToggle,
    Kill,
    None,
    ThreadError(String),
}

pub struct Events {
    receiver: mpsc::Receiver<Event>,
}

impl Events {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Events::make_event_threads()
    }

    pub fn make_event_threads() -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel();

        {
            thread::spawn(move || {
                // this is trash
                let tx1 = tx.clone();
                if let Err(e) = socket_listener(tx1) {
                    let err_str: String = format!("{}", e).clone();

                    match tx.send(Event::ThreadError(err_str)) {
                        Ok(_) => {}
                        // just print an error if we run in to one sending
                        Err(err) => eprintln!("{}", err),
                    }
                }
            });
        };

        Ok(Events { receiver: rx })
    }

    pub fn next(&self) -> Result<Event, Box<dyn Error>> {
        match self.receiver.recv() {
            Ok(val) => Ok(val),
            // this is bad and i feel bad
            Err(e) => Err(Box::from(format!("{}", e))),
        }
    }
}

pub fn evt_dispatch(evt_str: &str) -> Event {
    if evt_str.starts_with("play") {
        // slice off first 5 characters, trim characters from end of line
        let to_send: String = evt_str[5..].trim_start().trim_end().to_string();
        return Event::PlaySong(to_send);
    } else if evt_str == "pause" {
        return Event::Pause;
    } else if evt_str == "stop" {
        return Event::Stop;
    } else if evt_str == "toggle" {
        return Event::PauseToggle;
    } else if evt_str == "kill" {
        return Event::Kill;
    } else {
        return Event::None;
    }
}
