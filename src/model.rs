use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local, Timelike};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct AlarmClock {
    pub horaire: Horaire,
    pub status: bool,
    pub is_radio: bool,
    pub song: String,
    pub link: String,
    pub id: usize,
}

impl AlarmClock {
    pub fn new(hour: u8, minute: u8, second: u8, link: String, is_radio: bool, id: usize) -> Self {
        let song: String = if !is_radio {
            format!("music/Alarm_{}.mp3", id)
        } else {
            String::new()
        };
        Self {
            horaire: Horaire {
                hour,
                minute,
                second,
            },
            status: true,
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
