// use std::io;
// This is going to be me testing and experimenting with Rust.
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use crate::messages::WELCOME_MESSAGE;
use crate::system_info::get_system_info;

// https://rust-lang.github.io/book/ch07-05-separating-modules-into-different-files.html
pub mod number_generator_test;
mod number_guesser;
pub mod system_info;
pub mod messages;

fn main() {

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

    // Print the OS info.
    get_system_info();

    // let random_number = number_generator_test::generate_random_number();
    // println!("Random number: {random_number}");


}
