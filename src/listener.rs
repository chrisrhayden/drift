use std::error::Error;
use std::io::prelude::*;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;

use regex::Regex;

use events::{evt_dispatch, Event};

pub fn tcp_listener(
    sender: Sender<(Event, Option<TcpStream>)>,
    tcp_handler: TcpListener,
) -> Result<(), Box<dyn Error>> {
    let request_regex: Regex = Regex::new(r".+\r\n\r\n(.+)")?;

    for stream in tcp_handler.incoming() {
        let mut stream_buff = [0; 512];

        let mut stream = stream?;

        stream.read(&mut stream_buff)?;

        let stream_string = String::from_utf8_lossy(&stream_buff);

        let body = match parse_request(&request_regex, &*stream_string)? {
            Some(val) => val,
            None => {
                stream.write(b"http/1.1 400 bad data\r\n\r\n")?;
                continue;
            }
        };

        let evt = evt_dispatch(&body);

        sender.send((evt, Some(stream)))?;
    }

    Ok(())
}

// TODO: consider getting a library to parse the http response
fn parse_request(
    request_regex: &Regex,
    stream_string: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let capture_str = request_regex.captures(&stream_string).unwrap().get(1);

    let stream_msg = match capture_str {
        Some(message) => message
            .as_str()
            .trim_start()
            .trim_right_matches(char::from(0)),
        None => return Ok(None),
    };

    if stream_msg.is_empty() {
        return Ok(None);
    }

    Ok(Some(String::from(stream_msg)))
}
