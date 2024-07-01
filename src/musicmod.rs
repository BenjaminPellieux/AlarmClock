pub mod music {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::{Arc, Mutex};
    use tokio::task;

    pub struct MusicPlayer {
        sink: Arc<Mutex<Option<Sink>>>,
    }

    impl MusicPlayer {
        pub fn new() -> Self {
            MusicPlayer {
                sink: Arc::new(Mutex::new(None)),
            }
        }

        pub async fn play_file(&self, file_path: String) {
            let sink = self.create_sink().await;
            let file = BufReader::new(File::open(file_path).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
            let mut current_sink = self.sink.lock().unwrap();
            *current_sink = Some(sink);
        }

        pub async fn play_url(&self, url: String) {
            let sink = self.create_sink().await;
            let response = reqwest::blocking::get(&url).unwrap();
            let source = Decoder::new(BufReader::new(response)).unwrap();
            sink.append(source);
            let mut current_sink = self.sink.lock().unwrap();
            *current_sink = Some(sink);
        }

        pub fn stop(&self) {
            let mut sink = self.sink.lock().unwrap();
            if let Some(sink) = &*sink {
                sink.stop();
            }
            *sink = None;
        }

        async fn create_sink(&self) -> Sink {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink
        }
    }
}
