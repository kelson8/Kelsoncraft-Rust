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

// I2C
// use esp_hal::i2c::master::Config as I2cConfig; // for convenience, importing as alias
// use esp_hal::i2c::master::I2c;
// use esp_hal::time::Rate;
//
// // OLED
// use ssd1306::{I2CDisplayInterface, Ssd1306Async, prelude::*};
//
// // Embedded Graphics
// use embedded_graphics::{
//     mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
//     pixelcolor::BinaryColor,
//     prelude::Point,
//     prelude::*,
//     text::{Baseline, Text},
// };


// I got this working!
// I had to hold the BOOT button on the ESP32 while I was also running 'cargo run'

// Well, I tried to add the ssd1306, embedded-graphics, esp-hal, and tokio creates.
// But this just broke the build I guess.

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

    // Init I2C
    // let i2c_bus = I2c::new(
    //     peripherals.i2c0,
    //     // I2cConfig is alias of esp_hal::i2c::master::I2c::Config
    //     I2cConfig::default().with_frequency(Rate::from_khz(400)),
    // )
    //     .unwrap()
    //     .with_scl(peripherals.pins.gpio22)?
    //     .with_sda(peripherals.pins.gpio21)?
    //     .into_async();

    // Init SSD1306
    // let interface = I2CDisplayInterface::new(i2c_bus);
    // // initialize the display
    // let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    // .into_buffered_graphics_mode();
    // display.init().await.unwrap();

    // toggle_leds()?;

    // Power LED
    let mut red_led = PinDriver::output(peripherals.pins.gpio26)?;

    // Always turn on the power LED by default
    red_led.set_high()?;

    // Misc LEDs
    // let mut blue_led = PinDriver::output(peripherals.pins.gpio25)?;
    // let mut green_led = PinDriver::output(peripherals.pins.gpio27)?;

    // Blink the other LEDs
    // loop {
    //     blue_led.set_high()?;
    //     green_led.set_high()?;
    //     sleep(Duration::from_millis(500));
    //
    //     blue_led.set_low()?;
    //     green_led.set_low()?;
    //     sleep(Duration::from_millis(500));
    // }

    // log::info!("Hello, world!");

    // This line will not be executed, but is required for type compatibility
    // Due to Rust's static checks, it must be written after the loop
    #[allow(unreachable_code)]
    Ok(())
}
