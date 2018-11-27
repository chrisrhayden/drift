/* this is the lib to connect with drift_cli */

use std::error::Error;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs;


pub fn read_from_socket() -> Result<String, Box<dyn Error>> {
    let mut stream = UnixStream::connect("/tmp/drift_socket")?;

    let mut str_buf = String::new();

    stream.read_to_string(&mut str_buf).unwrap();

    Ok(str_buf)
}

pub fn write_to_socket(write_string: &str) -> Result<(), Box<dyn Error>> {
    let mut stream = UnixStream::connect("/tmp/drift_socket")?;

    stream.write_all(write_string.to_owned().as_bytes()).unwrap();

    Ok(())
}

pub fn get_songs_from_dir(path: PathBuf) -> Result<String, Box<dyn Error>> {
    if path.is_file() {
        let path = path.to_str().unwrap();

        return Err(Box::from(format!("{} is a file", path)));
    }

    let mut queue: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(path)? {
        // let entry = entry?;
        let mut entry_path = entry?.path();

        if entry_path.is_dir() {
            continue;
        }

        queue.push(entry_path);
    }

    let queue_str: String = queue
        .iter()
        .filter_map(|val| match val.to_str() {
            Some(ref val) if val.ends_with("mp3") => Some(String::from(*val)),
            _ => None,
        }).collect::<Vec<String>>()
        .join("\r\n\r\n");

    Ok(format!("add_queue {}", queue_str))
}
