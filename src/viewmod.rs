pub mod view {
    use gtk::prelude::*;
    use gtk::{CssProvider, StyleContext, ApplicationWindow, Box, Button, Label, Orientation, glib, CheckButton};
    use std::sync::{Arc, Mutex, MutexGuard};
    use async_channel::{unbounded, Receiver, Sender};
    use glib::{timeout_add_seconds, MainContext, ControlFlow};
    use crate::modelmod::model::Horaire;
    use crate::widgetmod::ihm::Widgets;
    use crate::controllermod::controller::Controller;

    #[derive(Clone, Debug)]
    pub struct View {
        widgets: Arc<Widgets>,
        pub controller: Option<Arc<Mutex<Controller>>>,
        sender: Sender<()>,
    }

    impl View {
        pub fn new() -> Arc<Mutex<Self>> {
            let (sender, receiver) = unbounded();
            let widgets = Widgets::new();
            
            // Step 1: Create View instance without controller
            let view: Arc<Mutex<View>> = Arc::new(Mutex::new(View {
                widgets: Arc::new(widgets),
                controller: None,
                sender: sender.clone(),
            }));
    
            // Step 2: Create Controller instance using the View instance
            let controller: Arc<Mutex<Controller>> = Arc::new(Mutex::new(Controller::new(view.clone())));
    
            view.lock().unwrap().controller = Some(controller.clone());
            view.lock().unwrap().connect_receiver(receiver);
            view
        }

        pub fn update_alarms_display(&mut self) {
            self.widgets.alarms_container.foreach(|child: &gtk::Widget| self.widgets.alarms_container.remove(child));
            for alarm in self.controller.as_ref().unwrap().lock().unwrap().alarms.iter() {
                let vbox_alarm: Box = Box::new(Orientation::Vertical, 5);
                vbox_alarm.set_widget_name("box-alarm");
                let hbox_alarm: Box = Box::new(Orientation::Horizontal, 5);
                let hbox_days: Box = Box::new(Orientation::Horizontal, 5);

                let hour_label = Label::new(Some(&format!("{:02}", alarm.horaire.hour)));
                let min_label = Label::new(Some(&format!("{:02}", alarm.horaire.minute)));
                let sec_label = Label::new(Some(&format!("{:02}", alarm.horaire.second)));
                hour_label.set_widget_name("label-large");
                min_label.set_widget_name("label-large");
                sec_label.set_widget_name("label-large");
                let link_label = Label::new(Some(&alarm.song));
                let alarm_name = Label::new(Some(&alarm.name));
                
                hbox_alarm.pack_start(&hour_label, true, true, 0);
                hbox_alarm.pack_start(&Label::new(Some("H")), false, false, 0);
                hbox_alarm.pack_start(&min_label, true, true, 0);
                hbox_alarm.pack_start(&Label::new(Some("Min")), false, false, 0);
                hbox_alarm.pack_start(&sec_label, true, true, 0);
                hbox_alarm.pack_start(&Label::new(Some("Sec")), false, false, 0);
                hbox_alarm.pack_start(&link_label, true, true, 0);
                hbox_alarm.pack_start(&alarm_name, true, true, 0);
    
                // Display days
                let days: [&str; 7] = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];
                for (i, &day) in days.iter().enumerate() {
                    let day_label: Label = Label::new(Some(day));
                    let day_checkbox: CheckButton = CheckButton::new();
                    day_checkbox.set_active(alarm.days[i]);
                    day_checkbox.set_sensitive(false);
                    hbox_days.pack_start(&day_label, true, true, 0);
                    hbox_days.pack_start(&day_checkbox, true, true, 0);
                }

                let delete_button: Button = Button::with_label("Supprimer");
                let active_radio: CheckButton = CheckButton::with_label("Active");
                active_radio.set_active(alarm.active);
                let delete_alarm_id: usize = alarm.a_id;
                let view_rc: Arc<Mutex<View>> = Arc::new(Mutex::new(self.clone()));

                delete_button.connect_clicked(move |_| {
                    let tmp_view = view_rc.lock().unwrap();
                    let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                    ctrl.delete_alarm(delete_alarm_id);
                });

                let view_rc: Arc<Mutex<View>> = Arc::new(Mutex::new(self.clone()));
                active_radio.connect_clicked(move |_| {
                    let tmp_view = view_rc.lock().unwrap();
                    let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                    ctrl.alarm_status(delete_alarm_id);
                });

                hbox_alarm.pack_start(&active_radio, false, false, 0);
                hbox_alarm.pack_start(&delete_button, false, false, 0);
                vbox_alarm.add(&hbox_alarm);
                vbox_alarm.add(&hbox_days);
                self.widgets.alarms_container.add(&vbox_alarm);
            }

            self.widgets.alarms_container.show_all();
        }

        pub fn build_ui(&mut self, window: &ApplicationWindow) {
            let provider: CssProvider = CssProvider::new();
            provider.load_from_path("style/styleapp.css").expect("Failed to load CSS");

            StyleContext::add_provider_for_screen(
                &gtk::prelude::WidgetExt::screen(window).unwrap(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        
            window.set_title("Alarm Clock");
            window.set_default_size(600, 600);

            let vbox = Box::new(Orientation::Vertical, 10);

            let hbox1 = Box::new(Orientation::Horizontal, 5);
            self.widgets.p_lcd_heure.set_widget_name("label-large");
            self.widgets.p_lcd_min.set_widget_name("label-large");
            self.widgets.p_lcd_sec.set_widget_name("label-large");
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

            let vbox_reveil = Box::new(Orientation::Vertical, 5);
            let hbox_reveil = Box::new(Orientation::Horizontal, 5);
            let hbox_days = Box::new(Orientation::Horizontal, 5);
            hbox_reveil.pack_start(&self.widgets.s_heur_box, true, true, 0);
            hbox_reveil.pack_start(&Label::new(Some("H")), false, false, 0);
            hbox_reveil.pack_start(&self.widgets.s_min_box, true, true, 0);
            hbox_reveil.pack_start(&Label::new(Some("Min")), false, false, 0);
            hbox_reveil.pack_start(&self.widgets.s_sec_box, true, true, 0);
            hbox_reveil.pack_start(&Label::new(Some("Sec")), false, false, 0);
            self.widgets.i_name_ac.set_placeholder_text("Nom de l'alarme".into());
            self.widgets.i_song_link.set_placeholder_text("URL de la music".into());
            hbox_reveil.pack_start(&self.widgets.i_name_ac, true, true, 0);
            hbox_reveil.pack_start(&self.widgets.p_cancel, true, true, 0);
            hbox_reveil.pack_start(&self.widgets.p_save, true, true, 0);

            // Add checkboxes for days
            for day_checkbox in self.widgets.days_checkbuttons.iter() {
                hbox_days.pack_start(day_checkbox, true, true, 0);
            }
            vbox_reveil.add(&hbox_reveil);
            vbox_reveil.pack_start(&self.widgets.i_song_link, true, false, 5);
            vbox_reveil.add(&hbox_days);
            self.widgets.g_alarm_clock.add(&vbox_reveil);

            self.widgets.g_alarm_clock_tab.add(&self.widgets.alarms_container);
            vbox.pack_start(&hbox1, false, false, 10);
            vbox.pack_start(&hbox2, false, false, 10);
            vbox.pack_start(&self.widgets.g_alarm_clock, false, false, 10);
            vbox.pack_start(&hbox_rad_b, false, false, 20);
            vbox.pack_start(&self.widgets.g_alarm_clock_tab, false, true, 10); 

            window.add(&vbox);
            
            // Update the time every second
            self.update_alarms_display();
            unsafe { self.update_time_labels() };
        }

        pub fn connect_signals(&mut self) {
            let view_rc: Arc<Mutex<View>> = Arc::new(Mutex::new(self.clone()));

            let view_clone = view_rc.clone();
            self.widgets.p_button_marche.connect_clicked(move |_| {
                let tmp_view = view_clone.lock().unwrap();
                let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                ctrl.on_marche_clicked();
            });

            let view_clone = view_rc.clone();
            self.widgets.p_button_arret.connect_clicked(move |_| {
                let tmp_view = view_clone.lock().unwrap();
                let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                ctrl.on_arret_clicked();
            });

            let view_clone = view_rc.clone();
            self.widgets.p_button_add_alarm_clock.connect_clicked(move |_| {
                let mut view = view_clone.lock().unwrap();
                view.on_new_alarm_clicked();
            });

            let view_clone = view_rc.clone();
            self.widgets.p_cancel.connect_clicked(move |_| {
                let view: MutexGuard<View> = view_clone.lock().unwrap();
                view.on_cancel_clicked();
            });

            let view_clone: Arc<Mutex<View>> = view_rc.clone();
            self.widgets.p_save.connect_clicked(move |_| {
                let tmp_view = view_clone.lock().unwrap();
                let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                let mut days: [bool; 7] = [false; 7];
                for (i, day_checkbox) in tmp_view.widgets.days_checkbuttons.iter().enumerate() {
                    days[i] = day_checkbox.is_active();
                }
                println!("[DEBUG] Save alarm clicked");
                ctrl.on_save_clicked(
                    tmp_view.widgets.i_name_ac.text().to_string(),
                    tmp_view.widgets.s_heur_box.value() as u8,
                    tmp_view.widgets.s_min_box.value() as u8,
                    tmp_view.widgets.s_sec_box.value() as u8,
                    tmp_view.widgets.i_song_link.text().to_string(),
                    days,
                );
                tmp_view.widgets.g_alarm_clock.hide();
            });

            // Radio buttons
            let view_clone = view_rc.clone();
            self.widgets.p_rad_b1.connect_toggled(move |radio| {
                if radio.is_active() {
                    let tmp_view: MutexGuard<View> = view_clone.lock().unwrap();
                    let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                    ctrl.on_radio_clicked(1);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b2.connect_toggled(move |radio| {
                if radio.is_active() {
                    let tmp_view = view_clone.lock().unwrap();
                    let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                    ctrl.on_radio_clicked(2);
                }
            });

            let view_clone: Arc<Mutex<View>> = view_rc.clone();
            self.widgets.p_rad_b3.connect_toggled(move |radio| {
                if radio.is_active() {
                    let tmp_view = view_clone.lock().unwrap();
                    let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                    ctrl.on_radio_clicked(3);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b4.connect_toggled(move |radio| {
                if radio.is_active() {
                    let tmp_view = view_clone.lock().unwrap();
                    let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                    ctrl.on_radio_clicked(4);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b5.connect_toggled(move |radio| {
                if radio.is_active() {
                    let tmp_view = view_clone.lock().unwrap();
                    let mut ctrl: MutexGuard<Controller> = tmp_view.controller.as_ref().unwrap().lock().unwrap();
                    ctrl.on_radio_clicked(5);
                }
            });
        }

        fn on_new_alarm_clicked(&mut self) {
            // Logic for adding new alarm
            let tmp_ctrl = self.controller.as_ref().unwrap().lock().unwrap();
            let horaire: MutexGuard<Horaire> = tmp_ctrl.horaire.lock().unwrap();
            self.widgets.s_heur_box.set_value(horaire.get_hour() as f64);
            self.widgets.s_min_box.set_value(horaire.get_min() as f64);
            self.widgets.s_sec_box.set_value(horaire.get_sec() as f64);
            self.widgets.g_alarm_clock.show_all();
        }

        pub fn on_cancel_clicked(&self) {
            self.widgets.g_alarm_clock.hide();
        }

        pub fn connect_receiver(&mut self, receiver: Receiver<()>) {
            let widgets_rc: Arc<Widgets> = self.widgets.clone();
            let controller_rc: Arc<Mutex<Controller>> = self.controller.as_ref().unwrap().clone();

            MainContext::default().spawn_local(async move {
                while let Ok(_) = receiver.recv().await {
                    controller_rc.lock().unwrap().update_time();
                    controller_rc.lock().unwrap().check_alarms();
                    let horaire_rc: Arc<Mutex<Horaire>> = controller_rc.lock().unwrap().get_horaire();
                    let horaire: MutexGuard<Horaire> = horaire_rc.lock().unwrap();
                    widgets_rc.p_lcd_heure.set_text(&format!("{:02}", horaire.get_hour()));
                    widgets_rc.p_lcd_min.set_text(&format!("{:02}", horaire.get_min()));
                    widgets_rc.p_lcd_sec.set_text(&format!("{:02}", horaire.get_sec()));
                }
            });
        }

        pub unsafe fn update_time_labels(&self) {
            let sender = self.sender.clone();

            timeout_add_seconds(1, move || {
                let _ = sender.send(());
                ControlFlow::Continue
            });
        }
    }
}
