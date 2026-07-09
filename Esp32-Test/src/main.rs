#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};

// https://dev.to/vaishnav_sabari_girish/embedded-rust-on-the-esp32-getting-started-setup-28en
// https://dev.to/vaishnav_sabari_girish/embedded-rust-on-the-esp32-blinking-an-led-2jk1

// This is a very early work in progress and currently gives off errors and doesn't build.

fn main() {
    // println!("Hello, world!");
}
