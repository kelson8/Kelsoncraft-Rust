
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

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};


use esp_hal::gpio::{Level, Output, OutputConfig};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// fn blink_leds() {
//
// }

// https://esp32.implrust.com/std-to-no-std/led.html
fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32 -o esp32-wroom-32e

    // LED blink time in ms, 1000 is 1 second.
    let led_blink_time = 1000;

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    // https://esp32.implrust.com/std-to-no-std/peripherals.html
    let peripherals = esp_hal::init(config);

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
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
