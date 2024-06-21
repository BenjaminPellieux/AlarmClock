use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    // Initialize GTK application
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new top level window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("My GTK App");
    window.set_default_size(350, 70);

    // Create a new button with label
    let button = Button::with_label("Click me!");

    // Connect button signal to callback function
    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    // Add the button to the window
    window.add(&button);

    // Connect the window delete event to the main quit function
    // window.connect_delete_event(|_, _| {
    //     gtk::main_quit()
    // });

    // Show all widgets within the window
    window.show_all();

    // Start the GTK main loop
    gtk::main();
}
