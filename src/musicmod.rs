pub mod music {
    use gstreamer::prelude::*;
    use gstreamer::{ElementFactory, Pipeline, MessageView};
    use std::sync::mpsc;
    use std::thread;

    pub enum MusicCommand {
        Stop,
    }

    pub struct MusicPlayer {
        sender: Option<mpsc::Sender<MusicCommand>>,
    }

    impl MusicPlayer {
        pub fn new() -> Self {
            gstreamer::init().unwrap();
            MusicPlayer { sender: None}
        }

        pub fn start(&mut self, url: String) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
            let (sender, receiver) = mpsc::channel();
            self.sender = Some(sender);

            thread::spawn(move || {
                let pipeline = Pipeline::new(None);
                let uridecodebin = ElementFactory::make("uridecodebin", None).unwrap();
                let audioconvert = ElementFactory::make("audioconvert", None).unwrap();
                let audioresample = ElementFactory::make("audioresample", None).unwrap();
                let autoaudiosink = ElementFactory::make("autoaudiosink", None).unwrap();

                pipeline.add_many(&[&uridecodebin, &audioconvert, &audioresample, &autoaudiosink]).unwrap();
                gstreamer::Element::link_many(&[&audioconvert, &audioresample, &autoaudiosink]).unwrap();

                uridecodebin.set_property("uri", &url).unwrap();

                uridecodebin.connect_pad_added(move |_element, src_pad| {
                    let sink_pad = audioconvert.static_pad("sink").unwrap();
                    src_pad.link(&sink_pad).unwrap();
                });

                pipeline.set_state(gstreamer::State::Playing).unwrap();

                let bus = pipeline.bus().unwrap();

                loop {
                    match receiver.try_recv() {
                        Ok(MusicCommand::Stop) | Err(mpsc::TryRecvError::Disconnected) => {
                            pipeline.set_state(gstreamer::State::Null).unwrap();
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

        pub fn stop(&mut self) {
            if let Some(sender) = &self.sender {
                let _ = sender.send(MusicCommand::Stop);
            }
        }

    }
}
