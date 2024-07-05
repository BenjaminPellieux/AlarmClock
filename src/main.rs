use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
mod controllermod;
mod viewmod;
use viewmod::view::View;
mod modelmod;
mod widgetmod;
mod musicmod;

// #[tokio::main]
// async fn main() {
//     let application: Application = Application::new(
//         Some("com.my.alarm_clock"),
//         Default::default(),
//     );//.expect("failed to initialize GTK application");

//     application.connect_activate(|app: &Application| {
//         let window: ApplicationWindow = ApplicationWindow::new(app);
//         let mut view: View = View::new();
//         let _res: () = match view.load_alarms() {
//             Ok(_res) => println!("[INFO]  File loaded"),
//             Err(error) => println!("[ERROR] Failed to load alarms {error:?}"),
//         };
//         view.build_ui(&window);
//         view.connect_signals();
//         //view.connect_signals();
//         window.show_all();
//         view.on_cancel_clicked();
//         view.on_radio_clicked(1);
//         view.on_arret_clicked();

//     });
//     application.run();
// }


fn main() {
    let app = gtk::Application::new(
        Some("com.example.AlarmClock"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);

        // Créer la vue
        let view: std::sync::Arc<std::sync::Mutex<View>> = View::new();

        // Charger les alarmes
        {
            let view = view.lock().unwrap();
            match view.controller.as_ref().unwrap().lock().unwrap().load_alarms() {
                Ok(_) => println!("[INFO]  File loaded"),
                Err(error) => println!("[ERROR] Failed to load alarms {error:?}"),
            };
        }

        // Construire l'interface utilisateur
        {
            let mut view = view.lock().unwrap();
            view.build_ui(&window);
            view.connect_signals();
        }

        // Afficher la fenêtre
        window.show_all();
    });

    app.run();
}