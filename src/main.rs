use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
mod viewmod;
use viewmod::view::View;
mod modelmod;


fn main() {
    let application: Application = Application::new(
        Some("com.example.alarm_clock"),
        Default::default(),
    );//.expect("failed to initialize GTK application");

    application.connect_activate(|app: &Application| {
        let window: ApplicationWindow = ApplicationWindow::new(app);
        let view: View = View::new();
        view.build_ui(&window);
        //view.connect_signals();
        window.show_all();

    });
    println!("[DEBUG] Application  build  ");
    
    application.run();
    println!("[DEBUG] Application running ");
}