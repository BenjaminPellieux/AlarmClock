pub mod view {
    use gtk::prelude::*;
    use gtk::{ApplicationWindow, Box, Button, Entry, Frame, Label, Orientation, RadioButton, SpinButton, glib};
    use std::sync::{Arc, Mutex, MutexGuard};
    use crate::modelmod::model::{AlarmClock, Horaire};
    use glib::{timeout_add_seconds, Priority, MainContext, Sender, ControlFlow};

    #[derive(Clone)]
    pub struct View {
        widgets: Arc<Widgets>,
        alarms: Arc<Mutex<Vec<AlarmClock>>>,
        current_radio: Arc<Mutex<Option<usize>>>,
        horaire: Arc<Mutex<Horaire>>,
        sender: Sender<()>,
    }

    #[derive(Clone)]
    struct Widgets {
        g_alarm_clock: Frame,
        g_alarm_clock_tab: Option<Frame>,
        s_heur_box: SpinButton,
        s_min_box: SpinButton,
        s_sec_box: SpinButton,
        i_song_link: Entry,
        p_cancel: Button,
        p_save: Button,
        p_button_marche: Button,
        p_button_arret: Button,
        p_button_add_alarm_clock: Button,
        p_button_del_alarm: Option<Button>,
        p_rad_b1: RadioButton,
        p_rad_b2: RadioButton,
        p_rad_b3: RadioButton,
        p_rad_b4: RadioButton,
        p_rad_b5: RadioButton,
        p_lcd_heure: Label,
        p_lcd_min: Label,
        p_lcd_sec: Label,
    }

    impl View {
        pub fn new() -> Self {
            let (sender, receiver) = MainContext::channel( Priority::DEFAULT_IDLE);
            let widgets: Widgets = Widgets::new();
            let alarms: Arc<Mutex<Vec<AlarmClock>>> = Arc::new(Mutex::new(Vec::new()));
            let current_radio: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(None));
            let horaire: Arc<Mutex<Horaire>> = Arc::new(Mutex::new(Horaire::new()));

            let view = Self {
                widgets: Arc::new(widgets),
                alarms,
                current_radio,
                horaire,
                sender,
            };

            view.connect_receiver(receiver);
            view
        }

        pub fn build_ui(&self, window: &ApplicationWindow) {
            // let window = ApplicationWindow::new(app);
            window.set_title("Alarm Clock");
            window.set_default_size(400, 400);

            let vbox = Box::new(Orientation::Vertical, 10);

            let hbox1 = Box::new(Orientation::Horizontal, 5);
            hbox1.pack_start(&self.widgets.p_lcd_heure, true, true, 0);
            hbox1.pack_start(&Label::new(Some("H")), false, false, 0);
            hbox1.pack_start(&self.widgets.p_lcd_min, true, true, 0);
            hbox1.pack_start(&Label::new(Some("Min")), false, false, 0);
            hbox1.pack_start(&self.widgets.p_lcd_sec, true, true, 0);
            hbox1.pack_start(&Label::new(Some("Sec")), false, false, 0);

            let hbox2 = Box::new(Orientation::Horizontal, 5);
            hbox2.pack_start(&self.widgets.p_button_marche, true, true, 0);
            hbox2.pack_start(&self.widgets.p_button_add_alarm_clock, true, true, 0);
            hbox2.pack_start(&self.widgets.p_button_arret, true, true, 0);

            let hbox_rad_b = Box::new(Orientation::Horizontal, 5);
            hbox_rad_b.pack_start(&self.widgets.p_rad_b1, true, true, 0);
            hbox_rad_b.pack_start(&self.widgets.p_rad_b2, true, true, 0);
            hbox_rad_b.pack_start(&self.widgets.p_rad_b3, true, true, 0);
            hbox_rad_b.pack_start(&self.widgets.p_rad_b4, true, true, 0);
            hbox_rad_b.pack_start(&self.widgets.p_rad_b5, true, true, 0);

            let hbox_reveil = Box::new(Orientation::Horizontal, 5);
            hbox_reveil.pack_start(&self.widgets.s_heur_box, true, true, 0);
            hbox_reveil.pack_start(&Label::new(Some("H")), false, false, 0);
            hbox_reveil.pack_start(&self.widgets.s_min_box, true, true, 0);
            hbox_reveil.pack_start(&Label::new(Some("Min")), false, false, 0);
            hbox_reveil.pack_start(&self.widgets.s_sec_box, true, true, 0);
            hbox_reveil.pack_start(&Label::new(Some("Sec")), false, false, 0);
            hbox_reveil.pack_start(&self.widgets.i_song_link, true, true, 0);
            hbox_reveil.pack_start(&self.widgets.p_cancel, true, true, 0);
            hbox_reveil.pack_start(&self.widgets.p_save, true, true, 0);

            self.widgets.g_alarm_clock.add(&hbox_reveil);

            vbox.pack_start(&hbox1, false, false, 10);
            vbox.pack_start(&hbox2, false, false, 10);
            vbox.pack_start(&self.widgets.g_alarm_clock, false, false, 10);
            vbox.pack_start(&hbox_rad_b, false, false, 20);

            window.add(&vbox);
            //window.show_all();

            // Connect signals
            self.connect_signals();

            // Update the time every second
            unsafe{self.update_time_labels()};
        }

        fn connect_signals(&self) {
            let view_rc = Arc::new(Mutex::new(self.clone()));

            // Marche button
            let view_clone = view_rc.clone();
            self.widgets.p_button_marche.connect_clicked(move |_| {
                let view = view_clone.lock().unwrap();
                view.on_marche_clicked();
            });

            // Arrêt button
            let view_clone = view_rc.clone();
            self.widgets.p_button_arret.connect_clicked(move |_| {
                let view = view_clone.lock().unwrap();
                view.on_arret_clicked();
            });

            // Ajouter un réveil button
            let view_clone = view_rc.clone();
            self.widgets.p_button_add_alarm_clock.connect_clicked(move |_| {
                let view = view_clone.lock().unwrap();
                view.on_new_alarm_clicked();
            });

            // Save button
            let view_clone = view_rc.clone();
            self.widgets.p_save.connect_clicked(move |_| {
                let view = view_clone.lock().unwrap();
                view.on_save_clicked();
            });

            // Cancel button
            let view_clone = view_rc.clone();
            self.widgets.p_cancel.connect_clicked(move |_| {
                let view = view_clone.lock().unwrap();
                view.on_cancel_clicked();
            });

            // Radio buttons
            let view_clone = view_rc.clone();
            self.widgets.p_rad_b1.connect_toggled(move |radio| {
                if radio.is_active() {
                    let view = view_clone.lock().unwrap();
                    view.on_radio_clicked(1);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b2.connect_toggled(move |radio| {
                if radio.is_active() {
                    let view = view_clone.lock().unwrap();
                    view.on_radio_clicked(2);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b3.connect_toggled(move |radio| {
                if radio.is_active() {
                    let view = view_clone.lock().unwrap();
                    view.on_radio_clicked(3);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b4.connect_toggled(move |radio| {
                if radio.is_active() {
                    let view = view_clone.lock().unwrap();
                    view.on_radio_clicked(4);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b5.connect_toggled(move |radio| {
                if radio.is_active() {
                    let view = view_clone.lock().unwrap();
                    view.on_radio_clicked(5);
                }
            });
        }

        fn on_marche_clicked(&self) {
            // Logic for marche button
            println!("Marche button clicked");
        }

        fn on_arret_clicked(&self) {
            // Logic for arrêt button
            println!("Arrêt button clicked");
        }

        fn on_new_alarm_clicked(&self) {
            // Logic for adding new alarm
            println!("Ajouter un réveil button clicked");
            self.widgets.g_alarm_clock.show_all();
        }

        fn on_save_clicked(&self) {
            // Logic for saving alarm
            println!("Save button clicked");
            self.widgets.g_alarm_clock.hide();
        }

        fn on_cancel_clicked(&self) {
            // Logic for canceling alarm
            println!("Cancel button clicked");
            self.widgets.g_alarm_clock.hide();
        }

        fn on_radio_clicked(&self, id: u8) {
            // Logic for radio button clicked
            println!("Radio button {} clicked", id);
        }

        unsafe fn update_time_labels(&self) {
            let horaire_rc = self.horaire.clone();
            let sender = self.sender.clone();
            
            timeout_add_seconds(1, move || {
                let mut horaire = horaire_rc.lock().unwrap();
                horaire.update_time();

                // Envoyer un signal pour mettre à jour les widgets
                sender.send(()).expect("Could not send update signal");

                ControlFlow::Continue
            });
        }

        fn connect_receiver(&self, receiver: glib::Receiver<()>) {
            let widgets_rc: Arc<Widgets> = self.widgets.clone();
            let horaire_rc: Arc<Mutex<Horaire>> = self.horaire.clone();
            
            receiver.attach(None, move |_| {
                let horaire: MutexGuard<Horaire> = horaire_rc.lock().unwrap();
                widgets_rc.p_lcd_heure.set_text(&format!("{:02}", horaire.hour));
                widgets_rc.p_lcd_min.set_text(&format!("{:02}", horaire.minute));
                widgets_rc.p_lcd_sec.set_text(&format!("{:02}", horaire.second));
                ControlFlow::Continue
            });
        }
    }

    impl Widgets {
        fn new() -> Self {
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
                i_song_link: Entry::new(),
                p_cancel: Button::with_label("Annuler"),
                p_save: Button::with_label("Sauvegarder"),
                p_button_marche: Button::with_label("Marche"),
                p_button_arret: Button::with_label("Arrêt"),
                p_button_add_alarm_clock: Button::with_label("Ajouter un réveil"),
                p_button_del_alarm: None,
                p_rad_b1,
                p_rad_b2,
                p_rad_b3,
                p_rad_b4,
                p_rad_b5,
                p_lcd_heure: Label::new(Some("00")),
                p_lcd_min: Label::new(Some("00")),
                p_lcd_sec: Label::new(Some("00")),
            }
        }
    }
}
