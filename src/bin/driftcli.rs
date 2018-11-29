extern crate clap;

use std::error::Error;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use clap::{App, Arg};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error {}", err);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let evt_str = get_args();
    let mut stream = UnixStream::connect("/tmp/drift_socket")?;
    stream
        .write_all(evt_str.as_bytes())
        .expect("failed to send string");
    Ok(())
}

fn get_args() -> String {
    let matches = App::new("driftcli")
        .arg(
            Arg::with_name("play song")
                .long("play")
                .short("p")
                .value_name("FILE")
                .takes_value(true)
                .help("play a song"),
        ).arg(
            Arg::with_name("stop song")
                .long("stop")
                .short("s")
                .help("stop the current song, Note: cant restart after"),
        ).arg(
            Arg::with_name("pause song")
                .long("pause")
                .short("P")
                .help("pause a song"),
        ).arg(
            Arg::with_name("toggle pause")
                .long("toggle")
                .short("t")
                .help("toggle pause of a song"),
        ).arg(
            Arg::with_name("kill daemon")
                .long("kill")
                .short("k")
                .help("kill the daemon"),
        ).get_matches();

    // TODO: reformat to match
    if matches.is_present("play song") {
        // we know it exists so unwrap should be fine
        format!("play {}", matches.value_of("play song").unwrap())
    } else if matches.is_present("stop song") {
        String::from("stop")
    } else if matches.is_present("toggle pause") {
        String::from("toggle")
    } else if matches.is_present("pause") {
        String::from("pause")
    } else if matches.is_present("kill daemon") {
        String::from("kill")
    } else {
        String::from("none")
    }
}
