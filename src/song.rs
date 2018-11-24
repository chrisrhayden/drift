use std::error::Error;
use std::fs::File;

use rodio::{Device, Sink};

pub struct Song {
    song_sink: Option<Sink>,
    device: Device,
}

impl Song {
    pub fn new() -> Self {
        Song {
            song_sink: None,
            device: rodio::default_output_device().unwrap(),
        }
    }

    pub fn set_song_sink(&mut self, song_sink: Sink) {
        self.song_sink = Some(song_sink)
    }

    pub fn pause_song(&mut self) {
        if let Some(ref song_sink) = self.song_sink {
            song_sink.pause();
        }
    }

    pub fn pause_toggle(&mut self) {
        if let Some(ref song_sink) = self.song_sink {
            if song_sink.is_paused() {
                song_sink.play();
            } else {
                song_sink.pause();
            }
        }
    }

    pub fn stop_song(&mut self) {
        if let Some(ref song_sink) = self.song_sink {
            song_sink.stop();
        }
    }

    pub fn play_song(&mut self, song: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(song)?;
        let song_sink = rodio::play_once(&self.device, file)?;
        self.set_song_sink(song_sink);

        Ok(())
    }
}
