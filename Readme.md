# Kelsoncraft-Rust Projects

This is a list of random and misc projects that I am working on with rust,
some of these came from the Rust documentation or other packages.

The Dev folder contains some of my dev testing.

I have some ESP32 testing in the esp32_test folder which I have now gotten to work by blinking an LED.

I am quite new to rust so a lot of this stuff is based off of the examples
or some crates that I find to use.

**Guide used for ESP32 setup**

I used this guide for setting up the ESP32 testing project
* https://developer.mamezou-tech.com/en/blogs/2025/05/19/using-rust-02/

Here is a guide for using the SSD1306 screen that I have for the ESP32
* https://esp32.implrust.com/oled/hello-rust/index.html

## Projects

**Dev folder**

This contains some of my original testing programs that I was messing around with and will
most likely be reused in my new code.

**esp32_test Folder**

Testing with a ESP32 with some buttons and LEDS, also making text display on a SSD1306 OLED display.
I may add more to this later.



**New Folder**

| Project Name      | Description                                                                                  |
|-------------------|----------------------------------------------------------------------------------------------|
| age-test          | Testing with encryption and decryption with Age.                                             |
| call-c-test       | Testing with calling C code in Rust, and calling Rust in C.                                  |
| cli-test          | Misc cli testing with command argument libraries.                                            |
| ftlk-test         | A GUI test to play around with.                                                              |
| gtk-test          | Testing with GTK4, so far this doesn't work                                                  |
| misc-test         | Main test that is now using the `test_library` crate within this repo.                       |
| test_library      | Test library that has JSON reading/writing, Vector2D and more for future use.                |
| raknet-client     | A very basic [Raknet client](https://github.com/b23r0/rust-raknet) copied from the examples. |
| raknet-server     | A very basic [Raknet server](https://github.com/b23r0/rust-raknet) copied from the examples. |
| web-requests-test | Testing with web requests and JSON data.                                                     |

# License
This list of projects are licensed under the MIT license.