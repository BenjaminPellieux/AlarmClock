pub mod music {
    use gstreamer::prelude::*;
    use gstreamer::{ElementFactory, Pipeline, MessageView, State};
    use std::sync::{Arc, mpsc};
    use std::thread;

    pub enum MusicCommand {
        Stop,
        PlayUrl(String),
        PlayFile(String),
    }

    pub struct MusicPlayer {
        sender: Option<mpsc::Sender<MusicCommand>>,
    }

    impl MusicPlayer {
        pub fn new() -> Self {
            gstreamer::init().unwrap();
            MusicPlayer { sender: None }
        }

        pub fn start(&mut self, command: MusicCommand) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
            let (sender, receiver) = mpsc::channel();
            self.sender = Some(sender.clone());

            thread::spawn(move || {
                let pipeline = Pipeline::new(None);
                let uridecodebin = ElementFactory::make("uridecodebin", None).unwrap();
                let audioconvert = ElementFactory::make("audioconvert", None).unwrap();
                let audioresample = ElementFactory::make("audioresample", None).unwrap();
                let autoaudiosink = ElementFactory::make("autoaudiosink", None).unwrap();

                println!("[DEBUG] Adding elements to pipeline");
                if pipeline.add_many(&[&uridecodebin, &audioconvert, &audioresample, &autoaudiosink]).is_err() {
                    eprintln!("Failed to add elements to pipeline");
                    return;
                }
                if gstreamer::Element::link_many(&[&audioconvert, &audioresample, &autoaudiosink]).is_err() {
                    eprintln!("Failed to link elements in pipeline");
                    return;
                }

                let audioconvert_clone = audioconvert.clone();
                uridecodebin.connect_pad_added(move |_element, src_pad| {
                    let sink_pad = audioconvert_clone.static_pad("sink").unwrap();
                    if src_pad.link(&sink_pad).is_err() {
                        eprintln!("Failed to link src pad to sink pad");
                    }
                });

                match command {
                    MusicCommand::PlayUrl(url) => {
                        println!("[DEBUG] Playing URL: {}", url);
                        if let Err(err) = uridecodebin.set_property("uri", &url) {
                            eprintln!("Failed to set URI for URL: {:?}", err);
                            return;
                        }
                    }
                    MusicCommand::PlayFile(file_path) => {
                        let filesrc = ElementFactory::make("filesrc", None).unwrap();
                        let decodebin = ElementFactory::make("decodebin", None).unwrap();
                        println!("[DEBUG] Playing File: {}", file_path);
                        if pipeline.add_many(&[&filesrc, &decodebin]).is_err() {
                            eprintln!("Failed to add file elements to pipeline");
                            return;
                        }
                        if filesrc.set_property("location", &file_path).is_err() {
                            eprintln!("Failed to set location for file");
                            return;
                        }
                        if gstreamer::Element::link_many(&[&filesrc, &decodebin]).is_err() {
                            eprintln!("Failed to link file elements in pipeline");
                            return;
                        }

                        let audioconvert_clone = audioconvert.clone();
                        decodebin.connect_pad_added(move |_element, src_pad| {
                            let sink_pad = audioconvert_clone.static_pad("sink").unwrap();
                            if src_pad.link(&sink_pad).is_err() {
                                eprintln!("Failed to link src pad to sink pad for file");
                            }
                        });
                    }
                    _ => {}
                }

                if let Err(err) = pipeline.set_state(State::Playing) {
                    eprintln!("Failed to set pipeline state to Playing: {:?}", err);
                    return;
                }

                let bus = pipeline.bus().unwrap();

                loop {
                    match receiver.try_recv() {
                        Ok(MusicCommand::PlayFile(_)) => { println!("Playing File"); }
                        Ok(MusicCommand::PlayUrl(_)) => { println!("Playing URL"); }
                        Ok(MusicCommand::Stop) | Err(mpsc::TryRecvError::Disconnected) => {
                            if let Err(err) = pipeline.set_state(State::Null) {
                                eprintln!("Failed to set pipeline state to Null: {:?}", err);
                            }
                            break;
                        }
                        Err(mpsc::TryRecvError::Empty) => {}
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
            self.start(MusicCommand::PlayUrl(url));
        }

        pub fn play_file(&mut self, file_path: String) {
            self.start(MusicCommand::PlayFile(file_path));
        }

        pub fn stop(&mut self) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
        }
    }
}
