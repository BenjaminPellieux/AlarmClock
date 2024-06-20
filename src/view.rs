// view.rs

use crate::model::{AlarmClock, Horaire};

use qt_core::{qs, QBox, QObject, QUrl, SlotNoArgs, SlotOfInt};
// use qt_core::connection::*;
use qt_widgets::{QApplication, QBoxLayout, QGroupBox, QHBoxLayout, QLCDNumber, QLineEdit, QPushButton,
    QRadioButton, QVBoxLayout, QWidget, QSpinBox, QLabel, QCheckBox, QMessageBox};
    
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
pub struct MyView {
    myradio_url: Vec<&'static str>,
    alarm_clock_list: Vec<AlarmClock>,
    heure_act: Rc<RefCell<Horaire>>,
    current_radio: u8,

    widgets: Widgets,
    parent: QBox<QObject>,
}

#[derive(Debug)]
struct Widgets {
    g_alarm_clock: QBox<QGroupBox>,
    g_alarm_clock_tab: Option<QBox<QGroupBox>>,
    s_heur_box: QBox<QSpinBox>,
    s_min_box: QBox<QSpinBox>,
    s_sec_box: QBox<QSpinBox>,
    i_song_link: QBox<QLineEdit>,
    p_cancel: QBox<QPushButton>,
    p_save: QBox<QPushButton>,
    p_button_marche: QBox<QPushButton>,
    p_button_arret: QBox<QPushButton>,
    p_button_add_alarm_clock: QBox<QPushButton>,
    p_button_del_alarm: Option<QBox<QPushButton>>,
    p_rad_b1: QBox<QRadioButton>,
    p_rad_b2: QBox<QRadioButton>,
    p_rad_b3: QBox<QRadioButton>,
    p_rad_b4: QBox<QRadioButton>,
    p_rad_b5: QBox<QRadioButton>,
    p_lcd_heure: QBox<QLCDNumber>,
    p_lcd_min: QBox<QLCDNumber>,
    p_lcd_sec: QBox<QLCDNumber>,
    p_hbox_reveil: QBox<QHBoxLayout>,
    p_vbox_song: QBox<QVBoxLayout>,
    p_vbox_p: QBox<QVBoxLayout>,
}

impl MyView {
    pub unsafe fn new() -> Rc<RefCell<Self>> {
        let parent: QBox<QObject> = QObject::new_0a();
        let view: Rc<RefCell<MyView>> = Rc::new(RefCell::new(Self {
            myradio_url: vec![
                "http://direct.franceinfo.fr/live/franceinfo-midfi.mp3",
                "http://direct.franceinter.fr/live/franceinter-midfi.mp3",
                "http://streaming.radio.rtl.fr/rtl-1-44-128",
                "http://cdn.nrjaudio.fm/audio1/fr/30401/mp3_128.mp3",
                "http://icecast.skyrock.net/s/natio_mp3_128k",
            ],
            alarm_clock_list: vec![],
            heure_act: Rc::new(RefCell::new(Horaire::new())),
            current_radio: 0,
            // radio_player: QMediaPlayer::new_0a(),
            widgets: Widgets {
                g_alarm_clock: QGroupBox::from_q_string(&qs("Nouveau réveil")),
                g_alarm_clock_tab: None,
                s_heur_box: QSpinBox::new_0a(),
                s_min_box: QSpinBox::new_0a(),
                s_sec_box: QSpinBox::new_0a(),
                i_song_link: QLineEdit::new(),
                p_cancel: QPushButton::from_q_string(&qs("Annuler")),
                p_save: QPushButton::from_q_string(&qs("Sauvegarder")),
                p_button_marche: QPushButton::from_q_string(&qs("Marche")),
                p_button_arret: QPushButton::from_q_string(&qs("Arrêt")),
                p_button_add_alarm_clock: QPushButton::from_q_string(&qs("Ajouter un réveil")),
                p_button_del_alarm: None,
                p_rad_b1: QRadioButton::from_q_string(&qs("France Info")),
                p_rad_b2: QRadioButton::from_q_string(&qs("France Inter")),
                p_rad_b3: QRadioButton::from_q_string(&qs("RTL")),
                p_rad_b4: QRadioButton::from_q_string(&qs("Rire & Chanson")),
                p_rad_b5: QRadioButton::from_q_string(&qs("Skyrock")),
                p_lcd_heure: QLCDNumber::new(),
                p_lcd_min: QLCDNumber::new(),
                p_lcd_sec: QLCDNumber::new(),
                p_hbox_reveil: QHBoxLayout::new_0a(),
                p_vbox_song: QVBoxLayout::new_0a(),
                p_vbox_p: QVBoxLayout::new_0a()
            },
            parent,
        }));

        view.borrow().add_ui_context();
        view.borrow().add_ui_new_alarm_field();
        view.borrow().add_ui_layout();
        let tmp_view: Rc<RefCell<MyView>> = Rc::clone(&view);
        view.borrow().add_ui_control(tmp_view);
        view
    }

    unsafe fn add_ui_context(&self) {
        println!("[INFO ADD UI context]");
        // self.widgets.p_lcd_heure.set_segment_style(qt_core::q_enum::QLCDNumberSegmentStyle::Flat);
        // self.widgets.p_lcd_min.set_segment_style(qt_core::q_enum::QLCDNumberSegmentStyle::Flat);
        // self.widgets.p_lcd_sec.set_segment_style(qt_core::q_enum::QLCDNumberSegmentStyle::Flat);
    }

    unsafe fn add_ui_layout(&self) {
        let window = QWidget::new_0a();
        let layout = QVBoxLayout::new_1a(&window);
        layout.add_widget(&self.widgets.p_lcd_heure);
        layout.add_widget(&self.widgets.p_lcd_min);
        layout.add_widget(&self.widgets.p_lcd_sec);
        layout.add_widget(&self.widgets.p_button_marche);
        layout.add_widget(&self.widgets.p_button_add_alarm_clock);
        layout.add_widget(&self.widgets.p_button_arret);
        layout.add_widget(&self.widgets.g_alarm_clock);
        if let Some(g_alarm_clock_tab) = &self.widgets.g_alarm_clock_tab {
            layout.add_widget(g_alarm_clock_tab);
        }
        layout.add_widget(&self.widgets.p_rad_b1);
        layout.add_widget(&self.widgets.p_rad_b2);
        layout.add_widget(&self.widgets.p_rad_b3);
        layout.add_widget(&self.widgets.p_rad_b4);
        layout.add_widget(&self.widgets.p_rad_b5);
        
        window.show();
    }

    unsafe fn add_ui_control(&self, view: Rc<RefCell<Self>>) {


        let parent: &QBox<QObject> = &self.parent;

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_radio_clicked_1: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_radio_clicked(1);
        });
        self.widgets.p_rad_b1.clicked().connect(&on_radio_clicked_1);

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_radio_clicked_2: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_radio_clicked(2);
        });
        self.widgets.p_rad_b2.clicked().connect(&on_radio_clicked_2);

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_radio_clicked_3: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_radio_clicked(3);
        });
        self.widgets.p_rad_b3.clicked().connect(&on_radio_clicked_3);

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_radio_clicked_4: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_radio_clicked(4);
        });
        self.widgets.p_rad_b4.clicked().connect(&on_radio_clicked_4);

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_radio_clicked_5: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_radio_clicked(5);
        });
        self.widgets.p_rad_b5.clicked().connect(&on_radio_clicked_5);

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_marche_clicked: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_marche_clicked();
        });
        self.widgets.p_button_marche.clicked().connect(&on_marche_clicked);

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_new_alarm_clicked: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_new_alarm_clicked();
        });
        self.widgets.p_button_add_alarm_clock.clicked().connect(&on_new_alarm_clicked);

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_arret_clicked: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_arret_clicked();
        });
        self.widgets.p_button_arret.clicked().connect(&on_arret_clicked);

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_cancel_clicked: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_cancel_clicked();
        });
        self.widgets.p_cancel.clicked().connect(&on_cancel_clicked);

        let view_clone: Rc<RefCell<MyView>> = Rc::clone(&view);
        let on_save_clicked: QBox<SlotNoArgs> = SlotNoArgs::new(parent, move || {
            view_clone.borrow_mut().on_save_clicked();
        });
        self.widgets.p_save.clicked().connect(&on_save_clicked);
    }


    unsafe fn add_ui_new_alarm_field(&self) {
        self.widgets.s_heur_box.set_range(0, 23);
        self.widgets.s_min_box.set_range(0, 59);
        self.widgets.s_sec_box.set_range(0, 59);
        self.widgets.p_vbox_song.add_widget(&QLabel::from_q_string(&qs("Veuillez selectioner une radio ou entrer l'URL youtube d'une musique")));
        self.widgets.p_vbox_song.add_widget(&self.widgets.i_song_link);
        self.widgets.p_hbox_reveil.add_widget(&self.widgets.s_heur_box);
        self.widgets.p_hbox_reveil.add_widget(&QLabel::from_q_string(&qs("H")));
        self.widgets.p_hbox_reveil.add_widget(&self.widgets.s_min_box);
        self.widgets.p_hbox_reveil.add_widget(&QLabel::from_q_string(&qs("min")));
        self.widgets.p_hbox_reveil.add_widget(&self.widgets.s_sec_box);
        self.widgets.p_hbox_reveil.add_widget(&QLabel::from_q_string(&qs("s")));
        self.widgets.p_hbox_reveil.add_layout_1a(&self.widgets.p_vbox_song);
        self.widgets.p_hbox_reveil.add_widget(&self.widgets.p_cancel);
        self.widgets.p_hbox_reveil.add_widget(&self.widgets.p_save);
    }

    fn on_radio_clicked(&mut self, id: u8) {
        println!("Radio clicked {}", id);
        self.current_radio = id;
        // self.radio_player.set_media(QUrl::from_q_string(&qs(self.myradio_url[id as usize - 1])));
    }

    fn on_marche_clicked(&self) {
        // println!("[INFO] radio URL : {:?}", self.radio_player.is_audio_available());
        // if !self.radio_player.is_audio_available() {
        //     QMessageBox::information_2a(&self.widgets.g_alarm_clock, &qs("Radio"), &qs("Aucune radio selectionnée"));
        // } else {
            // self.radio_player.play();
        // }
        println!("[DEBUG] on_marche_clicked");
    }

    unsafe fn on_cancel_clicked(&self) {
        self.widgets.g_alarm_clock.hide();
        self.widgets.s_heur_box.set_value(0);
        self.widgets.s_min_box.set_value(0);
        self.widgets.s_sec_box.set_value(0);
        self.widgets.i_song_link.set_text(&qs(""));
    }

    fn on_status_alarm_changed(&self, alarm: &mut AlarmClock) {
        alarm.status = !alarm.status;
    }

    unsafe fn on_delete_alarm_clicked(&mut self, alarm: &AlarmClock) {
        self.alarm_clock_list.retain(|a| a.id != alarm.id);
        self.add_ui_alarm_clock();
    }

    unsafe fn on_save_clicked(&mut self) {
        let h = self.widgets.s_heur_box.value();
        let m = self.widgets.s_min_box.value();
        let s = self.widgets.s_sec_box.value();
        let alarm_id = self.alarm_clock_list.len();
        let link;

        if !self.widgets.i_song_link.text().is_empty() {
            link = self.widgets.i_song_link.text().to_std_string();
            self.alarm_clock_list.push(AlarmClock::new(h as u8, m as u8, s as u8, link, false, alarm_id));
        } else {
            link = self.myradio_url[self.current_radio as usize - 1].to_string();
            self.alarm_clock_list.push(AlarmClock::new(h as u8, m as u8, s as u8, link, true, alarm_id));
        }

        let msg = format!("Votre réveil à {:02}:{:02}:{:02} a bien été enregistré !", h, m, s);
        //QMessageBox::information_2a(&self.widgets.g_alarm_clock, &qs("Message"), &qs(&msg));
        self.on_cancel_clicked();
        self.add_ui_alarm_clock();
    }

    unsafe fn on_new_alarm_clicked(&self) {
        self.widgets.g_alarm_clock.show()
    }

    fn on_arret_clicked(&self) {
        // self.radio_player.pause();
        println!("[DEBUG] Player pause");
    }

    pub unsafe fn update(&self) -> () {
        self.heure_act.borrow_mut().update_time();
        self.widgets.p_lcd_heure.display_int(self.heure_act.borrow().hour as i32);
        self.widgets.p_lcd_min.display_int(self.heure_act.borrow().minute as i32);
        self.widgets.p_lcd_sec.display_int(self.heure_act.borrow().second as i32);
        for alarm in &self.alarm_clock_list {
            if alarm.status && alarm.to_compare(&self.heure_act.borrow()) {
                if alarm.is_radio {
                    println!("[DEBUG] Alarm is radio true ");
                    // self.radio_player.set_media(QUrl::copy_from_q_string(&qs(&alarm.link)));
                } else {
                    println!("[DEBUG] Alarm is radio false ");
                    // self.radio_player.set_media(QUrl::from_local_file(&qs(&alarm.song)));
                }
                // self.radio_player.play();
            }
        }
    }

    unsafe fn add_ui_alarm_clock(&mut self) {
        if let Some(g_alarm_clock_tab) = &self.widgets.g_alarm_clock_tab {
            g_alarm_clock_tab.delete_later();
        }

        let g_alarm_clock_tab = QGroupBox::from_q_string(&qs("Alarmes Sauvegardées"));
        let p_alarm_clock_layout = QVBoxLayout::new_0a();

        for alarm in &self.alarm_clock_list {
            let alarm_item_layout: QBox<QHBoxLayout> = QHBoxLayout::new_0a();
            let check_box_enable: QBox<QCheckBox> = QCheckBox::new();

            let lcd_heur: QBox<QLCDNumber> = QLCDNumber::new();
            lcd_heur.display_int(alarm.horaire.hour as i32);
            //lcd_heur.set_segment_style(qt_core::q_enum::QLCDNumberSegmentStyle::Flat);

            let lcd_min: QBox<QLCDNumber> = QLCDNumber::new();
            lcd_min.display_int(alarm.horaire.minute as i32);
            //lcd_min.set_segment_style(qt_core::q_enum::QLCDNumberSegmentStyle::Flat);

            let lcd_sec: QBox<QLCDNumber> = QLCDNumber::new();
            lcd_sec.display_int(alarm.horaire.second as i32);
            //lcd_sec.set_segment_style(qt_core::q_enum::QLCDNumberSegmentStyle::Flat);

            check_box_enable.set_checked(alarm.status);

            let p_button_del_alarm = QPushButton::from_q_string(&qs("Supprimer"));
            // p_button_del_alarm.clicked().connect(&SlotNoArgs::new(self, move || {
            //     self.on_delete_alarm_clicked(alarm);
            // }));

            alarm_item_layout.add_widget(&lcd_heur);
            alarm_item_layout.add_widget(&QLabel::from_q_string(&qs("H")));
            alarm_item_layout.add_widget(&lcd_min);
            alarm_item_layout.add_widget(&QLabel::from_q_string(&qs("min")));
            alarm_item_layout.add_widget(&lcd_sec);
            alarm_item_layout.add_widget(&QLabel::from_q_string(&qs("s")));
            alarm_item_layout.add_widget(&check_box_enable);
            alarm_item_layout.add_widget(&p_button_del_alarm);

            p_alarm_clock_layout.add_layout_1a(&alarm_item_layout);
        }

        g_alarm_clock_tab.set_layout(&p_alarm_clock_layout);
        self.widgets.g_alarm_clock_tab = Some(g_alarm_clock_tab);
        self.widgets.p_vbox_p.add_widget(self.widgets.g_alarm_clock_tab.as_ref().unwrap());
    }
}

