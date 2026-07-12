# ESP32 Test

### Basic setup
Power LED on GPIO 26 - This gets turned on when the code is run.

So far, this program is just blinking the blue and green LEDs on GPIO 25 and GPIO 27.

I finally got the SSD1306 display working.

This guide was used
* https://esp32.implrust.com/oled/hello-rust/index.html

And some code from this GitHub repo
* https://github.com/ImplFerris/esp32-projects/tree/main/hello-oled

### Running/Building
It is a good idea to have a button that can reset the ESP32 for easily switching to download/flash mode
without unplugging and plugging in the usb each time.

You can short the `UP` pin to ground, the pin beside of the `EN` pin, and it'll reboot the ESP32.

To build this project, run `cargo build`.

To run this project on an ESP32:
1. Make sure the code builds.
2. Plug up the ESP32 while holding the `BOOT` button via Micro Usb to your PC.
3. Use the command `cargo run` to flash the project to the ESP32.

### Documentation
Here is the documentation for ESP32 and Rust that I am currently using.
* https://esp32.implrust.com

They have a lot of guides and examples to use on that documentation website.

