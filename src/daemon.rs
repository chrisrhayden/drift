use std::error::Error;

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

            println!("evt {:?}", evt);

            match evt {
                Event::PlaySong(val) => {
                    self.song.play_song(&val)?;
                }
                Event::Pause => {
                    self.song.pause_song();
                }
                Event::PauseToggle => {
                    self.song.pause_toggle();
                }
                Event::Stop => {
                    self.song.stop_song();
                }
                Event::Kill => {
                    break;
                }
                Event::ThreadError(err) => {
                    return Err(Box::from(err));
                }
                Event::None => {}
            };
        }

        Ok(true)
    }
}
