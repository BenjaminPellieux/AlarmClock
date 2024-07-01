
pub mod model {
    use chrono::{DateTime, Local, Timelike};
    use serde::{Serialize, Deserialize};
    use std::time::SystemTime;
    use std::process::Command;
    // use rodio::{Decoder, OutputStream, OutputStreamHandle, source::Source};


    #[derive(Clone, Debug)]
    pub enum RadioStation {
        FranceInfo,
        FranceInter,
        RTL,
        RireChanson,
        Skyrock,
    }

    #[derive(Clone, Debug)]
    pub struct Radio {
        pub status: bool,
        pub selected_radio: Option<RadioStation>,
    }

    impl Radio {
        pub fn new() -> Self {
            Radio {
                status: false,
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

        pub fn get_hour(&self) -> u8{
            return self.hour;
        }
        pub fn get_min(&self) -> u8{
            return self.minute;
        }
        pub fn get_sec(&self) -> u8{
            return self.minute;
        }
       
    }


    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AlarmClock {
        pub horaire: Horaire,
        pub active: bool,
        pub is_radio: bool,
        pub song: String,
        pub link: String,
        pub id: usize,
        pub name: String
    }
    // https://www.youtube.com/watch?v=4qR5xmglC9g
    impl AlarmClock {
        pub fn new(name: String, hour: u8, minute: u8, second: u8, link: String, is_radio: bool, id: usize) -> Self {
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
                name,
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