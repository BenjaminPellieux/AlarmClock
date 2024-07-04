pub mod controller {
    use crate::modelmod::model::{AlarmClock, Horaire, Radio, RadioStation};
    use crate::musicmod::music::{WavPlayer, RadioPlayer, Music};
    use crate::viewmod::view::View;
    use gtk::glib::{Sender};
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    pub struct Controller {
        view: Arc<Mutex<View>>,
        alarms: Arc<Mutex<Vec<AlarmClock>>>,
        current_radio: Arc<Mutex<Radio>>,
        horaire: Arc<Mutex<Horaire>>,
        radio_player: Arc<Mutex<RadioPlayer>>,
        wav_player: Arc<Mutex<WavPlayer>>,
        sender: Sender<()>,
    }

    impl Controller {
        pub fn new(view: Arc<Mutex<View>>, sender: Sender<()>) -> Self {
            let alarms = Arc::new(Mutex::new(Vec::new()));
            let current_radio = Arc::new(Mutex::new(Radio::new()));
            let horaire = Arc::new(Mutex::new(Horaire::new()));
            let radio_player = Arc::new(Mutex::new(RadioPlayer::new()));
            let wav_player = Arc::new(Mutex::new(WavPlayer::new()));

            Controller {
                view,
                alarms,
                current_radio,
                horaire,
                radio_player,
                wav_player,
                sender,
            }
        }

        pub fn update_alarm_id(&self) {
            let mut alarms = self.alarms.lock().unwrap();
            for (i, alarm) in alarms.iter_mut().enumerate() {
                alarm.a_id = i;
            }
        }

        pub fn add_alarm(&self, name: String, hour: u8, minute: u8, second: u8, song: String, is_radio: bool, radio_station: Option<RadioStation>, days: [bool; 7]) {
            let mut alarms = self.alarms.lock().unwrap();
            let new_alarm = AlarmClock::new(alarms.len(), name, hour, minute, second, song, is_radio, radio_station, days);
            alarms.push(new_alarm);
            self.save_alarms().expect("Failed to save alarms");
        }

        pub fn save_alarms(&self) -> std::io::Result<()> {
            let alarms = self.alarms.lock().unwrap();
            let serialized = serde_json::to_string(&*alarms)?;
            let mut file = std::fs::File::create("ser/alarms.json")?;
            file.write_all(serialized.as_bytes())?;
            Ok(())
        }

        pub fn load_alarms(&self) -> std::io::Result<()> {
            let mut file = std::fs::File::open("ser/alarms.json").unwrap_or_else(|_| std::fs::File::create("ser/alarms.json").unwrap());
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            if !contents.is_empty() {
                let alarms: Vec<AlarmClock> = serde_json::from_str(&contents)?;
                let mut alarms_lock = self.alarms.lock().unwrap();
                *alarms_lock = alarms;
            }
            self.update_alarm_id();
            Ok(())
        }

        pub fn delete_alarm(&self, alarm_id: usize) {
            let mut alarms = self.alarms.lock().unwrap();
            if let Some(index) = alarms.iter().position(|alarm| alarm.a_id == alarm_id) {
                if !alarms[index].is_radio {
                    std::fs::remove_file(&alarms[index].song).unwrap_or(());
                }
                alarms.remove(index);
                self.save_alarms().expect("Failed to save alarms");
            }
        }

        pub fn check_alarms(&self) {
            self.load_alarms().expect("Failed to load alarms");

            let current_time = self.horaire.lock().unwrap().clone();
            let day_of_week = chrono::Local::now().weekday().num_days_from_monday() as usize;

            let alarms = self.alarms.lock().unwrap();
            for alarm in alarms.iter() {
                if alarm.active && alarm.to_compare(&current_time, day_of_week) {
                    if alarm.is_radio {
                        self.current_radio.lock().unwrap().selected_radio = alarm.a_radio.clone();
                        self.start_player(true, "".to_string());
                    } else {
                        self.start_player(false, alarm.song.clone());
                    }
                    break;
                }
            }
        }

        fn start_player(&self, is_radio: bool, file_path: String) {
            let radio_player = self.radio_player.clone();
            let wav_player = self.wav_player.clone();
            let current_radio = self.current_radio.clone();
            gtk::glib::MainContext::default().spawn_local(async move {
                if is_radio {
                    if let Some(url) = current_radio.lock().unwrap().get_url() {
                        radio_player.lock().unwrap().play(url);
                    }
                } else {
                    wav_player.lock().unwrap().play(file_path);
                }
            });
        }

        pub fn stop_player(&self) {
            self.radio_player.lock().unwrap().stop();
            self.wav_player.lock().unwrap().stop();
        }
    }
}
