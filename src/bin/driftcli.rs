extern crate drift;
extern crate clap;

use std::error::Error;
use std::path::PathBuf;

use clap::{Arg, App};

use drift::client;

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
        ).arg(
            Arg::with_name("add directory")
                .long("add_dir")
                .short("a")
                .takes_value(true)
                .value_name("DIRECTORY")
                .help("add a directory of songs to the queue"),
        ).arg(
            Arg::with_name("current track")
                .long("current")
                .short("C")
                .takes_value(false)
                .help("display the current track and info"),
        ).get_matches();

    let mut write_str: String = String::new();

    if let Some(song_path) = arg_match.value_of("play") {
        write_str = format!("play {}", song_path);
    } else if let Some(dir_str) = arg_match.value_of("add directory") {
        write_str = client::get_songs_from_dir(PathBuf::from(dir_str))?;
    } else if arg_match.is_present("pause") {
        write_str = String::from("pause");
    } else if arg_match.is_present("toggle pause") {
        write_str = String::from("toggle");
    } else if arg_match.is_present("stop") {
        write_str = String::from("stop");
    } else if arg_match.is_present("current track") {
        write_str = String::from("current");
    } else if arg_match.is_present("show") {
        write_str = String::from("show");
    }

    if write_str == "show" {
        client::write_to_socket(&write_str)?;

        client::read_from_socket()?;
    } else if !write_str.is_empty() {
        client::write_to_socket(&write_str)?;
    }

    Ok(())
}
