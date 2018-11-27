use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use rodio::{Device, Sink};

use queue::Queue;

pub struct Song {
    song_sink: Option<Sink>,
    device: Device,
}

impl Song {
    pub fn new() -> Self {
        Song {
            song_sink: None,
            device: rodio::default_output_device()
                .expect("couldn't get device"),
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

    pub fn play_song(&mut self, song: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::open(song)?;
        let song_sink = rodio::play_once(&self.device, file)?;
        self.set_song_sink(song_sink);

        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        match self.song_sink {
            Some(ref sink) => !sink.empty(),
            _ => false,
        }
    }

    // YES i dont need as fucking option
    pub fn info_string(&self, queue_struct: &Queue) -> String {
        if self.is_playing() {
            let mut formated = String::new();

            if let Some(song) = &queue_struct.current_song {
                // there is a song that is playing there for its on the disk
                let current_song_file = PathBuf::from(song)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();

                let next_songs = queue_struct
                    .queue
                    .iter()
                    .map(|song| match song.file_name() {
                        Some(val) => val.to_string_lossy().into_owned(),
                        None => String::from("no song?"),
                    }).collect::<Vec<String>>()
                    .join("\n");

                formated = format!(
                    "current song: {} the queue: {}",
                    current_song_file, next_songs
                );
            }

            formated
        } else {
            String::from("nothing is playing")
        }
    }
}
