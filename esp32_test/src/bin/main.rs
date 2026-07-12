
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

use esp_hal::delay::Delay;

use esp_hal::gpio::{Level, Output, OutputConfig, Input, InputConfig, Pull};

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
    mono_font::{MonoTextStyle, MonoTextStyleBuilder, ascii::FONT_6X10, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
    text::{Baseline, Text},
};

// New for separating this into multiple files.
mod oled_text;

use oled_text::font_for_size;

use oled_text::{
    draw_line1,
    draw_line2,
    draw_line3,
    draw_line4,
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
// fn draw_line1(text_size: u8, x: i32, text: &str) {
//
// }

// fn font_for_size(text_size: u8) -> &'static embedded_graphics::mono_font::MonoFont<'static> {
//     match text_size {
//         0..=1 => &FONT_6X10,
//         _ => &FONT_10X20,
//     }
// }



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
    let _led_blink_time = 1000;

    // Button delay time for when the buttons are pressed in ms.
    let led_button_delay_time = 25;

    // Size for the font, not sure how this works.
    let text_size = 1;

    let config = esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max());

    // https://docs.rs/esp-hal/latest/esp_hal/gpio/struct.Input.html#method.new
    let input_config = InputConfig::default().with_pull(Pull::Up);

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
    draw_line1(&mut display, text_size, 5, "Hello, world!").unwrap();
    draw_line2(&mut display, text_size, 5, "Line 2").unwrap();
    draw_line3(&mut display, text_size, 5, "Line 3").unwrap();
    draw_line4(&mut display, text_size, 5, "Line 4").unwrap();

    // Write data to the display
    display.flush().await.unwrap();
    //

    // let mut led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());
    // Power LED
    let mut power_led = Output::new(peripherals.GPIO26, Level::High, OutputConfig::default());

    // Always turn on the power LED by default
    power_led.set_high();

    // Misc LEDs, off by default.
    let mut blue_led = Output::new(peripherals.GPIO25, Level::Low, OutputConfig::default());
    let mut green_led = Output::new(peripherals.GPIO27, Level::Low, OutputConfig::default());
    // let mut blue_led = PinDriver::output(peripherals.pins.gpio25)?;
    // let mut green_led = PinDriver::output(peripherals.pins.gpio27)?;

    // LED status, I may do something like in my C++ code
    // https://github.com/kelson8/KCNet-ESP32-SSD1306/blob/main/src/main.cpp#L41-L47
    // https://github.com/kelson8/KCNet-ESP32-SSD1306/blob/main/src/main.cpp#L92-L124
    let mut blue_led_toggled = false;
    let mut green_led_toggled = false;


    // https://dev.to/vaishnav_sabari_girish/embedded-rust-on-the-esp32-interfacing-button-with-esp32-kd9
    // Buttons
    let blue_led_button = Input::new(peripherals.GPIO12, input_config);
    let green_led_button = Input::new(peripherals.GPIO14, input_config);
    let delay = Delay::new();

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
        // Not sure if this is needed here.
        // Well this was just causing a 1 second delay for the program.
        // Timer::after(Duration::from_secs(1)).await;



        //-----
        // Blink the LEDs, TODO Move this into a function.
        // green_led.toggle();
        // blue_led.toggle();
        // blocking_delay(Duration::from_millis(led_blink_time));

        //-----
        // Buttons
        // TODO Fix these to work right.
        // TODO Move this into a single function later, for now this is how this'll work..
        // TODO Make this store the button state.
        // Blue LED handling
        if blue_led_button.is_low() && !blue_led_toggled {
            blue_led.set_high();
            // info!("Blue LED on");
            delay.delay_millis(led_button_delay_time);
            blue_led_toggled = true;
        } else {
            blue_led.set_low();
            blue_led_toggled = false;
            // info!("Blue LED off");
            // delay.delay_millis(led_button_delay_time);
        }

        // Green LED handling
        if green_led_button.is_low() && !green_led_toggled {
            green_led.set_high();
            // info!("Green LED on");
            delay.delay_millis(led_button_delay_time);
            green_led_toggled = true;
        } else {
            green_led.set_low();
            green_led_toggled = false;
            // info!("Green LED off");
            // delay.delay_millis(led_button_delay_time);
        }


    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
