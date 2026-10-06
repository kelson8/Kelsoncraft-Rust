// use fltk::{app, prelude::*, window::Window};
// use fltk::{prelude::*, *};

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};


// https://fltk-rs.github.io/fltk-book/Home.html
// This seems easy enough to use once I learn more Rust.
// I could make a UI for something.

// https://fltk-rs.github.io/fltk-book/Widgets.html

// https://fltk-rs.github.io/fltk-book/Buttons.html

// TODO Look into this for SQLite and an ORM
// https://github.com/tokio-rs/toasty
// Another ORM like library
// https://github.com/transact-rs/sqlx

// SQLite specifc bindings
// https://github.com/rusqlite/rusqlite

// Date and time
// https://github.com/chronotope/chrono

// Full list of Rust applications and libraries
// https://github.com/rust-unofficial/awesome-rust#database-1

fn main() {
    let app = app::App::default();

    let mut text_shown = false;


    let mut my_window = Window::new(400, 300, 400, 300, "Test Window");
    let mut frame = Frame::default().with_size(200,100).center_of(&my_window);
    let mut button = Button::new(160, 200, 80, 40, "Test Button");

    my_window.end();
    my_window.show();

    // This works for toggling the text display.
    // button.set_callback(move |_| frame.set_label("Test from Button"));
    button.set_callback(move |_| {
        text_shown = !text_shown;
        if(text_shown) {
            frame.set_label("Test from Button");
        } else {
            frame.set_label("");
        }

    });

    app.run().unwrap();
}
