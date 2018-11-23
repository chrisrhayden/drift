use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;

use regex::Regex;

use events::Event;

pub fn tcp_listener(sender: Sender<Event>) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.3.3:3333")?;

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let to_send: String = handle_connection(&mut stream)?;

        sender.send(Event::PlaySong(to_send))?;
    }

    Ok(())
}

// TODO: consider getting a library to parse the http response
fn handle_connection(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let request_regex: Regex = Regex::new(r".+\r\n\r\n(.+)").unwrap();

    let mut stream_buff = [0; 512];

    stream.read(&mut stream_buff).unwrap();

    let stream_string = String::from_utf8_lossy(&stream_buff);

    let capture_str = request_regex.captures(&*stream_string).unwrap().get(1);

    let stream_msg = match capture_str {
        Some(message) => message.as_str(),
        None => return Err(Box::from("did not pass a message")),
    };

    stream
        .write("HTTP/1.1 201 Created\r\n\r\n".as_bytes())
        .unwrap();
    stream.flush().unwrap();

    Ok(String::from(stream_msg))
}
