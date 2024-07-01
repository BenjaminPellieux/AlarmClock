pub mod ihm{
    use gtk::{Box, Button, Entry, Frame, Label, Orientation, RadioButton, SpinButton};
    
   #[derive(Clone)]
    pub struct Widgets {
        pub g_alarm_clock: Frame,
        pub g_alarm_clock_tab: Option<Frame>,
        pub s_heur_box: SpinButton,
        pub s_min_box: SpinButton,
        pub s_sec_box: SpinButton,
        pub i_name_ac: Entry,
        pub i_song_link: Entry,
        pub p_cancel: Button,
        pub p_save: Button,
        pub p_button_marche: Button,
        pub p_button_arret: Button,
        pub p_button_add_alarm_clock: Button,
        pub p_rad_b1: RadioButton,
        pub p_rad_b2: RadioButton,
        pub p_rad_b3: RadioButton,
        pub p_rad_b4: RadioButton,
        pub p_rad_b5: RadioButton,
        pub p_lcd_heure: Label,
        pub p_lcd_min: Label,
        pub p_lcd_sec: Label,
        pub alarms_container: Box
    }


    impl Widgets {
        pub fn new() -> Self {
            let p_rad_b1 = RadioButton::with_label("France Info");
            let p_rad_b2 = RadioButton::with_label_from_widget(&p_rad_b1, "France Inter");
            let p_rad_b3 = RadioButton::with_label_from_widget(&p_rad_b1, "RTL");
            let p_rad_b4 = RadioButton::with_label_from_widget(&p_rad_b1, "Rire & Chanson");
            let p_rad_b5 = RadioButton::with_label_from_widget(&p_rad_b1, "Skyrock");

            Widgets {
                g_alarm_clock: Frame::new(Some("Nouveau réveil")),
                g_alarm_clock_tab: None,
                s_heur_box: SpinButton::with_range(0.0, 23.0, 1.0),
                s_min_box: SpinButton::with_range(0.0, 59.0, 1.0),
                s_sec_box: SpinButton::with_range(0.0, 59.0, 1.0),
                i_name_ac: Entry::new(),
                i_song_link: Entry::new(),
                p_cancel: Button::with_label("Annuler"),
                p_save: Button::with_label("Sauvegarder"),
                p_button_marche: Button::with_label("Marche"),
                p_button_arret: Button::with_label("Arrêt"),
                p_button_add_alarm_clock: Button::with_label("Ajouter un réveil"),
                p_rad_b1,
                p_rad_b2,
                p_rad_b3,
                p_rad_b4,
                p_rad_b5,
                p_lcd_heure: Label::new(Some("00")),
                p_lcd_min: Label::new(Some("00")),
                p_lcd_sec: Label::new(Some("00")),
                alarms_container: Box::new(Orientation::Vertical, 10),
            }
        }
    }
}