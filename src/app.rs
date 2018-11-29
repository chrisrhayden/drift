use std::path::PathBuf;

use rodio;

pub struct App {
    pub socket_path: PathBuf,
    pub device_source: rodio::Device,
}

impl App {
    pub fn new(path: Option<PathBuf>) -> Self {
        let socket_path: PathBuf = if let Some(socket_path) = path {
            socket_path
        } else {
            PathBuf::from("/tmp/drift_socket")
        };

        let device_source = rodio::default_output_device().unwrap();

        App {
            socket_path,
            device_source,
        }
    }
}
