/* this is the lib to connect with drift_cli */

use std::error::Error;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;


pub fn write_to_socket(write_string: &str) -> Result<(), Box<dyn Error>> {
    let mut stream = UnixStream::connect("/tmp/drift_socket")?;

    stream.write_all(write_string.to_owned().as_bytes()).unwrap();

    Ok(())
}
