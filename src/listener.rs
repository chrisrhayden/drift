use std::fs;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::sync::mpsc::Sender;
use std::os::unix::net::UnixListener;

use events::{evt_dispatch, Event};

pub fn socket_listener(sender: Sender<Event>) -> Result<(), Box<dyn Error>> {
    let socket_path = Path::new("/tmp/drift_socket");

    if socket_path.exists() {
        match socket_path.to_str() {
            Some(val) => {
                fs::remove_file(&socket_path)
                    .expect(&format!("couldn't delete file: {}", val));
            }
            None => {}
        };
    }

    let stream = match UnixListener::bind(&socket_path) {
        Ok(stream) => stream,
        Err(_) => panic!("failed to bind socket"),
    };

    for client in stream.incoming() {
        let mut client = client.unwrap();

        let mut buf_str = String::new();

        client
            .read_to_string(&mut buf_str)
            .expect("couldn't red to string");

        let new_buf = buf_str.trim_start().trim_end();

        let evt = evt_dispatch(&new_buf);

        sender.send(evt).expect("couldn't send event");
    }

    sender.send(Event::None).expect("failed to send event");

    Ok(())
}
