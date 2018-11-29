use rodio::Sink;

pub struct CurrentStatus {
    pub sink: Option<Sink>,
    pub stoped: bool,
    pub song_name: String,
}

impl CurrentStatus {
    pub fn new() -> Self {
        CurrentStatus {
            sink: None,
            stoped: true,
            song_name: String::from("No song is playing"),
        }
    }

    pub fn set_sink(&mut self, sink: Sink) {
        self.sink = Some(sink);
    }

    pub fn set_sink_empty(&mut self) {
        self.sink = None;
    }

    pub fn set_playing(&mut self) {
        self.stoped = false;
    }

    pub fn set_stoped(&mut self) {
        self.stoped = true;
    }

    pub fn set_song_name(&mut self, song_name: String) {
        self.song_name = song_name;
    }

    pub fn set_status_playing(&mut self, sink: Sink, song_name: String) {
        self.set_sink(sink);
        self.set_song_name(song_name);
        self.set_playing();
    }

    pub fn set_status_stoped(&mut self) {
        self.set_sink_empty();
        self.song_name = String::from("None");
        self.set_stoped();
    }
}
