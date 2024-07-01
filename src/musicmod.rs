pub mod music{

    use gstreamer::prelude::*;
    use gstreamer::{ElementFactory, Pipeline, MessageView, State};
    use std::sync::{Arc, Mutex};
    use tokio::task;

    pub struct MusicPlayer {
        pipeline: Arc<Mutex<Option<Pipeline>>>,
    }

    impl MusicPlayer {
        pub fn new() -> Self {
            gstreamer::init().unwrap();
            MusicPlayer {
                pipeline: Arc::new(Mutex::new(None)),
            }
        }

        pub async fn play_file(&mut self, file_path: String) {
            let pipeline = self.create_pipeline(&("file://".to_string() + &file_path));
            self.start_pipeline(pipeline).await;
        }

        pub async fn play_url(&mut self, url: String) {
            let pipeline = self.create_pipeline(&url);
            self.start_pipeline(pipeline).await;
        }

        fn create_pipeline(&self, uri: &str) -> Pipeline {
            let pipeline = Pipeline::new(None);

            let uridecodebin = ElementFactory::make("uridecodebin", None).unwrap();
            let audioconvert = ElementFactory::make("audioconvert", None).unwrap();
            let audioresample = ElementFactory::make("audioresample", None).unwrap();
            let autoaudiosink = ElementFactory::make("autoaudiosink", None).unwrap();

            pipeline.add_many(&[&uridecodebin, &audioconvert, &audioresample, &autoaudiosink]).unwrap();
            gstreamer::Element::link_many(&[&audioconvert, &audioresample, &autoaudiosink]).unwrap();

            uridecodebin.set_property("uri", &uri).unwrap();

            uridecodebin.connect_pad_added(move |_element, src_pad| {
                let sink_pad = audioconvert.static_pad("sink").unwrap();
                src_pad.link(&sink_pad).unwrap();
            });

            pipeline
        }

        async fn start_pipeline(&mut self, pipeline: Pipeline) {
            let pipeline_clone = pipeline.clone();
            let pipeline_arc = self.pipeline.clone();

            task::spawn_blocking(move || {
                pipeline_clone.set_state(State::Playing).unwrap();

                let bus = pipeline_clone.bus().unwrap();
                for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
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

                pipeline_clone.set_state(State::Null).unwrap();
            })
            .await
            .unwrap();

            let mut current_pipeline = pipeline_arc.lock().unwrap();
            *current_pipeline = Some(pipeline);
        }

        pub fn stop(&mut self) {
            let mut pipeline = self.pipeline.lock().unwrap();
            if let Some(ref pipeline) = *pipeline {
                pipeline.set_state(State::Null).unwrap();
            }
            *pipeline = None;
        }
    }
}