extern crate drift;
extern crate clap;

use std::error::Error;

use clap::{Arg, App};

use drift::client::write_to_socket;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let arg_match = App::new("driftcli")
        .arg(
            Arg::with_name("play")
                .long("play")
                .short("p")
                .value_name("FILE")
                .takes_value(true)
                .help("play a given song"),
        ).arg(
            Arg::with_name("toggle pause")
                .long("toggle")
                .short("t")
                .takes_value(false)
                .help("toggle pause for the playing song"),
        ).arg(
            Arg::with_name("stop")
                .long("stop")
                .short("s")
                .takes_value(false)
                .help("stop the playing song"),
        ).get_matches();

    let mut write_str: String = String::new();

    if let Some(song_path) = arg_match.value_of("play") {
        write_str = format!("play {}", song_path);

    } else if arg_match.is_present("toggle pause") {
        write_str = String::from("toggle");
    } else if arg_match.is_present("stop") {
        write_str = String::from("stop");
    }

    if !write_str.is_empty() {
        write_to_socket(&write_str)?;
    }

    Ok(())
}
