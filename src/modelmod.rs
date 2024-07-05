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
        /// Crée une nouvelle instance de `Radio`.
        ///
        /// # Returns
        ///
        /// Une nouvelle instance de `Radio` avec `selected_radio` initialisé à `None`.
        pub fn new() -> Self {
            Radio {
                selected_radio: None,
            }
        }

        /// Retourne l'URL de la station de radio sélectionnée.
        ///
        /// # Returns
        ///
        /// Une option contenant l'URL de la station de radio sélectionnée, ou `None` si aucune station n'est sélectionnée.
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
        /// Crée une nouvelle instance de `Horaire` en initialisant l'heure actuelle.
        ///
        /// # Returns
        ///
        /// Une nouvelle instance de `Horaire` avec l'heure actuelle.
        pub fn new() -> Self {
            let now: SystemTime = SystemTime::now();
            let datetime: DateTime<Local> = now.into();
            Self {
                hour: datetime.hour() as u8,
                minute: datetime.minute() as u8,
                second: datetime.second() as u8,
            }
        }

        /// Met à jour l'heure actuelle.
        pub fn update_time(&mut self) {
            let now: SystemTime = SystemTime::now();
            let datetime: DateTime<Local> = now.into();
            self.hour = datetime.hour() as u8;
            self.minute = datetime.minute() as u8;
            self.second = datetime.second() as u8;
        }

        /// Retourne l'heure actuelle.
        ///
        /// # Returns
        ///
        /// L'heure actuelle.
        pub fn get_hour(&self) -> u8 {
            self.hour
        }

        /// Retourne les minutes actuelles.
        ///
        /// # Returns
        ///
        /// Les minutes actuelles.
        pub fn get_min(&self) -> u8 {
            self.minute
        }

        /// Retourne les secondes actuelles.
        ///
        /// # Returns
        ///
        /// Les secondes actuelles.
        pub fn get_sec(&self) -> u8 {
            self.second
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
            let (song_title, song_path) = if !is_radio {
                Self::get_song(a_id, link)
            } else {
                (String::new(), String::new())
            };

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

        /// Télécharge la chanson à partir du lien fourni et renvoie le titre et le chemin de la chanson.
        ///
        /// # Parameters
        ///
        /// * `a_id`: L'identifiant de l'alarme.
        /// * `link`: Le lien de la musique.
        ///
        /// # Returns
        ///
        /// Un tuple contenant le titre de la chanson et le chemin de la chanson.
        fn get_song(a_id: usize, link: String) -> (String, String) {
            let song_path = format!("song/Alarm_{}.wav", a_id);
            let mut song_title = String::new();
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
                if fs::metadata(&song_path).is_ok() {
                    let output = Command::new("yt-dlp")
                        .args(["--get-title", &link, "--cookies-from-browser", "firefox", "--print", "title"])
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

        /// Compare l'heure actuelle avec l'heure de l'alarme.
        ///
        /// # Parameters
        ///
        /// * `other`: L'heure actuelle.
        /// * `day_of_week`: Le jour de la semaine actuel.
        ///
        /// # Returns
        ///
        /// `true` si l'heure et le jour correspondent à l'alarme, `false` sinon.
        pub fn to_compare(&self, other: &Horaire, day_of_week: usize) -> bool {
            self.horaire.hour == other.hour &&
            self.horaire.minute == other.minute &&
            self.horaire.second == other.second &&
            self.days[day_of_week]
        }
    }
}
