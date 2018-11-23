use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;

use regex::Regex;

use events::Event;

pub fn tcp_listener(sender: Sender<Event>) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.3.3:3333")?;
    let request_regex: Regex =
        Regex::new(r".+\r\n\r\n(.+)").expect("couldn't make regex?");

    for stream in listener.incoming() {
        let mut stream = stream?;

        let body: String = match parse_request(&request_regex, &mut stream)? {
            Some(val) => val,
            None => continue,
        };

        if body.starts_with("play") {
            // trim characters from end of line, slice off first 5 characters
            let to_send: String = body.trim_right()[5..].to_string();
            sender.send(Event::PlaySong(to_send))?;
        } else if body.starts_with("stop") {
            sender.send(Event::Stop)?;
        } else if body.starts_with("pause") {
            sender.send(Event::Pause)?;
        }
    }

    Ok(())
}

// TODO: consider getting a library to parse the http response
fn parse_request(
    request_regex: &Regex,
    stream: &mut TcpStream,
) -> Result<Option<String>, Box<dyn Error>> {
    let mut stream_buff = [0; 512];

    stream.read(&mut stream_buff)?;

    let stream_string = String::from_utf8_lossy(&stream_buff);
    let capture_str = request_regex.captures(&*stream_string).unwrap().get(1);

    let stream_msg = match capture_str {
        Some(message) => message.as_str().trim_right_matches(char::from(0)),
        None => return Err(Box::from("did not pass a message")),
    };

    if stream_msg.is_empty() {
        stream.write(b"HTTP/1.1 400 no body\r\n\r\n").unwrap();
        stream.flush().unwrap();

        return Ok(None);
    }

    stream.write(b"HTTP/1.1 201 Created\r\n\r\n").unwrap();
    stream.flush().unwrap();

    Ok(Some(String::from(stream_msg)))
}
