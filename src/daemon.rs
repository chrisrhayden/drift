use std::error::Error;
use std::fs::File;

// use self::rodio::Decoder;
// use self::rodio::play_once;
// use self::rodio::source::Source;
use rodio::Device;
use rodio::Sink;

use events::{Event, Events};

struct Song {
    song_sink: Option<Sink>,
    device: Device,
}

impl Song {
    fn new() -> Self {
        Song {
            song_sink: None,
            device: rodio::default_output_device().unwrap(),
        }
    }

    fn set_song_sink(&mut self, song_sink: Sink) {
        self.song_sink = Some(song_sink)
    }
}

pub struct Daemon {
    song: Song,
}

impl Daemon {
    pub fn new() -> Self {
        Daemon { song: Song::new() }
    }

    pub fn run(&mut self) -> Result<bool, Box<dyn Error>> {
        println!("daemon is running");

        let events = Events::new();

        loop {
            match events.next()? {
                Event::PlaySong(val) => {
                    self.play_song(&val)?;
                }
                Event::Stop => break,
                Event::Pause => self.pause_song(),
            }
        }

        Ok(true)
    }

    fn play_song(&mut self, song: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(song)?;
        let song_sink = rodio::play_once(&self.song.device, file)?;
        self.song.set_song_sink(song_sink);

        Ok(())
    }

    fn pause_song(&mut self) {
        if let Some(song_sink) = self.song.song_sink {

        }
    }
}
