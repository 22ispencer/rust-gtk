use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.magma.pumice";

fn main() -> glib::ExitCode  {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Run the application
    app.run()
}
