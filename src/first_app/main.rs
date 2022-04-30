use gtk::prelude::*;
use gtk::Application;
use gtk4 as gtk;

fn main() {
    // Create a new Application;
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Run the application
    app.run();
}
