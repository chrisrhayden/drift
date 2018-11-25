use std::env::current_dir;
use std::error::Error;
use std::path::PathBuf;

pub struct Queue {
    current_song: Option<PathBuf>,
    queue: Vec<PathBuf>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            current_song: None,
            queue: vec![],
        }
    }

    pub fn add_current_song(&mut self, current_song: PathBuf) {
        self.current_song = Some(current_song);
    }

    pub fn get_next_songs(&self) -> Result<(), Box<dyn Error>> {
        let current_song = match &self.current_song {
            Some(val) => val,
            None => return Ok(()),
        };

        let base_path: PathBuf = match current_song.parent() {
            Some(val) => val.to_owned(),
            None => return Err(Box::from("no parent dir")),
        };

        for dir in base_path.read_dir() {
            println!("found file: {:?}", dir);
        }

        Ok(())
    }

    pub fn add_song_start_queue(
        &mut self,
        song: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        self.add_current_song(song);
        self.get_next_songs()?;

        Ok(())
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let cur_dir = current_dir()?;
    let q = Queue::new();

    q.get_next_songs()?;

    Ok(())
}
