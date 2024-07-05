use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
mod viewmod;
use viewmod::view::View;
mod modelmod;
mod widgetmod;
mod musicmod;

#[tokio::main]
async fn main() {
    // Crée une nouvelle application GTK avec l'identifiant "com.my.alarm_clock".
    let application: Application = Application::new(
        Some("com.my.alarm_clock"),
        Default::default(),
    );

    // Connecte la fonction de rappel pour l'activation de l'application.
    application.connect_activate(|app: &Application| {
        // Crée une nouvelle fenêtre de l'application.
        let window: ApplicationWindow = ApplicationWindow::new(app);
        
        // Crée une nouvelle instance de la vue.
        let mut view: View = View::new();
        
        // Charge les alarmes sauvegardées, s'il y en a.
        let _res: () = match view.load_alarms() {
            Ok(_res) => println!("[INFO]  File loaded"),
            Err(error) => println!("[ERROR] Failed to load alarms {error:?}"),
        };

        // Construit l'interface utilisateur de la vue.
        view.build_ui(&window);
        
        // Connecte les signaux (événements utilisateur) à la vue.
        view.connect_signals();
        
        // Affiche tous les widgets de la fenêtre.
        window.show_all();
        
        // Exemples d'appels de fonctions sur la vue.
        view.on_cancel_clicked();
        view.on_radio_clicked(1);
        view.on_arret_clicked();
    });

    // Exécute l'application.
    application.run();
}
