pub mod view {
    use gtk::prelude::*;
    use gtk::{CssProvider, StyleContext, ApplicationWindow, Box, Button, Label, Orientation, glib, CheckButton};
    use std::sync::{Arc, Mutex};
    use async_channel::{unbounded, Receiver, Sender};
    use glib::{timeout_add_seconds, MainContext, ControlFlow};
    use std::fs::{File, remove_file};
    use std::io::{self, Read, Write};
    use serde_json;
    use std::{thread, time};
    use chrono::prelude::*;
    use crate::modelmod::model::{AlarmClock, Horaire, Radio, RadioStation};
    use crate::musicmod::music::{WavPlayer, RadioPlayer, Music};
    use crate::widgetmod::ihm::Widgets;

    /// Structure représentant la vue de l'application.
    #[derive(Clone)]
    pub struct View {
        widgets: Arc<Widgets>,
        alarms: Vec<AlarmClock>,
        current_radio: Arc<Mutex<Radio>>,
        horaire: Arc<Mutex<Horaire>>,
        sender: Sender<()>,
        radio_player: Arc<Mutex<RadioPlayer>>,
        wav_player: Arc<Mutex<WavPlayer>>,
        player_status: bool,
    }

    impl View {
        /// Crée une nouvelle instance de `View`.
        ///
        /// # Returns
        ///
        /// Une nouvelle instance de `View`.
        pub fn new() -> Self {
            let (sender, receiver) = unbounded();
            let widgets = Widgets::new();
            let alarms = vec![];
            let current_radio = Arc::new(Mutex::new(Radio::new()));
            let horaire = Arc::new(Mutex::new(Horaire::new()));
            let radio_player = Arc::new(Mutex::new(RadioPlayer::new()));
            let wav_player = Arc::new(Mutex::new(WavPlayer::new()));
            let player_status = false;

            let mut view = Self {
                widgets: Arc::new(widgets),
                alarms,
                current_radio,
                horaire,
                sender,
                radio_player,
                wav_player,
                player_status,
            };
            view.connect_receiver(receiver);
            view
        }

        /// Met à jour l'identifiant des alarmes.
        fn update_alarm_id(&mut self) {
            let mut count = 0;
            for alarm in self.alarms.iter_mut() {
                alarm.a_id = count;
                count += 1;
            }
        }

        /// Ajoute des alarmes en les chargeant depuis le fichier de sauvegarde.
        fn add_alarms(&mut self) {
            let _res = match self.load_alarms() {
                Ok(_res) => println!("[INFO] File loaded"),
                Err(error) => println!("[ERROR] Failed to load alarms {error:?}"),
            };
            let mut days = [false; 7];
            for (i, day_checkbox) in self.widgets.days_checkbuttons.iter().enumerate() {
                days[i] = day_checkbox.is_active();
            }
            let name_alarm = self.widgets.i_name_ac.text().to_string();
            let url_song = self.widgets.i_song_link.text().to_string();
            let tmp_alarm: AlarmClock;
            if url_song.is_empty() && self.current_radio.lock().unwrap().selected_radio.is_none() {
                println!("[ERROR] No song URL & No radio selected");
            } else if !url_song.is_empty() {
                tmp_alarm = AlarmClock::new(
                    self.alarms.len(),
                    name_alarm,
                    self.widgets.s_heur_box.value() as u8,
                    self.widgets.s_min_box.value() as u8,
                    self.widgets.s_sec_box.value() as u8,
                    url_song,
                    false,
                    None,
                    days,
                );
                self.alarms.push(tmp_alarm);
            } else {
                tmp_alarm = AlarmClock::new(
                    self.alarms.len(),
                    name_alarm,
                    self.widgets.s_heur_box.value() as u8,
                    self.widgets.s_min_box.value() as u8,
                    self.widgets.s_sec_box.value() as u8,
                    "".to_string(),
                    true,
                    self.current_radio.lock().unwrap().selected_radio.clone(),
                    days.clone(),
                );
                self.alarms.push(tmp_alarm);
            }
        }

        /// Sauvegarde les alarmes dans un fichier.
        ///
        /// # Returns
        ///
        /// `io::Result<()>` - Résultat de l'opération de sauvegarde.
        fn save_alarms(&mut self) -> io::Result<()> {
            let alarms = &self.alarms;
            let serialized = serde_json::to_string(&*alarms)?;
            let mut file = File::create("ser/alarms.json")?;
            file.write_all(serialized.as_bytes())?;
            Ok(())
        }

        /// Charge les alarmes depuis un fichier.
        ///
        /// # Returns
        ///
        /// `io::Result<()>` - Résultat de l'opération de chargement.
        pub fn load_alarms(&mut self) -> io::Result<()> {
            let mut file = File::open("ser/alarms.json").unwrap_or_else(|_| File::create("ser/alarms.json").unwrap());
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            if !contents.is_empty() {
                let alarms: Vec<AlarmClock> = serde_json::from_str(&contents)?;
                self.alarms = alarms.to_vec();
            } else {
                self.alarms = Vec::new();
                println!("[INFO] No alarms found");
            }
            self.update_alarm_id();
            Ok(())
        }

        /// Met à jour l'affichage des alarmes.
        fn update_alarms_display(&mut self) {
            self.widgets.alarms_container.foreach(|child: &gtk::Widget| self.widgets.alarms_container.remove(child));
            for alarm in self.alarms.iter() {
                let vbox_alarm = Box::new(Orientation::Vertical, 5);
                vbox_alarm.set_widget_name("box-alarm");
                let hbox_alarm = Box::new(Orientation::Horizontal, 5);
                let hbox_days = Box::new(Orientation::Horizontal, 5);

                let hour_label = Label::new(Some(&format!("{:02}", alarm.horaire.hour)));
                let min_label = Label::new(Some(&format!("{:02}", alarm.horaire.minute)));
                let sec_label = Label::new(Some(&format!("{:02}", alarm.horaire.second)));
                hour_label.set_widget_name("label-large");
                min_label.set_widget_name("label-large");
                sec_label.set_widget_name("label-large");
                let link_label = Label::new(Some(&alarm.song_title));
                let alamrm_name = Label::new(Some(&alarm.name));
                
                hbox_alarm.pack_start(&hour_label, true, true, 0);
                hbox_alarm.pack_start(&Label::new(Some("H")), false, false, 0);
                hbox_alarm.pack_start(&min_label, true, true, 0);
                hbox_alarm.pack_start(&Label::new(Some("Min")), false, false, 0);
                hbox_alarm.pack_start(&sec_label, true, true, 0);
                hbox_alarm.pack_start(&Label::new(Some("Sec")), false, false, 0);
                hbox_alarm.pack_start(&link_label, true, true, 0);
                hbox_alarm.pack_start(&alamrm_name, true, true, 0);
    
                // Affichage des jours
                let days = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];
                for (i, &day) in days.iter().enumerate() {
                    let day_label = Label::new(Some(day));
                    let day_checkbox = CheckButton::new();
                    day_checkbox.set_active(alarm.days[i]);
                    day_checkbox.set_sensitive(false);
                    hbox_days.pack_start(&day_label, true, true, 0);
                    hbox_days.pack_start(&day_checkbox, true, true, 0);
                }

                let delete_button = Button::with_label("Supprimer");
                let active_radio = CheckButton::with_label("Active");
                active_radio.set_active(alarm.active);
                let delete_alarm_id = alarm.a_id;
                let view_rc = Arc::new(Mutex::new(self.clone()));

                delete_button.connect_clicked(move |_| {
                    let mut view = view_rc.lock().unwrap();
                    view.delete_alarm(delete_alarm_id);
                });

                let view_rc = Arc::new(Mutex::new(self.clone()));
                active_radio.connect_clicked(move |_| {
                    let mut view = view_rc.lock().unwrap();
                    view.alarm_status(delete_alarm_id);
                });

                hbox_alarm.pack_start(&active_radio, false, false, 0);
                hbox_alarm.pack_start(&delete_button, false, false, 0);
                vbox_alarm.add(&hbox_alarm);
                vbox_alarm.add(&hbox_days);
                self.widgets.alarms_container.add(&vbox_alarm);
            }

            self.widgets.alarms_container.show_all();
        }

        /// Met à jour l'état d'une alarme (active ou non).
        ///
        /// # Parameters
        ///
        /// * `alarm_id` - Identifiant de l'alarme à mettre à jour.
        fn alarm_status(&mut self, alarm_id: usize) {
            for alarm in self.alarms.iter_mut() {
                if alarm.a_id == alarm_id {
                    alarm.active = !alarm.active;
                }
            }

            self.save_alarms().expect("Failed to save alarms");
            self.update_alarms_display();
        }

        /// Supprime une chanson en fonction du chemin fourni.
        ///
        /// # Parameters
        ///
        /// * `path` - Chemin de la chanson à supprimer.
        fn delet_song(&mut self, path: String) {
            let _ = remove_file(path);
        }

        /// Supprime une alarme en fonction de son identifiant.
        ///
        /// # Parameters
        ///
        /// * `alarm_id` - Identifiant de l'alarme à supprimer.
        fn delete_alarm(&mut self, alarm_id: usize) {
            if let Some(index) = self.alarms.iter().position(|alarm: &AlarmClock| alarm.a_id == alarm_id) {
                if !self.alarms[index].is_radio {
                    self.delet_song(self.alarms[index].song_path.clone());
                }
                self.alarms.remove(index);
                self.save_alarms().expect("Failed to save alarms");
                self.update_alarms_display();
            }
        }

        /// Construit l'interface utilisateur de l'application.
        ///
        /// # Parameters
        ///
        /// * `window` - Fenêtre principale de l'application.
        pub fn build_ui(&mut self, window: &ApplicationWindow) {
            let provider = CssProvider::new();
            provider.load_from_path("style/styleapp.css").expect("Failed to load CSS");

            StyleContext::add_provider_for_screen(
                &gtk::prelude::WidgetExt::screen(window).unwrap(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        
            window.set_title("Alarm Clock");
            window.set_default_size(600, 800);

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
            self.widgets.i_song_link.set_placeholder_text("URL de la musique".into());
            hbox_reveil.pack_start(&self.widgets.i_name_ac, true, true, 0);
            hbox_reveil.pack_start(&self.widgets.p_cancel, true, true, 0);
            hbox_reveil.pack_start(&self.widgets.p_save, true, true, 0);
            
            // Ajouter des cases à cocher pour les jours
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
            
            // Met à jour les alarmes et le temps
            self.update_alarms_display();
            unsafe { self.update_time_labels() };
        }

        /// Connecte les signaux aux boutons et autres widgets.
        pub fn connect_signals(&mut self) {
            let view_rc = Arc::new(Mutex::new(self.clone()));

            // Bouton Marche
            let view_clone = view_rc.clone();
            self.widgets.p_button_marche.connect_clicked(move |_| {
                let mut view = view_clone.lock().unwrap();
                view.on_marche_clicked();
            });

            // Bouton Arrêt
            let view_clone = view_rc.clone();
            self.widgets.p_button_arret.connect_clicked(move |_| {
                let mut view = view_clone.lock().unwrap();
                view.on_arret_clicked();
            });

            // Bouton Ajouter un réveil
            let view_clone = view_rc.clone();
            self.widgets.p_button_add_alarm_clock.connect_clicked(move |_| {
                let view = view_clone.lock().unwrap();
                view.on_new_alarm_clicked();
            });

            // Bouton Sauvegarder
            let view_clone = view_rc.clone();
            self.widgets.p_save.connect_clicked(move |_| {
                let mut view = view_clone.lock().unwrap();
                view.on_save_clicked();
            });

            // Bouton Annuler
            let view_clone = view_rc.clone();
            self.widgets.p_cancel.connect_clicked(move |_| {
                let view = view_clone.lock().unwrap();
                view.on_cancel_clicked();
            });

            // Boutons Radio
            let view_clone = view_rc.clone();
            self.widgets.p_rad_b1.connect_toggled(move |radio| {
                if radio.is_active() {
                    let mut view = view_clone.lock().unwrap();
                    view.on_radio_clicked(1);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b2.connect_toggled(move |radio| {
                if radio.is_active() {
                    let mut view = view_clone.lock().unwrap();
                    view.on_radio_clicked(2);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b3.connect_toggled(move |radio| {
                if radio.is_active() {
                    let mut view = view_clone.lock().unwrap();
                    view.on_radio_clicked(3);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b4.connect_toggled(move |radio| {
                if radio.is_active() {
                    let mut view = view_clone.lock().unwrap();
                    view.on_radio_clicked(4);
                }
            });

            let view_clone = view_rc.clone();
            self.widgets.p_rad_b5.connect_toggled(move |radio| {
                if radio.is_active() {
                    let mut view = view_clone.lock().unwrap();
                    view.on_radio_clicked(5);
                }
            });
        }

        /// Vérifie les alarmes et déclenche celles qui sont actives à l'heure actuelle.
        pub fn check_alarms(&mut self) {
            let _res = match self.load_alarms() {
                Ok(_res) => {},
                Err(error) => println!("[ERROR] Failed to load alarms {error:?}"),
            };

            let current_time = self.horaire.lock().unwrap().clone();
            let day_of_week = Local::now().weekday().num_days_from_monday() as usize; // 0 pour Lundi, 6 pour Dimanche
            for alarm in self.alarms.iter() {
                if alarm.active && alarm.to_compare(&current_time, day_of_week) {
                    if alarm.is_radio {
                        self.current_radio.lock().unwrap().selected_radio = alarm.a_radio.clone();
                        self.start_player(true, "".to_string());
                        break;
                    } else {
                        self.start_player(false, alarm.song_path.clone());
                        break;
                    }
                }
            }
        }

        /// Démarre le lecteur de musique ou de radio.
        ///
        /// # Parameters
        ///
        /// * `radio` - Indique s'il s'agit d'une radio.
        /// * `file_path` - Chemin du fichier à lire.
        fn start_player(&mut self, radio: bool, file_path: String) {
            self.player_status = true;
            let current_radio = self.current_radio.clone();
            let radio_player = self.radio_player.clone();
            let wav_player = self.wav_player.clone();
            gtk::glib::MainContext::default().spawn_local(async move {
                if radio {
                    if let Some(url) = current_radio.lock().unwrap().get_url() {
                        radio_player.lock().unwrap().play(url.to_string());
                    } else {
                        println!("No radio selected");
                    }
                } else {
                    wav_player.lock().unwrap().play(file_path);
                }
            });
        }

        /// Arrête le lecteur de musique ou de radio.
        pub fn stop_player(&mut self) {
            self.radio_player.lock().unwrap().stop();
            self.wav_player.lock().unwrap().stop();
            self.player_status = false;
        }

        /// Gestionnaire pour le clic sur le bouton Marche.
        pub fn on_marche_clicked(&mut self) {
            if self.player_status {
                println!("[INFO] Radio already running");
                self.stop_player();
            }
            self.start_player(true, "".to_string());
        }

        /// Gestionnaire pour le clic sur le bouton Arrêt.
        pub fn on_arret_clicked(&mut self) {
            self.stop_player();
            println!("[INFO] Stop Radio");
        }

        /// Affiche le formulaire pour ajouter une nouvelle alarme.
        fn on_new_alarm_clicked(&self) {
            let horaire = self.horaire.lock().unwrap();
            self.widgets.s_heur_box.set_value(horaire.get_hour() as f64);
            self.widgets.s_min_box.set_value(horaire.get_min() as f64);
            self.widgets.s_sec_box.set_value(horaire.get_sec() as f64);
            self.widgets.g_alarm_clock.show_all();
        }

        /// Sauvegarde une nouvelle alarme.
        fn on_save_clicked(&mut self) {
            self.add_alarms();
            self.save_alarms().expect("Failed to save alarms");
            self.update_alarms_display();
            self.widgets.g_alarm_clock.hide();
        }

        /// Annule l'ajout d'une nouvelle alarme.
        pub fn on_cancel_clicked(&self) {
            self.widgets.g_alarm_clock.hide();
        }

        /// Gestionnaire pour le clic sur un bouton radio.
        ///
        /// # Parameters
        ///
        /// * `id_radio` - Identifiant de la station de radio sélectionnée.
        pub fn on_radio_clicked(&mut self, id_radio: u8) {
            match id_radio {
                1 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::FranceInfo),
                2 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::FranceInter),
                3 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::RTL),
                4 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::RireChanson),
                5 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::Skyrock),
                _ => println!("Radio button {} clicked", id_radio),
            };
            println!("[INFO] Radio button {} radio status {}", id_radio, self.player_status);
            if self.player_status {
                self.on_arret_clicked();
                thread::sleep(time::Duration::from_millis(10));
                self.on_marche_clicked();
            }
        }

        /// Met à jour l'heure affichée dans les labels.
        unsafe fn update_time_labels(&self) {
            let horaire_rc = self.horaire.clone();
            let sender = self.sender.clone();

            timeout_add_seconds(1, move || {
                let mut horaire = horaire_rc.lock().unwrap();
                horaire.update_time();

                // Envoyer un signal pour mettre à jour les widgets
                if let Err(e) = sender.try_send(()) {
                    eprintln!("[ERROR] Failed to send update signal: {:?}", e);
                }

                ControlFlow::Continue
            });
        }

        /// Connecte le récepteur de messages pour mettre à jour l'affichage de l'horloge et vérifier les alarmes.
        ///
        /// # Parameters
        ///
        /// * `receiver` - Récepteur de messages pour les mises à jour.
        fn connect_receiver(&mut self, receiver: Receiver<()>) {
            let widgets_rc = self.widgets.clone();
            let horaire_rc = self.horaire.clone();
            let view_rc = Arc::new(Mutex::new(self.clone()));

            MainContext::default().spawn_local(async move {
                while let Ok(_) = receiver.recv().await {
                    view_rc.lock().unwrap().check_alarms();
                    let horaire = horaire_rc.lock().unwrap();
                    widgets_rc.p_lcd_heure.set_text(&format!("{:02}", horaire.get_hour()));
                    widgets_rc.p_lcd_min.set_text(&format!("{:02}", horaire.get_min()));
                    widgets_rc.p_lcd_sec.set_text(&format!("{:02}", horaire.get_sec()));
                }
            });
        }
    }
}
