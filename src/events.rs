use std::sync::mpsc;
use std::thread;
use std::os::unix::net::UnixListener;
use std::fs;

use std::io::Read;

use app::App;

#[derive(Debug)]
pub enum Event {
    Kill,
    None,
    Stop,
    Pause,
    TogglePause,
    Play(String),
}

// TODO: remove this
#[allow(dead_code)]
pub struct Events {
    receiver: mpsc::Receiver<Event>,
    sender: mpsc::Sender<Event>,
    unix_stream: UnixListener,
}

impl Events {
    pub fn new(app: &App) -> Self {
        let (sender, receiver, unix_stream) =
            Events::make_listener_thread(&app);

        Events {
            receiver,
            sender,
            unix_stream,
        }
    }

    fn make_listener_thread(
        app: &App,
    ) -> (mpsc::Sender<Event>, mpsc::Receiver<Event>, UnixListener) {
        let socket_path = app.socket_path.to_owned();

        let (tx, rx) = mpsc::channel();

        if socket_path.exists() {
            fs::remove_file(&socket_path).unwrap();
        }

        // get a handle of a socket
        let stream = match UnixListener::bind(&socket_path) {
            Ok(stream) => stream,
            Err(_) => {
                let path = socket_path.to_string_lossy();
                panic!("couldn't connect to socket_path {}", path);
            }
        };

        // make a clone to send to the thread
        let stream1 = match stream.try_clone() {
            Err(_) => panic!("couldn't clone stream"),
            Ok(stream) => stream,
        };

        let tx1 = tx.clone();

        thread::spawn(move || {
            for client in stream1.incoming() {
                let mut client = client.unwrap();

                let mut buf_string = String::new();

                client
                    .read_to_string(&mut buf_string)
                    .expect("received string with invalid utf8 characters");

                let buf_string = buf_string.trim_start().trim_end();

                let evt = event_dispatch(&buf_string);

                tx1.send(evt).expect("couldn't send event");
            }

            tx1.send(Event::Kill).unwrap();
        });

        (tx, rx, stream)
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }
}

fn event_dispatch(evt_str: &str) -> Event {
    if evt_str == "pause" {
        Event::Pause
    } else if evt_str == "toggle" {
        Event::TogglePause
    } else if evt_str == "stop" {
        Event::Stop
    } else if evt_str == "kill" {
        Event::Kill
    } else if evt_str.starts_with("play") {
        // cut off play and the space, strip newlines and other fluff
        let to_send: String = evt_str[5..].trim_start().trim_end().to_string();
        Event::Play(to_send)
    } else {
        Event::None
    }
}
