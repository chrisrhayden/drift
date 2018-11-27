use std::error::Error;
use std::path::PathBuf;

use queue::Queue;

use events::{Event, Events};
use song::Song;

pub struct Daemon {
    song: Song,
    queue: Queue,
}

impl Daemon {
    pub fn new() -> Self {
        Daemon {
            song: Song::new(),
            queue: Queue::new(),
        }
    }

    pub fn run(&mut self) -> Result<bool, Box<dyn Error>> {
        let events = Events::new()?;

        loop {
            let evt = events.next()?;

            println!("evt {:?}", evt);

            match evt {
                Event::PlaySong(val) => {
                    self.song.play_song(&PathBuf::from(val))?;
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
                Event::AddQueue(queue_str) => {
                    let song = self.queue.add_to_queue(queue_str)?;

                    if !self.song.is_playing() {
                        self.song.play_song(&song)?;
                    };
                }
                Event::Update => {
                    if !self.song.is_playing() {
                        let song = match self.queue.get_next_song() {
                            Some(song) => song,
                            None => continue,
                        };

                        self.song.play_song(&song)?;
                    }
                }
                Event::Show => {
                    let info_string = self.song.info_string(&self.queue);

                    events.sender.send(Event::Info(info_string))?;
                }
                _ => continue,
            };
        }

        Ok(true)
    }
}
