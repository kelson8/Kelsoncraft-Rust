// use std::io;
// This is going to be me testing and experimenting with Rust.
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use crate::messages::WELCOME_MESSAGE;
use crate::system_info::{get_current_os, get_system_info};

use dotenv::dotenv;

// https://rust-lang.github.io/book/ch07-05-separating-modules-into-different-files.html
pub mod number_generator_test;
mod number_guesser;
pub mod system_info;
pub mod messages;

// I would like to look into the Iced Gui for Rust in the future.
// https://book.iced.rs/first-steps.html

// TODO Look into conditional compilation like in C/C++
// https://stackoverflow.com/questions/27632660/how-do-i-use-conditional-compilation-with-cfg-and-cargo

fn main() {

    // Read the .env file
    dotenv().ok();

    // Print the welcome message
    println!("=====================");
    println!("{}", WELCOME_MESSAGE);
    println!("=====================\n");

    // Run the number guesser.
    // number_guesser::number_guesser();

    // I may have figured out the impl syntax.
    // Well nevermind this just calls the struct...
    // number_generator_test::NumberGeneratorTest
    // {
    //     random_number: 1
    // };

    // https://doc.rust-lang.org/book/ch03-02-data-types.html
    // const PROGRAM_VERSION_NUM: f64 = 0.1;
    // I can use hex numbers directly in Rust like this? I wonder if this can be done in
    //  C/C++ or other languages, I think it can.
    // Generated hex numbers with the website below.
    // https://www.browserling.com/tools/random-hex
    // let number_test: i64 = 0x22c250d7;
    // println!("{}", number_test);


    // Print the OS info.
    get_system_info();

    // Print the current OS, such as Windows 11 Home or Pro.
    // get_current_os();

    // let random_number = number_generator_test::generate_random_number();
    // println!("Random number: {random_number}");


}
