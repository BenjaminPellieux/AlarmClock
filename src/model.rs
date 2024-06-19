use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;
use std::cell::RefCell;


pub struct Alarm {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub is_radio: bool,
    pub audio_link: String,
    pub song_path: String,
    pub id: u32,
    pub status: bool,
}

pub struct AlarmClock {
    pub alarms: RefCell<Vec<Alarm>>,
    current_time: RefCell<String>,
}

impl AlarmClock {
    pub fn new() -> Rc<Self> {
        Rc::new(AlarmClock {
            alarms: RefCell::new(Vec::new()),
            current_time: RefCell::new(Self::get_current_time()),
        })
    }

    fn get_current_time() -> String {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let in_seconds = since_the_epoch.as_secs();
        let local_time = chrono::NaiveDateTime::from_timestamp(in_seconds as i64, 0);
        local_time.format("%H:%M:%S").to_string()
    }

    fn update_time(&self) {
        *self.current_time.borrow_mut() = Self::get_current_time();
    }

    fn add_alarm(&self, hour: u8, minute: u8, second: u8, is_radio: bool, audio_link: String, song_path: String) {
        let id = self.alarms.borrow().len() as u32;
        let alarm = Alarm {
            hour,
            minute,
            second,
            is_radio,
            audio_link,
            song_path,
            id,
            status: false,
        };
        self.alarms.borrow_mut().push(alarm);
    }

    fn remove_alarm(&self, id: u32) {
        self.alarms.borrow_mut().retain(|alarm| alarm.id != id);
    }

    fn toggle_alarm_status(&self, id: u32) {
        if let Some(alarm) = self.alarms.borrow_mut().iter_mut().find(|alarm| alarm.id == id) {
            alarm.status = !alarm.status;
        }
    }
}
