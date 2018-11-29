use std::fs::File;
use std::path::PathBuf;
use std::error::Error;

use status::CurrentStatus;

use rodio::{Device, Sink};

pub fn play_song(
    device_source: &Device,
    current_status: &mut CurrentStatus,
    path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let p_str = path.to_string_lossy();
    if !path.exists() {
        return Err(Box::from(format!("file not found: {}", p_str)));
    }

    let song_file = File::open(path).unwrap();
    let song_sink = rodio::play_once(device_source, song_file)?;

    current_status.set_status_playing(song_sink, String::from(p_str));

    Ok(())
}

pub fn pause_song(song: &Option<Sink>) {
    if let Some(song) = song {
        song.pause();
    }
}

pub fn toggle_pause_song(song: &Option<Sink>) {
    if let Some(song) = song {
        match song.is_paused() {
            true => song.play(),
            false => song.pause(),
        }
    }
}

pub fn stop_song(current_status: &mut CurrentStatus) {
    if let Some(sink) = &current_status.sink {
        sink.stop();
    }

    current_status.set_status_stoped();
}
