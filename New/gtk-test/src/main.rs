use gtk::{
    glib::{self, clone},
    prelude::*,
};

// https://github.com/gtk-rs/gtk4-rs/blob/main/examples/basics/main.rs
// https://github.com/gtk-rs/gtk4-rs/blob/main/examples/grid_packing/main.rs

// TODO Fix the GTK 4 test to work in here, no buttons or anything shows up.

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("net.kelsoncraft.gtk-test")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(app: &gtk::Application) {
    // let window = gtk::ApplicationWindow::new(application);

    // window.set_title(Some("First GTK Program"));
    // window.set_title("First GTK Program");
    // window.set_default_size(200, 120);

    // New code below from here
    // https://gtk-rs.org/gtk4-rs/git/book/hello_world.html

    // Create a button with label and margins
    let button = gtk::Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}
