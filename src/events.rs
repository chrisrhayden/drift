use std::thread;
use std::sync::mpsc;
use std::error::Error;
use std::time::Duration;

use listener::socket_listener;

#[derive(Debug)]
pub enum Event {
    Stop,
    Pause,
    PauseToggle,
    Kill,
    None,
    Update,
    Show,
    Info(String),
    PlaySong(String),
    AddQueue(String),
    ThreadError(String),
}

pub struct Events {
    pub receiver: mpsc::Receiver<Event>,
    pub sender: mpsc::Sender<Event>,
    pub update_trhead: thread::JoinHandle<Event>,
    pub socket_thread: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Events::make_event_threads()
    }

    pub fn make_event_threads() -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        let socket_thread = thread::spawn(move || {
            if let Err(e) = socket_listener(&tx1) {
                let err_str: String = format!("{}", e).clone();

                match tx1.send(Event::ThreadError(err_str)) {
                    Ok(_) => {}
                    // just print an error if we run in to one sending
                    Err(err) => eprintln!("{}", err),
                }
            }
        });

        let tx2 = tx.clone();
        let update_trhead = thread::spawn(move || loop {
            tx2.send(Event::Update).unwrap();
            thread::sleep(Duration::from_millis(500));
        });

        Ok(Events { receiver: rx, sender: tx, update_trhead, socket_thread })
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
    } else if evt_str.starts_with("add_queue") {
        let to_send = evt_str[10..].trim_start().trim_end().to_string();
        return Event::AddQueue(to_send);
    } else if evt_str == "pause" {
        return Event::Pause;
    } else if evt_str == "stop" {
        return Event::Stop;
    } else if evt_str == "toggle" {
        return Event::PauseToggle;
    } else if evt_str == "kill" {
        return Event::Kill;
    } else if evt_str == "show" {
        return Event::Show;
    } else {
        return Event::None;
    }
}
