use std::error::Error;
use std::path::PathBuf;

use app::App;
use events::{Event, Events};
use status::CurrentStatus;
use player;

pub struct Daemon {
    current_status: CurrentStatus,
    events: Events,
    app: App,
}

impl Daemon {
    pub fn new() -> Self {
        let this_app = App::new(None);

        Daemon {
            current_status: CurrentStatus::new(),
            events: Events::new(&this_app),
            app: this_app,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let evt = match self.events.next() {
                Err(err) => panic!("{}", err),
                Ok(evt) => evt,
            };

            println!("{:?}", evt);

            match evt {
                Event::Kill => break,
                Event::None => continue,
                Event::Stop => player::stop_song(&mut self.current_status),
                Event::Pause => player::pause_song(&self.current_status.sink),
                Event::TogglePause => {
                    player::toggle_pause_song(&self.current_status.sink)
                }
                Event::Play(song) => {
                    let song = PathBuf::from(song);
                    if let Err(err) = player::play_song(
                        &self.app.device_source,
                        &mut self.current_status,
                        &song,
                    ) {
                        // TODO: think about what should happen here
                        eprintln!("Error playing song: {}", err);
                    };
                }
            }
        }

        Ok(())
    }
}
