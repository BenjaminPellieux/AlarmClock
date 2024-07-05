pub mod controller {
    use crate::modelmod::model::{AlarmClock, Horaire, Radio, RadioStation};
    use crate::musicmod::music::{WavPlayer, RadioPlayer, Music};
    use crate::viewmod::view::View;
    use std::sync::{Arc, Mutex};
    use chrono::prelude::*;
    use std::fs::{File, remove_file};
    use std::io::{self, Read, Write};
    use std::{thread, time};

    #[derive(Clone, Debug)]
    pub struct Controller {
        view: Arc<Mutex<View>>,
        pub alarms: Vec<AlarmClock>,
        current_radio: Arc<Mutex<Radio>>,
        pub horaire: Arc<Mutex<Horaire>>,
        radio_player: Arc<Mutex<RadioPlayer>>,
        wav_player: Arc<Mutex<WavPlayer>>,
        player_status: bool,
    }

    impl Controller {
        pub fn new(view: Arc<Mutex<View>>) -> Self {
            let alarms: Vec<AlarmClock> = Vec::new();
            let current_radio: Arc<Mutex<Radio>> = Arc::new(Mutex::new(Radio::new()));
            let horaire: Arc<Mutex<Horaire>> = Arc::new(Mutex::new(Horaire::new()));
            let radio_player: Arc<Mutex<RadioPlayer>> = Arc::new(Mutex::new(RadioPlayer::new()));
            let wav_player: Arc<Mutex<WavPlayer>> = Arc::new(Mutex::new(WavPlayer::new()));
            let player_status = false;
            Controller {
                view,
                alarms,
                current_radio,
                horaire,
                radio_player,
                wav_player,
                player_status,
            }
        }

        fn update_alarm_id(&mut self) {
            let mut count: usize = 0;
            for alarm in self.alarms.iter_mut() {
                alarm.a_id = count;
                count += 1;
            }
        }

        fn add_alarm(&mut self, name: String, hour: u8, minute: u8, second: u8, song: String, days: [bool; 7]) {
            println!("[DEBUG] New alarm name clicked: {}", name);
            let tmp_alarm: AlarmClock;
            if song.is_empty() && self.current_radio.lock().unwrap().selected_radio.is_none() {
                println!("[ERROR] No song URL & No radio selected");
                return;
            } else if !song.is_empty() {
                println!("[DEBUG] New alarm song: {}", song);
                tmp_alarm = AlarmClock::new(self.alarms.len(), name, hour, minute, second, song, false, None, days);
                self.alarms.push(tmp_alarm);
            } else {
                println!("[DEBUG] New alarm radio");
                tmp_alarm = AlarmClock::new(self.alarms.len(), name, hour, minute, second, "".to_string(), true, self.current_radio.lock().unwrap().selected_radio.clone(), days.clone());
                self.alarms.push(tmp_alarm);
            }
            self.save_alarms().expect("Failed to save alarms");
            self.update_alarm_id();
            self.update_alarms_display();
        }

        pub fn save_alarms(&mut self) -> std::io::Result<()> {
            let alarms = &self.alarms;
            let serialized = serde_json::to_string(&*alarms)?;
            let mut file = std::fs::File::create("ser/alarms.json")?;
            file.write_all(serialized.as_bytes())?;
            Ok(())
        }

        pub fn load_alarms(&mut self) -> io::Result<()> {
            let mut file: File = File::open("ser/alarms.json").unwrap_or_else(|_| File::create("ser/alarms.json").unwrap());
            let mut contents: String = String::new();
            file.read_to_string(&mut contents)?;
            if !contents.is_empty() {
                let alarms: Vec<AlarmClock> = serde_json::from_str(&contents)?;
                self.alarms = alarms.to_vec();
            } else {
                self.alarms = Vec::new();
                println!("[INFO] No alarms found");
            }
            self.update_alarm_id();
            self.update_alarms_display();
            Ok(())
        }

        pub fn delete_alarm(&mut self, alarm_id: usize) {
            if let Some(index) = self.alarms.iter().position(|alarm: &AlarmClock| alarm.a_id == alarm_id) {
                if !self.alarms[index].is_radio {
                    self.delete_song(self.alarms[index].song.clone());
                }
                self.alarms.remove(index);
                self.save_alarms().expect("Failed to save alarms");
                self.update_alarms_display();
            }
        }

        fn delete_song(&mut self, path: String) {
            let _ = remove_file(path);
        }

        pub fn check_alarms(&mut self) {
            println!("[DEBUG] Check alarms");
            let current_time = self.horaire.lock().unwrap().clone();
            let day_of_week = Local::now().weekday().num_days_from_monday() as usize; // 0 pour Lundi, 6 pour Dimanche
            for alarm in self.alarms.iter() {
                println!("[DEBUG] Alarm: {:?}", alarm);
                if alarm.active && alarm.to_compare(&current_time, day_of_week) {
                    if alarm.is_radio {
                        self.current_radio.lock().unwrap().selected_radio = alarm.a_radio.clone();
                        self.start_player(true, "".to_string());
                        break;
                    } else {
                        self.start_player(false, alarm.song.clone());
                        break;
                    }
                }
            }
        }

        pub fn on_marche_clicked(&mut self) {
            if self.player_status {
                println!("[INFO] Radio already running");
                self.stop_player();
            }
            self.start_player(true, "".to_string());
        }

        pub fn on_arret_clicked(&mut self) {
            self.stop_player();
            println!("[INFO] Stop Radio");
        }

        fn start_player(&mut self, is_radio: bool, file_path: String) {
            let radio_player = self.radio_player.clone();
            let wav_player = self.wav_player.clone();
            let current_radio = self.current_radio.clone();
            self.player_status = true;
            gtk::glib::MainContext::default().spawn_local(async move {
                if is_radio {
                    if let Some(url) = current_radio.lock().unwrap().get_url() {
                        radio_player.lock().unwrap().play(url.to_string());
                    }
                } else {
                    wav_player.lock().unwrap().play(file_path);
                }
            });
        }

        pub fn stop_player(&mut self) {
            self.radio_player.lock().unwrap().stop();
            self.wav_player.lock().unwrap().stop();
            self.player_status = false;
        }

        pub fn alarm_status(&mut self, alarm_id: usize) {
            for alarm in self.alarms.iter_mut() {
                if alarm.a_id == alarm_id {
                    alarm.active = !alarm.active;
                }
            }
            self.update_alarms_display();
        }

        pub fn on_save_clicked(&mut self, name: String, hour: u8, minute: u8, second: u8, song: String, days: [bool; 7]) {
            println!("[DEBUG] Save clicked");
            self.add_alarm(name, hour, minute, second, song, days);
        }

        pub fn on_radio_clicked(&mut self, id_radio: u8) {
            match id_radio {
                1 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::FranceInfo),
                2 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::FranceInter),
                3 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::RTL),
                4 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::RireChanson),
                5 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::Skyrock),
                _ => println!("Radio button {} clicked", id_radio),
            };
            println!("Radio button {} clicked \n radio status {}", id_radio, self.player_status);
            if self.player_status {
                self.on_arret_clicked();
                thread::sleep(time::Duration::from_millis(10));
                self.on_marche_clicked();
            }
        }

        pub fn get_horaire(&self) -> Arc<Mutex<Horaire>> {
            self.horaire.clone()
        }

        pub fn update_time(&mut self) {
            println!("[DEBUG] update time");
            self.horaire.lock().unwrap().update_time();
        }

        fn update_alarms_display(&mut self) {
            let mut view: std::sync::MutexGuard<View> = self.view.lock().unwrap();
            view.update_alarms_display();
        }
    }
}
