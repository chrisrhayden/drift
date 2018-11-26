extern crate drift;

use drift::daemon::Daemon;

fn main() {
    let mut da: Daemon = Daemon::new();

    if let Err(err) = da.run() {
        eprintln!("Error: {}", err);
    }
}
