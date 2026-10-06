// https://doc.rust-lang.org/rust-by-example/crates/lib.html

// TODO Figure out how to use this for modules later.

// mod test_library {

pub mod json_handler;

use rand::RngExt;

/// Generate a random number
pub fn generate_random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();

    rng.random_range(min..max)
}

// pub fn public_function() {
//     println!("called test `public_function()`");
// }

// fn private_function() {
//     println!("called test `private_function()`");
// }
//
// pub fn indirect_access() {
//     print!("called test `indirect_access()`> ");
//
//     private_function();
// }

// }