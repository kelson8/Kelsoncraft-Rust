// https://developer.mamezou-tech.com/en/blogs/2025/05/19/using-rust-02/

use anyhow::Result;
use esp_idf_svc::{
    hal::{gpio::PinDriver, peripherals::Peripherals},
    sys::link_patches,
    log::EspLogger,
};
use std::{
    thread::sleep,
    time::Duration
};

// I got this working!
// I had to hold the BOOT button on the ESP32 while I was also running 'cargo run'

// Hmm, I'm not sure how to make this into another function...
// fn toggle_leds() -> Result<()> {
//     let peripherals = Peripherals::take().unwrap();
//     let mut blue_led = PinDriver::output(peripherals.pins.gpio26)?;
//
//     loop {
//         // blue_led.set_high().expect("Panic message");
//         blue_led.set_high()?;
//         sleep(Duration::from_millis(500));
//
//         // blue_led?.set_low().expect("Panic message");
//         blue_led.set_low()?;
//         sleep(Duration::from_millis(500));
//     }
//
//     // This line will not be executed, but is required for type compatibility
//     // Due to Rust's static checks, it must be written after the loop
//     #[allow(unreachable_code)]
//     Ok(())
// }

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    // New
    let peripherals = Peripherals::take().unwrap();

    // toggle_leds()?;

    let mut blue_led = PinDriver::output(peripherals.pins.gpio26)?;

    loop {
        // blue_led.set_high().expect("Panic message");
        blue_led.set_high()?;
        sleep(Duration::from_millis(500));

        // blue_led?.set_low().expect("Panic message");
        blue_led.set_low()?;
        sleep(Duration::from_millis(500));
    }

    // log::info!("Hello, world!");

    // This line will not be executed, but is required for type compatibility
    // Due to Rust's static checks, it must be written after the loop
    #[allow(unreachable_code)]
    Ok(())
}
