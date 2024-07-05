pub mod music {
    use gstreamer::prelude::*;
    use gstreamer::{ElementFactory, Pipeline, MessageView, State};
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::mpsc::{channel, Sender, TryRecvError};
    use std::thread;

    pub enum MusicCommand {
        Stop,
        PlayUrl(String),
        PlayFile(String),
    }

    pub trait Music {
        /// Joue une source de musique.
        ///
        /// # Parameters
        ///
        /// * `source`: Le chemin ou l'URL de la source de musique.
        fn play(&mut self, source: String);

        /// Arrête la lecture de la musique.
        fn stop(&mut self);
    }

    pub struct RadioPlayer {
        sender: Option<Sender<MusicCommand>>,
    }

    pub struct WavPlayer {
        sender: Option<Sender<MusicCommand>>,
    }

    impl RadioPlayer {
        /// Crée une nouvelle instance de `RadioPlayer`.
        ///
        /// # Returns
        ///
        /// Une nouvelle instance de `RadioPlayer`.
        pub fn new() -> Self {
            gstreamer::init().unwrap();
            RadioPlayer {
                sender: None,
            }
        }

        /// Démarre la lecture de la musique en fonction de la commande donnée.
        ///
        /// # Parameters
        ///
        /// * `command`: La commande de lecture de musique (URL ou fichier).
        fn start(&mut self, command: MusicCommand) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
            let (sender, receiver) = channel();
            self.sender = Some(sender.clone());
            let pipeline = Pipeline::new(None);
            let uridecodebin = ElementFactory::make("uridecodebin", None).unwrap();
            let audioconvert = ElementFactory::make("audioconvert", None).unwrap();
            let audioresample = ElementFactory::make("audioresample", None).unwrap();
            let autoaudiosink = ElementFactory::make("autoaudiosink", None).unwrap();

            thread::spawn(move || {
                match command {
                    MusicCommand::PlayUrl(url) => {
                        if pipeline.add_many(&[&uridecodebin, &audioconvert, &audioresample, &autoaudiosink]).is_err() {
                            eprintln!("Failed to add elements to pipeline");
                            return;
                        }
                        if gstreamer::Element::link_many(&[&audioconvert, &audioresample, &autoaudiosink]).is_err() {
                            eprintln!("Failed to link elements in pipeline");
                            return;
                        }

                        uridecodebin.connect_pad_added(move |_element, src_pad| {
                            let sink_pad = audioconvert.static_pad("sink").unwrap();
                            if src_pad.link(&sink_pad).is_err() {
                                eprintln!("Failed to link src pad to sink pad");
                            }
                        });

                        if uridecodebin.set_property("uri", &url).is_err() {
                            eprintln!("Failed to set URI for URL");
                            return;
                        }

                        if pipeline.set_state(State::Playing).is_err() {
                            eprintln!("Failed to set pipeline state to Playing");
                            return;
                        }
                    }
                    _ => {}
                }
                let bus = pipeline.bus().unwrap();

                loop {
                    match receiver.try_recv() {
                        Ok(MusicCommand::Stop) | Err(TryRecvError::Disconnected) => {
                            if pipeline.set_state(State::Null).is_err() {
                                eprintln!("Failed to set pipeline state to Null");
                            }
                            break;
                        }
                        Err(TryRecvError::Empty) => {}
                        _ => {}
                    }

                    for msg in bus.iter_timed(gstreamer::ClockTime::from_seconds(1)) {
                        match msg.view() {
                            MessageView::Eos(..) => break,
                            MessageView::Error(err) => {
                                eprintln!(
                                    "Error from {:?}: {} ({:?})",
                                    err.src().map(|s| s.path_string()),
                                    err.error(),
                                    err.debug()
                                );
                                break;
                            }
                            _ => (),
                        }
                    }
                }
            });
        }
    }

    impl WavPlayer {
        /// Crée une nouvelle instance de `WavPlayer`.
        ///
        /// # Returns
        ///
        /// Une nouvelle instance de `WavPlayer`.
        pub fn new() -> Self {
            WavPlayer {
                sender: None,
            }
        }

        /// Démarre la lecture de la musique en fonction de la commande donnée.
        ///
        /// # Parameters
        ///
        /// * `command`: La commande de lecture de musique (URL ou fichier).
        fn start(&mut self, command: MusicCommand) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
            let (sender, receiver) = channel();
            self.sender = Some(sender.clone());

            thread::spawn(move || {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();

                match command {
                    MusicCommand::PlayFile(file_path) => {
                        println!("[INFO] play music");
                        let file = BufReader::new(File::open(file_path).unwrap());
                        let source = Decoder::new(file).unwrap();
                        sink.append(source);
                    }
                    _ => {}
                }

                loop {
                    match receiver.try_recv() {
                        Ok(MusicCommand::Stop) | Err(TryRecvError::Disconnected) => {
                            sink.stop();
                            break;
                        }
                        Err(TryRecvError::Empty) => {}
                        _ => {}
                    }
                }
            });
        }
    }

    impl Music for RadioPlayer {
        /// Joue une URL de musique.
        ///
        /// # Parameters
        ///
        /// * `url`: L'URL de la musique à jouer.
        fn play(&mut self, url: String) {
            self.start(MusicCommand::PlayUrl(url));
        }

        /// Arrête la lecture de la musique.
        fn stop(&mut self) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
        }
    }

    impl Music for WavPlayer {
        /// Joue un fichier de musique.
        ///
        /// # Parameters
        ///
        /// * `file_path`: Le chemin du fichier de musique à jouer.
        fn play(&mut self, file_path: String) {
            self.start(MusicCommand::PlayFile(file_path));
        }

        /// Arrête la lecture de la musique.
        fn stop(&mut self) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
        }
    }
}
