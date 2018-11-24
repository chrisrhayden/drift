use std::error::Error;
use std::sync::mpsc;
use std::thread;

use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use listener::tcp_listener;

pub enum Event {
    PlaySong(String),
    Stop,
    Pause,
    PauseToggle,
    Kill,
    None,
    ThreadError(Arc<String>),
}

pub struct Events {
    receiver: mpsc::Receiver<(Event, Option<TcpStream>)>,
}

impl Events {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Events::make_event_threads()
    }

    pub fn make_event_threads() -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel();

        let tcp_handler = TcpListener::bind("127.0.3.3:3333").unwrap();

        {
            thread::spawn(move || {
                // this is trash
                let tx1 = tx.clone();
                if let Err(e) = tcp_listener(tx1, tcp_handler) {
                    let err_str: String = format!("{}", e);

                    match tx
                        .send((Event::ThreadError(Arc::from(err_str)), None))
                    {
                        Ok(_) => {}
                        Err(err) => println!("{}", err),
                    }
                }
            });
        };

        Ok(Events { receiver: rx })
    }

    pub fn next(&self) -> Result<(Event, Option<TcpStream>), Box<dyn Error>> {
        match self.receiver.recv() {
            Ok(val) => Ok(val),
            // TODO: graceful shutdown here i think
            // this is bad and i feel bad
            Err(e) => Err(Box::from(format!("{}", e))),
        }
    }
}

pub fn evt_dispatch(evt_str: &str) -> Event {
    if evt_str.starts_with("play") {
        // trim characters from end of line, slice off first 5 characters
        let to_send: String = evt_str[5..].trim_start().trim_end().to_string();
        return Event::PlaySong(to_send);
    } else if evt_str.starts_with("stop") {
        return Event::Stop;
    } else if evt_str.starts_with("pause") {
        return Event::Pause;
    } else if evt_str == "toggle" {
        return Event::PauseToggle;
    } else if evt_str.starts_with("kill") {
        return Event::Kill;
    } else {
        return Event::None;
    }
}
