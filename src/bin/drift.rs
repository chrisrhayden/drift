extern crate drift;

use std::error::Error;

use drift::daemon::Daemon;

fn main() {
    if let Err(err) = run() {
        eprintln!("Server Error: {}", err);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut da = Daemon::new();

    da.run()
}
