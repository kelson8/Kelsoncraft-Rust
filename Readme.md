# Kelsoncraft-Rust Projects

### Build status

**This build.yml is only for the kcnet-rust project currently.**

<img src="https://git.kelsoncraft.net/kelson8/Kelsoncraft-Rust/badges/workflows/build.yml/badge.svg">

<img src="https://git.kelsoncraft.net/kelson8/Kelsoncraft-Rust/badges/issues/open.svg">

<img src="https://git.kelsoncraft.net/kelson8/Kelsoncraft-Rust/badges/pulls/open.svg">

----

This is a list of random and misc projects that I am working on with rust,
some of these came from the Rust documentation or other packages.

The Dev folder contains some of my older dev testing, I have since switched to the `kcnet-rust` Rust project in the `New` folder.

I have some ESP32 testing in the esp32_test folder which I have now gotten to work by blinking an LED.

I am quite new to rust so a lot of this stuff is based off of the examples
or some crates that I find to use.

**KCNet-Rust**

The [KCNet-Rust](https://git.kelsoncraft.net/kelson8/Kelsoncraft-Rust/src/branch/master/New/kcnet-rust) project is my new main testing project, I will probably rename it to kcnet_rust or something else later.
Also, I have a library in this repository named [kcnet_lib](https://git.kelsoncraft.net/kelson8/Kelsoncraft-Rust/src/branch/master/New/kcnet_lib) which that project depends on.

You can encrypt/decrypt your .env file in the `KCNet-Rust` project by using the `encrypt-env.sh` and `decrypt-env.sh` files.

These will require the following set in your `.bashrc` or `.zshrc`.
You will need to put in your Age public key, and a path to the Age private key file.
This is only ever used for the encrypt and decrypt .env scripts.

* export AGE_PUBLIC_KEY=
* export AGE_KEY_FILE=
* export SOPS_AGE_KEY_FILE=$AGE_KEY_FILE


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

| Project Name      | Description                                                                                                                                                         |
|-------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| age-test          | Testing with encryption and decryption with Age.                                                                                                                    |
| call-c-test       | Testing with calling C code in Rust, and calling Rust in C.                                                                                                         |
| cli-test          | Misc cli testing with command argument libraries.                                                                                                                   |
| ftlk-test         | A GUI test to play around with.                                                                                                                                     |
| ftp-test          | A very basic FTP test with no login and no SSL, using [libunftp](https://github.com/bolcom/libunftp) and [unftp-sbe-fs](https://github.com/rmokerone/unftp-sbe-fs). |
| gtk-test          | Testing with GTK4, so far this doesn't work                                                                                                                         |
| imgui-test        | Testing with ImGui and Rust.                                                                                                                                        |
| kcnet_lib         | Test library that has JSON reading/writing, Vector2D and more for future use.                                                                                       |
| kcnet-slint-test  | Test with the [Slint UI](https://github.com/slint-ui/slint/) for Rust.                                                                                              |
| kcnet-rust        | Main test that is now using the `kcnet_lib` crate within this repo.                                                                                                 |
| raknet-client     | A very basic [Raknet client](https://github.com/b23r0/rust-raknet) copied from the examples.                                                                        |
| raknet-server     | A very basic [Raknet server](https://github.com/b23r0/rust-raknet) copied from the examples.                                                                        |
| tcp-server-test   | This is a very basic TCP Server that I will play around with a bit.                                                                                                 |
| web-requests-test | Testing with web requests and JSON data.                                                                                                                            |


# License
This list of projects are licensed under the MIT license.