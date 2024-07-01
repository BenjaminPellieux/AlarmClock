pub mod view {
    use gtk::prelude::*;
    use gtk::{ApplicationWindow, Box, Button, Label, Orientation, glib, CheckButton};
    use std::sync::{Arc, Mutex, MutexGuard};
    use glib::{timeout_add_seconds, Priority, MainContext, Sender, ControlFlow, Source};
    use std::fs::File;
    use std::io::{self, Read, Write};
    use std::future::Future;
    use serde_json;
    use crate::modelmod::model::{AlarmClock, Horaire, Radio, RadioStation};
    use crate::musicmod::music::MusicPlayer;
    use crate::widgetmod::ihm::Widgets;

    #[derive(Clone)]
    pub struct View {
        widgets: Arc<Widgets>,
        alarms: Vec<AlarmClock>,
        current_radio: Arc<Mutex<Radio>>,
        horaire: Arc<Mutex<Horaire>>,
        sender: Sender<()>,
        music_player: Arc<Mutex<MusicPlayer>>,
    }

    impl View {
        pub fn new() -> Self {
            let (sender, receiver) = MainContext::channel(Priority::DEFAULT_IDLE);
            let widgets: Widgets = Widgets::new();
            let alarms: Vec<AlarmClock> = vec![];
            let current_radio: Arc<Mutex<Radio>> = Arc::new(Mutex::new(Radio::new()));
            let horaire: Arc<Mutex<Horaire>> = Arc::new(Mutex::new(Horaire::new()));
            let music_player: Arc<Mutex<MusicPlayer>> = Arc::new(Mutex::new(MusicPlayer::new()));

            let view: View = Self {
                widgets: Arc::new(widgets),
                alarms,
                current_radio,
                horaire,
                sender,
                music_player,
            };
            view.connect_receiver(receiver);
            view
        }


        fn update_alarm_id(&mut self){
            let mut count:  usize = 0;
            for alarm in self.alarms.iter_mut(){
                alarm.id = count;
                count+=1;
            }
        }
        fn add_alarms(&mut self){
            
            println!("[DEBUG] nb alamrs : {}", self.alarms.len());
            let _res: () = match self.load_alarms() {
                Ok(_res) => println!("[INFO]  File loaded"),
                Err(error) => println!("[ERROR] Failed to load alarms {error:?}"),
            };
            let name_alarm: String  = self.widgets.i_name_ac.text().to_string();
            let url_song: String  = self.widgets.i_song_link.text().to_string();
            let tmp_alarm: AlarmClock;
            println!("[DEBUG] nb alamrs : {}", self.alarms.len());
            println!("[DEBUG] url: {}", url_song);
            if url_song.ne(&"") && self.current_radio.lock().unwrap().selected_radio.is_some() {
                println!("[ERROR] No song URL & No radio selected");
                 
            }
            else if  url_song.ne(&"") {
                println!("[DEBUG] is radio FALSE ");
                tmp_alarm = AlarmClock::new(name_alarm, self.widgets.s_heur_box.value() as u8,
                                            self.widgets.s_min_box.value() as u8,
                                            self.widgets.s_sec_box.value() as u8,
                                            url_song, false, 
                                            self.alarms.len());
                self.alarms.push(tmp_alarm);

            }else{
                println!("[DEBUG] is radio TRUE ");
                tmp_alarm = AlarmClock::new(name_alarm, self.widgets.s_heur_box.value() as u8,
                                            self.widgets.s_min_box.value() as u8,
                                            self.widgets.s_sec_box.value() as u8,
                                            url_song, true, 
                                            self.alarms.len());
                self.alarms.push(tmp_alarm);
            }
            println!("[DEBUG] nb alamrs : {}", self.alarms.len());
            

            println!("[DEBUG] alarms vec {:#?}",self.alarms);

            
        }


        fn save_alarms(&mut self) -> io::Result<()>{
            let alarms: &Vec<AlarmClock>  = &self.alarms;
            let serialized: String = serde_json::to_string(&*alarms)?;
            let mut file: File = File::create("ser/alarms.json")?;
            file.write_all(serialized.as_bytes())?;
            Ok(())
        }


        pub fn load_alarms(&mut self) -> io::Result<()> {
            let mut file: File = File::open("ser/alarms.json").unwrap_or_else(|_| File::create("ser/alarms.json").unwrap());
            let mut contents: String = String::new();
            file.read_to_string(&mut contents)?;
            if !contents.is_empty() {
                let alarms: Vec<AlarmClock> = serde_json::from_str(&contents)?;
                self.alarms = alarms.to_vec();
            }else{
                self.alarms = Vec::new();
                println!("[INFO] No alarms finded ");
            }
            self.update_alarm_id();
            Ok(())
        }

        fn update_alarms_display(&mut self) {
            self.widgets.alarms_container.foreach(|child: &gtk::Widget| self.widgets.alarms_container.remove(child));
            println!("[DEBUG] Update alarms display");
            println!("[DEBUG] update alamrs display width {}",self.alarms.len());
            for alarm in self.alarms.iter() {
                let alarm_box: Box = Box::new(Orientation::Horizontal, 5);
                
                let hour_label = Label::new(Some(&format!("{:02}", alarm.horaire.hour)));
                let min_label = Label::new(Some(&format!("{:02}", alarm.horaire.minute)));
                let sec_label = Label::new(Some(&format!("{:02}", alarm.horaire.second)));
                let link_label = Label::new(Some(&alarm.song));
                let alamrm_name = Label::new(Some(&alarm.name));
                
                alarm_box.pack_start(&hour_label, true, true, 0);
                alarm_box.pack_start(&Label::new(Some("H")), false, false, 0);
                alarm_box.pack_start(&min_label, true, true, 0);
                alarm_box.pack_start(&Label::new(Some("Min")), false, false, 0);
                alarm_box.pack_start(&sec_label, true, true, 0);
                alarm_box.pack_start(&Label::new(Some("Sec")), false, false, 0);
                alarm_box.pack_start(&link_label, true, true, 0);
                alarm_box.pack_start(&alamrm_name, true, true, 0);
    
                let delete_button: Button = Button::with_label("Supprimer");
                let active_radio: CheckButton = CheckButton::with_label("Active");
                active_radio.set_active(alarm.active);
                let delete_alarm_id: usize = alarm.id;
                let view_rc: Arc<Mutex<View>> = Arc::new(Mutex::new(self.clone()));

                delete_button.connect_clicked(move |_| {
                    let mut view: MutexGuard<View> = view_rc.lock().unwrap();
                    view.delete_alarm(delete_alarm_id);
                });

                let view_rc: Arc<Mutex<View>> = Arc::new(Mutex::new(self.clone()));
                active_radio.connect_clicked(move |_| {
                    let mut view: MutexGuard<View> = view_rc.lock().unwrap();
                    view.alarm_status(delete_alarm_id);
                });

                alarm_box.pack_start(&active_radio, false, false, 0);
                alarm_box.pack_start(&delete_button, false, false, 0);
    
                self.widgets.alarms_container.pack_start(&alarm_box, false, false, 5);
            }
    
            self.widgets.alarms_container.show_all();
        }


        fn alarm_status(&mut self, alarm_id: usize) {
            for alarm in self.alarms.iter_mut(){
                if alarm.id == alarm_id{
                    alarm.active = !alarm.active;
                    println!("[DEBUG] alam {}, active {}", alarm.id, alarm.active);
                }
            }

            self.save_alarms().expect("Failed to save alarms");
            self.update_alarms_display();
        }

        fn delete_alarm(&mut self, alarm_id: usize) {
            println!("[DEBUG] Deleting alarms clock {} ID :{}",self.alarms.len(), alarm_id);
            //self.alarms.retain(|alarm: &AlarmClock| );
            if let Some(index) =  self.alarms.iter().position(|alarm: &AlarmClock| alarm.id == alarm_id){
                self.alarms.remove(index);
                println!("[DEBUG] Deleting alarms clock {}",self.alarms.len());
                println!("[DEBUG] alarms vec {:#?}",self.alarms);
                self.save_alarms().expect("Failed to save alarms");
                self.update_alarms_display();
            }

           
        }

        pub fn build_ui(&mut self, window: &ApplicationWindow) {
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
            self.widgets.i_name_ac.set_placeholder_text("Nom de l'alarme".into());
            self.widgets.i_song_link.set_placeholder_text("URL de la music".into());
            hbox_reveil.pack_start(&self.widgets.i_name_ac, true, true, 0);
            hbox_reveil.pack_start(&self.widgets.i_song_link, true, true, 0);
            hbox_reveil.pack_start(&self.widgets.p_cancel, true, true, 0);
            hbox_reveil.pack_start(&self.widgets.p_save, true, true, 0);

            self.widgets.g_alarm_clock.add(&hbox_reveil);

            vbox.pack_start(&hbox1, false, false, 10);
            vbox.pack_start(&hbox2, false, false, 10);
            vbox.pack_start(&self.widgets.g_alarm_clock, false, false, 10);
            vbox.pack_start(&hbox_rad_b, false, false, 20);
            vbox.pack_start(&self.widgets.alarms_container, true, true, 10); 

            window.add(&vbox);
            //window.show_all();

            // Connect signals
            
            
            // Update the time every second
            self.update_alarms_display();
            unsafe{self.update_time_labels()};
        }

        pub fn connect_signals(&mut self) {
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
                let mut view = view_clone.lock().unwrap();
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

            let view_clone: Arc<Mutex<View>> = view_rc.clone();
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

        pub fn on_marche_clicked(&self) {
            let current_radio = self.current_radio.clone();
            let music_player = self.music_player.clone();
            gtk::glib::MainContext::default().spawn_local(async move {
                if let Some(url) = current_radio.lock().unwrap().get_url() {
                    music_player.lock().unwrap().play_url(url.to_string()).await;
                } else {
                    println!("No radio selected");
                }
            });
        }

        pub fn on_arret_clicked(&self) {
            let music_player = self.music_player.clone();
            gtk::glib::MainContext::default().spawn_local(async move {
                music_player.lock().unwrap().stop().await;
            });
        }



        fn on_new_alarm_clicked(&self) {
            // Logic for adding new alarm
            println!("Ajouter un réveil button clicked");
            let horaire: MutexGuard<Horaire> = self.horaire.lock().unwrap();
            self.widgets.s_heur_box.set_text(&format!("{:02}", horaire.get_hour()));
            self.widgets.s_min_box.set_text(&format!("{:02}", horaire.get_min()));
            self.widgets.s_sec_box.set_text(&format!("{:02}", horaire.get_sec()));
            self.widgets.g_alarm_clock.show_all();
        }

        fn on_save_clicked(&mut self) {
            // Logic for saving alarm
            println!("Save button clicked");
            self.add_alarms();
            self.save_alarms().expect("Failed to save alarms");
            self.update_alarms_display();
            self.widgets.g_alarm_clock.hide();
        }

        pub fn on_cancel_clicked(&self) {
            // Logic for canceling alarm
            println!("Cancel button clicked");
            self.widgets.g_alarm_clock.hide();
        }

        pub fn on_radio_clicked(&mut self, id_radio: u8) {
            // Logic for radio button clicked
            match id_radio{
                1 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::FranceInfo),
                2 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::FranceInter),
                3 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::RTL),
                4 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::RireChanson),
                5 => self.current_radio.lock().unwrap().selected_radio = Some(RadioStation::Skyrock),
                _ => println!("Radio button {} clicked", id_radio),
            };
            println!("Radio button {} clicked", id_radio);
            
            
        }

        unsafe fn update_time_labels(&self) {
            let horaire_rc: Arc<Mutex<Horaire>> = self.horaire.clone();
            let sender: Sender<()> = self.sender.clone();
            
            timeout_add_seconds(1, move || {
                let mut horaire: MutexGuard<Horaire> = horaire_rc.lock().unwrap();
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
                widgets_rc.p_lcd_heure.set_text(&format!("{:02}", horaire.get_hour()));
                widgets_rc.p_lcd_min.set_text(&format!("{:02}", horaire.get_min()));
                widgets_rc.p_lcd_sec.set_text(&format!("{:02}", horaire.get_sec()));
                // widgets_rc.s_heur_box.set_text(&format!("{:02}", horaire.hour));
                // widgets_rc.s_min_box.set_text(&format!("{:02}", horaire.minute));
                // widgets_rc.s_sec_box.set_text(&format!("{:02}", horaire.second));
                ControlFlow::Continue
            });
        }
    }

    
}
