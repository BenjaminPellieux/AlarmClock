pub mod music {
    use gstreamer::prelude::*;
    use gstreamer::{ElementFactory, Pipeline, MessageView, State};
    use std::thread;
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::mpsc::{channel, Sender, TryRecvError};

    pub enum MusicCommand {
        Stop,
        Play(String),
    }

    pub struct WavPlayer {
        sender: Option<Sender<MusicCommand>>,
    }

    pub struct RadioPlayer {
        sender: Option<Sender<MusicCommand>>,
    }

    impl RadioPlayer {
        pub fn new() -> Self {
            gstreamer::init().unwrap();
            RadioPlayer { sender: None }
        }

        pub fn start(&mut self, command: MusicCommand) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
            let (sender, receiver) = channel();
            self.sender = Some(sender.clone());

            thread::spawn(move || {
                let pipeline = Pipeline::new(None);
                let uridecodebin = ElementFactory::make("uridecodebin", None).unwrap();
                let audioconvert = ElementFactory::make("audioconvert", None).unwrap();
                let audioresample = ElementFactory::make("audioresample", None).unwrap();
                let autoaudiosink = ElementFactory::make("autoaudiosink", None).unwrap();

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

                match command {
                    MusicCommand::Play(url) => {
                        println!("[INFO] play radio");
                        if uridecodebin.set_property("uri", &url).is_err() {
                            eprintln!("Failed to set URI for URL");
                            return;
                        }
                    }
                    _ => {}
                }

                if pipeline.set_state(State::Playing).is_err() {
                    eprintln!("Failed to set pipeline state to Playing");
                    return;
                }

                let bus = pipeline.bus().unwrap();

                loop {
                    match receiver.try_recv() {
                        Ok(MusicCommand::Play(_)) => {},
                        Ok(MusicCommand::Stop) | Err(TryRecvError::Disconnected) => {
                            println!("[INFO] stop radio");
                            if pipeline.set_state(State::Null).is_err() {
                                eprintln!("Failed to set pipeline state to Null");
                            }
                            break;
                        }
                        Err(TryRecvError::Empty) => {}
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

        pub fn play_url(&mut self, url: String) {
            self.start(MusicCommand::Play(url));
        }

        pub fn stop(&mut self) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
        }
    }

    
    impl WavPlayer {
        pub fn new() -> Self {
            WavPlayer { sender: None }
        }

        pub fn start(&mut self, command: MusicCommand) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
            let (sender, receiver) = channel();
            self.sender = Some(sender.clone());

            thread::spawn(move || {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();


                match command {
                    MusicCommand::Play(file_path) => {
                        println!("[INFO] play music");
                        let file = BufReader::new(File::open(file_path).unwrap());
                        let source = Decoder::new(file).unwrap();
                        sink.append(source);
                    }
                    _ => {},
                }
                loop {
                    match receiver.recv().unwrap() {
                        MusicCommand::Play(_) => {},
                        MusicCommand::Stop => {
                            println!("[INFO] stop music");
                            sink.stop();
                            break;
                        }
                    }
                }
            });
        }

        pub fn play_file(&mut self, file_path: String) {
            self.start(MusicCommand::Play(file_path));
        }

        pub fn stop(&mut self) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
        }
    }
}
