use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
mod viewmod;
use viewmod::view::View;
mod modelmod;
mod widgetmod;
mod musicmod;

#[tokio::main]
async fn main() {
    let application: Application = Application::new(
        Some("com.example.alarm_clock"),
        Default::default(),
    );//.expect("failed to initialize GTK application");

    application.connect_activate(|app: &Application| {
        let window: ApplicationWindow = ApplicationWindow::new(app);
        let mut view: View = View::new();
        let _res: () = match view.load_alarms() {
            Ok(_res) => println!("[INFO]  File loaded"),
            Err(error) => println!("[ERROR] Failed to load alarms {error:?}"),
        };
        view.build_ui(&window);
        view.connect_signals();
        //view.connect_signals();
        window.show_all();
        view.on_cancel_clicked();
        view.on_radio_clicked(1);

    });
    println!("[DEBUG] Application  build  ");
    
    application.run();
    println!("[DEBUG] Application running ");
}