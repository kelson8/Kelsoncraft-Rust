
// https://esp32.implrust.com/std-to-no-std/no-std.html
#![no_std]

// https://esp32.implrust.com/std-to-no-std/no-main.html
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

// New for async
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Instant};
//

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
// use esp_hal::time::{Duration, Instant};


use esp_hal::gpio::{Level, Output, OutputConfig};

// For SSD1306
// https://esp32.implrust.com/oled/hello-rust/index.html
// I2C
use esp_hal::i2c::master::Config as I2cConfig; // for convenience, importing as alias
use esp_hal::i2c::master::I2c;
use esp_hal::time::Rate;

// OLED
use ssd1306::{I2CDisplayInterface, Ssd1306Async, prelude::*};

// Embedded Graphics
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
    text::{Baseline, Text},
};
//


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}


// fn blink_leds() {
//
// }

// https://esp32.implrust.com/std-to-no-std/led.html
// TODO Test if this will work with async
fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}

// TODO Move the oled display into here for my functions.
// async fn oled_display() {
//     let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
//         .into_buffered_graphics_mode();
//     display.init().await.unwrap();
// }

// Adapted from my C++ functions
// https://github.com/kelson8/KCNet-ESP32-SSD1306/blob/main/src/oled-functions.cpp

// TODO Fix these to work
// Y values:
// Line 1: 10
// Line 2: 25
// Line 3: 40
// Line 4: 55
fn draw_line1(text_size: u8, x: i32, text: &str) {

}

//

// Original main function, without esp_rtos and async support.
// #[main]

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
// fn main() -> ! {
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32 -o esp32-wroom-32e

    // LED blink time in ms, 1000 is 1 second.
    let led_blink_time = 1000;

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    // https://esp32.implrust.com/std-to-no-std/peripherals.html
    let peripherals = esp_hal::init(config);


    // For async
    // https://github.com/ImplFerris/esp32-projects/blob/main/hello-oled/src/bin/main.rs
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");
    // TODO: Spawn some tasks
    let _ = spawner;
    //

    // Init I2C
    let i2c_bus = I2c::new(
        peripherals.I2C0,
        // I2cConfig is alias of esp_hal::i2c::master::I2c::Config
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
        .unwrap()
        .with_scl(peripherals.GPIO22)
        .with_sda(peripherals.GPIO21)
        .into_async();
    //

    // Init SSD1306
    let interface = I2CDisplayInterface::new(i2c_bus);
    // initialize the display
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();

    //

    // Display text on SSD1306
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello, Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();


    // Write data to the display
    display.flush().await.unwrap();
    //

    // let mut led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());
    // Power LED
    let mut power_led = Output::new(peripherals.GPIO26, Level::High, OutputConfig::default());

    // Always turn on the power LED by default
    power_led.set_high();

    // Misc LEDs
    let mut blue_led = Output::new(peripherals.GPIO25, Level::High, OutputConfig::default());
    let mut green_led = Output::new(peripherals.GPIO27, Level::High, OutputConfig::default());
    // let mut blue_led = PinDriver::output(peripherals.pins.gpio25)?;
    // let mut green_led = PinDriver::output(peripherals.pins.gpio27)?;

    // The following pins are used to bootstrap the chip. They are available
    // for use, but check the datasheet of the module for more information on them.
    // - GPIO0
    // - GPIO2
    // - GPIO5
    // - GPIO12
    // - GPIO15
    // These GPIO pins are in use by some feature of the module and should not be used.
    let _ = peripherals.GPIO6;
    let _ = peripherals.GPIO7;
    let _ = peripherals.GPIO8;
    let _ = peripherals.GPIO9;
    let _ = peripherals.GPIO10;
    let _ = peripherals.GPIO11;
    let _ = peripherals.GPIO16;
    let _ = peripherals.GPIO20;

    loop {
        // Blink the LEDs, TODO Move this into a function.
        green_led.toggle();
        blue_led.toggle();
        blocking_delay(Duration::from_millis(led_blink_time));

        // Not sure if this is needed here.
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
