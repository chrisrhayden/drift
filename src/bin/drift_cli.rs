use std::net::TcpStream;
use std::error::Error;
use std::io::Write;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.3.3:3333")?;

    stream.write("play /home/chris/proj/drift/avil_14th.mp3".as_bytes())?;

    Ok(())
}
