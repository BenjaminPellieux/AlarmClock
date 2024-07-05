
pub mod model {
    use chrono::{DateTime, Local, Timelike};
    use serde::{Serialize, Deserialize};
    use std::time::SystemTime;
    use std::process::Command;
    use std::fs;
    use std::str;

    #[derive(Clone, Serialize, Deserialize)]
    pub enum RadioStation {
        FranceInfo,
        FranceInter,
        RTL,
        RireChanson,
        Skyrock,
    }

    #[derive(Clone)]
    pub struct Radio {
        pub selected_radio: Option<RadioStation>,
    }

    impl Radio {
        pub fn new() -> Self {
            Radio {
                selected_radio: None,
            }
        }

        pub fn get_url(&self) -> Option<&'static str> {
            match self.selected_radio {
                Some(RadioStation::FranceInfo) => Some("http://direct.franceinfo.fr/live/franceinfo-midfi.mp3"),
                Some(RadioStation::FranceInter) => Some("http://direct.franceinter.fr/live/franceinter-midfi.mp3"),
                Some(RadioStation::RTL) => Some("http://streaming.radio.rtl.fr/rtl-1-44-128"),
                Some(RadioStation::RireChanson) => Some("http://cdn.nrjaudio.fm/audio1/fr/30401/mp3_128.mp3"),
                Some(RadioStation::Skyrock) => Some("http://icecast.skyrock.net/s/natio_mp3_128k"),
                None => None,
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Horaire {
        pub hour: u8,
        pub minute: u8,
        pub second: u8,
    }

    impl Horaire {
        pub fn new() -> Self {
            let now: SystemTime = SystemTime::now();
            let datetime: DateTime<Local> = now.into();
            Self {
                hour: datetime.hour() as u8,
                minute: datetime.minute() as u8,
                second: datetime.second() as u8,
            }
        }

        pub fn update_time(&mut self) {
            let now: SystemTime = SystemTime::now();
            let datetime: DateTime<Local> = now.into();
            self.hour = datetime.hour() as u8;
            self.minute = datetime.minute() as u8;
            self.second = datetime.second() as u8;
        }

        pub fn get_hour(&self) -> u8{
            return self.hour;
        }
        pub fn get_min(&self) -> u8{
            return self.minute;
        }
        pub fn get_sec(&self) -> u8{
            return self.second;
        }
       
    }


    #[derive(Clone, Serialize, Deserialize)]
    pub struct AlarmClock {
        pub horaire: Horaire,
        pub active: bool,
        pub is_radio: bool,
        pub song_path: String,
        pub song_title: String,
        pub a_radio: Option<RadioStation>,
        pub a_id: usize,
        pub name: String,
        pub days: [bool; 7],
    }
    // https://www.youtube.com/watch?v=4qR5xmglC9g
    // https://www.youtube.com/watch?v=GFUN4pqAhLg
    impl AlarmClock {
          /// Crée une nouvelle instance d'`AlarmClock`.
            ///
            /// # Parameters
            ///
            /// * `a_id`: L'identifiant de l'alarme.
            /// * `name`: Le nom de l'alarme.
            /// * `hour`: L'heure de l'alarme.
            /// * `minute`: Les minutes de l'alarme.
            /// * `second`: Les secondes de l'alarme.
            /// * `link`: Le lien de la musique ou de la radio.
            /// * `is_radio`: Indique si c'est une radio.
            /// * `a_radio`: L'option de station de radio.
            /// * `days`: Les jours de l'alarme.
            ///
            /// # Returns
            ///
            /// Une nouvelle instance d'`AlarmClock`.
        pub fn new(a_id: usize, name: String, hour: u8, minute: u8, second: u8, link: String, is_radio: bool, a_radio: Option<RadioStation>, days: [bool; 7]) -> Self {
            let mut song_path: String = String::new();
            let mut song_title: String = String::new();
            if !is_radio {
                (song_title, song_path) = Self::get_song(a_id, link);  
            }

            
            Self {
                a_id,
                name,
                horaire: Horaire {
                    hour,
                    minute,
                    second,
                },
                active: true,
                is_radio,
                song_path,
                song_title,
                a_radio,
                days,
            }
        }


        fn get_song(a_id: usize, link: String) -> (String,String) {
            let song_path: String = format!("song/Alarm_{}.wav", a_id);
            let mut song_title:  String = String::new();
            let status = Command::new("yt-dlp")
                .args([
                    "--format", "bestaudio",
                    "--extract-audio",
                    "--audio-format", "wav",
                    "--cookies-from-browser", "firefox",
                    "--output", song_path.as_str(),
                    &link,
                ])
                .status()
                .expect("[ERROR] Failed to execute yt-dlp");

            if status.success() {
                // Vérifier si le fichier a été téléchargé
                if fs::metadata(&song_path).is_ok() {
                    // Récupérer le titre de la chanson
                    let output = Command::new("yt-dlp")
                        .args(["--get-title", &link,"--cookies-from-browser", "firefox", "--print", "title"])
                        .output()
                        .expect("[ERROR] Failed to retrieve song title");

                    if output.status.success() {
                        song_title = String::from_utf8_lossy(&output.stdout).to_string();
                        println!("[INFO] Song downloaded: {} path: {}", song_title, song_path);
                    } else {
                        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
                        eprintln!("[ERROR] Failed to retrieve song title: {}", error_message);
                    }
                } else {
                    eprintln!("[ERROR] File not downloaded: {}", song_path);
                }
            } else {
                eprintln!("[ERROR] Failed to download music");
            }
            let split_title: Vec<&str> = song_title.split('\n').collect();
            song_title = split_title[0].to_string();
            (song_title, song_path)
        }

        pub fn to_compare(&self, other: &Horaire, day_of_week: usize) -> bool {
            self.horaire.hour == other.hour &&
            self.horaire.minute == other.minute &&
            self.horaire.second == other.second &&
            self.days[day_of_week]
        }
    }
}