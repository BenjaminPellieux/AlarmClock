mod model;

use qt_core::{qs, slot, ContextMenuPolicy, QBox, QObject, QPoint, SlotNoArgs};
use qt_widgets::{
    cpp_core::Ptr, QApplication, QCheckBox, QGroupBox, QHBoxLayout, QLabel, QLCDNumber,
    QPushButton, QRadioButton, QSpinBox, QVBoxLayout, QWidget,
};
use std::rc::Rc;
use std::cell::RefCell;




fn main() {
    QApplication::init(|_| unsafe{
        let alarm_clock = model::AlarmClock::new();

        let window = unsafe { QWidget::new_0a() };
        window.set_window_title(&qt_core::qs("Alarm Clock"));

        let main_layout = unsafe { QVBoxLayout::new_0a() };

        let time_label = unsafe { QLabel::new() };
        let time_label_ref: *mut QLabel = time_label.as_mut_raw_ptr();
        main_layout.add_widget(time_label.into_ptr());

        // let update_time_slot = Slot::new(alarm_clock => move || {
        //     alarm_clock.update_time();
        //     unsafe {
        //         time_label_ref.set_text(&qt_core::qs(alarm_clock.current_time.borrow().as_str()));
        //     }
        // });

        let button = unsafe { QPushButton::new() };
        button.set_text(&qt_core::qs("Add Alarm"));
        // unsafe {
        //     button.clicked().connect(&update_time_slot);
        // }
        main_layout.add_widget(button.into_ptr());

        let alarm_list_layout = unsafe { QVBoxLayout::new_0a() };

        for alarm in alarm_clock.alarms.borrow().iter() {
            let alarm_item_layout = unsafe { QHBoxLayout::new_0a() };

            let hour_lcd = unsafe { QLCDNumber::new() };
            hour_lcd.display_int(alarm.hour as i32);
            alarm_item_layout.add_widget(hour_lcd.into_ptr());

            let minute_lcd = unsafe { QLCDNumber::new() };
            minute_lcd.display_int(alarm.minute as i32);
            alarm_item_layout.add_widget(minute_lcd.into_ptr());

            let second_lcd = unsafe { QLCDNumber::new() };
            second_lcd.display_int(alarm.second as i32);
            alarm_item_layout.add_widget(second_lcd.into_ptr());

            let check_box = unsafe { QCheckBox::new() };
            check_box.set_checked(alarm.status);
            alarm_item_layout.add_widget(check_box.into_ptr());

            let delete_button = unsafe { QPushButton::new() };
            delete_button.set_text(&qt_core::qs("Delete"));
            alarm_item_layout.add_widget(delete_button.into_ptr());

            alarm_list_layout.add_layout_1a(alarm_item_layout.into_ptr());
        }

        main_layout.add_layout_1a(alarm_list_layout.into_ptr());
        unsafe {
            window.set_layout(main_layout.into_ptr());
        }

        window.show();

        QApplication::exec()
    });
}
