//! Minimal stateful application using the high-level `dear-app` runtime.

use dear_app::{AppConfig, RunError, imgui::Condition, run_ui};

// Taken from this example here
// https://github.com/Latias94/dear-imgui-rs/blob/main/examples/00-quickstart/hello_world.rs

// Other Git repos for this
// https://github.com/Yatekii/imgui-wgpu-rs
// 

const WINDOW_TITLE: &'static str = "KCNet ImGui";
const WINDOW_WIDTH: f32 = 720.0;
const WINDOW_HEIGHT: f32 = 480.0;

fn main() -> Result<(), RunError> {
    let config = AppConfig {
        window_title: WINDOW_TITLE.to_owned(),
        window_size: (WINDOW_WIDTH.into(), WINDOW_HEIGHT.into()),
        ..Default::default()
    };
    let mut _counter = 0;
    let mut show_hello = true;

    let main_menu_title = "Main Menu";

    run_ui(config, move |ui| {
        if show_hello {
            ui.window(main_menu_title)
                .opened(&mut show_hello)
                .size([360.0, 180.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text("Welcome to KCNet ImGui in Rust");
                    ui.text("I may use this for modding ReVC\n or other games in the future.");


                    // if ui.button("Click me") {
                    //     counter += 1;
                    // }
                    // ui.same_line();
                    // ui.text(format!("Counter: {counter}"));


                    ui.separator();


                });
        } else {
            ui.window(main_menu_title)
                .size([280.0, 120.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text("Hello window is closed.");
                    if ui.button("Reopen Hello") {
                        show_hello = true;
                    }
                });
        }
    })
}