extern crate rodio;
extern crate regex;

mod daemon;
mod events;
mod listener;

use daemon::Daemon;

fn main() {
    let mut da: Daemon = Daemon::new();

    if let Err(err) = da.run() {
        eprintln!("Error: {}", err);
    }
}
