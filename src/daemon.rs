use std::error::Error;
use std::io::Write;
use std::net::TcpStream;

use events::{Event, Events};
use song::Song;

pub struct Daemon {
    song: Song,
}

impl Daemon {
    pub fn new() -> Self {
        Daemon { song: Song::new() }
    }

    pub fn run(&mut self) -> Result<bool, Box<dyn Error>> {
        let events = Events::new()?;

        loop {
            let evt = events.next()?;
            let mut stream: TcpStream = match evt.1 {
                Some(val) => val,
                None => return Err(Box::from("couldn't get stream")),
            };

            match evt.0 {
                Event::PlaySong(val) => {
                    self.song.play_song(&val)?;
                    stream.write(b"HTTP/1.1 200 playing song\r\n\r\n")?;
                }
                Event::Pause => {
                    self.song.pause_song();
                    stream.write(b"HTTP/1.1 200 pause\r\n\r\n")?;
                }
                Event::PauseToggle => {
                    self.song.pause_toggle();
                    stream.write(b"HTTP/1.1 200 toggle pause\r\n\r\n")?;
                }
                Event::Stop => {
                    self.song.stop_song();
                    stream.write(b"HTTP/1.1 200 stoping song\r\n\r\n")?;
                }
                Event::None => {
                    stream
                        .write(b"HTTP/1.1 200 how did this happen\r\n\r\n")?;
                }
                Event::Kill => {
                    stream.write(b"HTTP/1.1 200 killing server\r\n\r\n")?;
                    break;
                }
                _ => break,
            };
        }

        Ok(true)
    }
}
