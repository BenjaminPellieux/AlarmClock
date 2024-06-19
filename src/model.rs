use std::time::{SystemTime, UNIX_EPOCH};

// Define the model for the alarm clock
#[derive(QObject, Default)]
struct AlarmClock {
    base: qt_base_class!(trait QObject),
    alarms: qt_property!(Vec<Alarm>),
    current_time: qt_property!(String),
    #[qt_signal]
    alarms_updated: Signal<()>,
    #[qt_signal]
    time_updated: Signal<()>,
}

#[derive(Default)]
struct Alarm {
    hour: u8,
    minute: u8,
    second: u8,
    is_radio: bool,
    audio_link: String,
    song_path: String,
    id: u32,
    status: bool,
}
