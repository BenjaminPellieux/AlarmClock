
pub mod model {
    use chrono::{DateTime, Local, Timelike};
    use serde::{Serialize, Deserialize};
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::process::{Command, Child};


    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Radio{
        vecURL: Vec<String>,
        status: bool,
        selected_radio: i8,
        }

    impl Radio{
        pub fn new() -> Self {

            Radio{
                vecURL: vec!["http://direct.franceinfo.fr/live/franceinfo-midfi.mp3".to_string(), 
                                "http://direct.franceinter.fr/live/franceinter-midfi.mp3".to_string(),
                                "http://streaming.radio.rtl.fr/rtl-1-44-128".to_string(),
                                "http://cdn.nrjaudio.fm/audio1/fr/30401/mp3_128.mp3".to_string(),
                                "http://icecast.skyrock.net/s/natio_mp3_128k".to_string()],
                status: false,
                selected_radio: -1,
            }
        }
        pub fn select_radio(&mut self){

        }
    }
    #[derive(Clone, Debug, Serialize, Deserialize)]
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
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AlarmClock {
        pub horaire: Horaire,
        pub active: bool,
        pub is_radio: bool,
        pub song: String,
        pub link: String,
        pub id: usize,
    }
    // https://www.youtube.com/watch?v=4qR5xmglC9g
    impl AlarmClock {
        pub fn new(hour: u8, minute: u8, second: u8, link: String, is_radio: bool, id: usize) -> Self {
            let song: String = String::new();
            if !is_radio {
                let song : String  =  format!("song/Alarm_{}.mp3", id);
                println!("[DEBUG] Dowloading song {}",song);
                Command::new("yt-dlp")

                .args(["--format", "bestaudio", 
                       "--extract-audio",
                        "--audio-format", "mp3",
                        "--cookies-from-browser", "firefox",
                        "--output",format!("song/Alarm_{}.mp3",id).as_str(),
                        format!("{}",link).as_str()])
                .spawn()
                .expect("[ERROR] Failed to download music");
                
            } 

            
            Self {
                horaire: Horaire {
                    hour,
                    minute,
                    second,
                },
                active: true,
                is_radio,
                song,
                link,
                id,
            }
        }

        pub fn to_compare(&self, other: &Horaire) -> bool {
            self.horaire.hour == other.hour &&
            self.horaire.minute == other.minute &&
            self.horaire.second == other.second
        }
    }
}