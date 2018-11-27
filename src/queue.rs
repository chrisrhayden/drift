use std::error::Error;
use std::path::PathBuf;

pub struct Queue {
    pub current_song: Option<PathBuf>,
    pub queue: Vec<PathBuf>,
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

    pub fn add_to_queue(
        &mut self,
        queue_str: String,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let to_convert = queue_str.split("\r\n\r\n").collect::<Vec<_>>();

        let mut queue_vec: Vec<PathBuf> =
            to_convert.iter().map(|val| PathBuf::from(val)).collect();

        let current_song: PathBuf = queue_vec.pop().unwrap();

        self.queue.append(&mut queue_vec);

        Ok(current_song)
    }

    pub fn get_next_song(&mut self) -> Option<PathBuf> {
        let next_song = match self.queue.pop() {
            Some(song) => Some(song),
            None => return None,
        };

        self.current_song = next_song.clone();

        next_song
    }
}
