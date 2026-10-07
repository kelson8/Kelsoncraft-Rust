
// https://doc.rust-lang.org/rust-by-example/hello/print.html

// extern crate kcnet_lib

// Now this almost works in here.
// use kcnet_lib::{*};
use kcnet_lib::{json_handler, log_handler::logger, number_generators};
use kcnet_lib::json_handler::{Player, PlayerState};

use vector2d::Vector2D;
use rand::{rng, RngExt};

// https://docs.rs/base64/latest/base64/
// use base64::prelude::*;

// https://docs.rs/sha256/latest/sha256/
// use sha256::{digest, try_digest};

// https://docs.rs/argon2/latest/argon2/
// use argon2::{
    // password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
    // Argon2
// };

// Logging
// https://github.com/rust-lang/log
// use log::{info, trace, warn};
// use log::{info};

use log::{debug, error, info, trace, warn};
// use log4rs;

//

use std::{env, fs};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use rand::rngs::ThreadRng;
// https://docs.rs/vector2d/latest/vector2d/

// For logging to file
use log::LevelFilter;
//

// Guide on the question mark operator
// https://tutorials.dodatech.com/rust-systems/question-mark-operator/

// Rust security hand book
// https://github.com/yevh/rust-security-handbook

// Someone made a RakNet server in Rust
// https://github.com/b23r0/rust-raknet

// JSON in rust
// https://github.com/cloudwego/sonic-rs

// This JSON rust library might be easier to use
// https://crates.io/crates/serde_json




/// This generates a random Vector2D position, mostly just for something random.
/// TODO How can I make this like a template in C++? So It can return a Vector2D and Vector3D.
/// `min_range` The minimum range for the coordinates.
/// `max_range` The maximum range for the coordinates.
fn generate_random_position(min_range: f32, max_range: f32) -> Vector2D<f32> {
    // println!("Generating random position");
    // https://docs.rs/rand/latest/rand/

    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
    let mut rng = rand::rng();
    // println!("Integer: {}", rng.random_range(min_range..max_range));

    let random_pos1 = rng.random_range(min_range..max_range);
    let random_pos2 = rng.random_range(min_range..max_range);

    // println!("Float: {}", rng.random_range(min_range..max_range));
    // println!("Random number: {random_number}");

    Vector2D::new(random_pos1, random_pos2)
}

/// Hash a string with SHA256.
// fn sha256_hash_string(string_to_hash: String) -> String {
//     let input = String::from(&string_to_hash);
//     let val = digest(input);
//     val
// }

// Well I need to learn more error handling in Rust before I move on with it.

/**
// fn hash_password_argon2(password: &str) {
fn hash_password_argon2(password: &[u8]) {
    let argon2 = Argon2::default();

    // Hash the password to PHC string ($argon2id$v=19$...)
    // let password_hash = argon2.hash_password(password)?.to_string();
    let password_hash = argon2.hash_password(password)?.to_string();
}

fn verify_password_argon2(password: &u8, password_hash: &str) -> bool {
    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    // let parsed_hash = PasswordHash::new(&password_hash)?;
    // assert!(Argon2::default().verify_password(password, &parsed_hash).is_ok());
}
*/

// Some Platform specfic test code
// https://www.rustfaq.org/en/how-to-use-cfg-attributes-for-platform-specific-code/
// https://elitedev.in/rust/8_essential_rust_techniques_for_seamless_cross-platform_development_from_conditional_compilation_to_multi-target_testing/

// #[cfg(target_os = "windows")]
// fn get_config_path() -> String {
//     "C:\\ProgramData\\rust-test\\config.toml".to_string()
// }
//
// #[cfg(target_os = "linux")]
// fn get_config_path() -> String {
//     "/etc/rust-test/config.toml".to_string()
// }
//
// #[cfg(target_os = "macos")]
// fn get_config_path() -> String {
//     "/Library/Application Support/rust-test/config.toml".to_string()
// }
// //

//------------
/// Async test
struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            // println!("Hello world");
            println!("Completed task");
            // println!("Completed task in {}", self.when.elapsed().as_secs());
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

//--------------------

async fn test_async(mut rng: ThreadRng) {
    let random_wait_time = rng.random_range(3000..10000);

    // Async test
    // This mostly just waits for a few seconds.
    let when = Instant::now() + Duration::from_millis(random_wait_time);
    let future = Delay { when };


    // Async is working! I can run this while the other function is running.
    // Although if I put this under future.await or under assert it won't work.
    println!("The current folder is: {}", env::current_dir().unwrap().display());
    // println!("Waiting for {} seconds", when.elapsed().as_secs());
    println!("Waiting for {} milliseconds", random_wait_time);
    //

    let out = future.await;
    // println!("Completed task");
    assert_eq!(out, "done");


}

/// Log some text to a file with my custom log format.
/// TODO Make this get the program name from the cargo crate.
/// TODO Make this log to a file later, for now it just logs to the console.
pub fn log_text(text: &str) {
    info!("[Misc-Test]: {}", text);

    // https://docs.rs/simple-logging/latest/simple_logging/
    // TODO Figure this out for file logging.
    // simple_logging::log_to_file("test.log", LevelFilter::Info).expect("Error logging to file");



}

//---------------

/// Basic player test for the Player struct in json_handler.rs.
fn player_test() {
    let mut rng = rand::rng();
    // Random Vector2D
    let random_position = generate_random_position(0.0, 200.0);
    // println!("Random position: {:?}", random_position);

    let random_health = rng.random_range(1..100);
    let random_armor = rng.random_range(1..100);
    let random_hunger = rng.random_range(1..100);
    let random_heading = rng.random_range(0.0..180.0);

    let player = Player {
        name: "Admin".parse().unwrap(),
        health: random_health,
        armor: random_armor,
        hunger: random_hunger,

        status: PlayerState::MainMenu,
        position: random_position,
        heading: random_heading
    };

    // Print the test values for the player.
    println!("Player status:\n Name: {}\n Health: {}\n\
                Armor: {}\n Hunger: {}\n Status: {:?}\n\
                Position: {:?}\n Heading: {}",
             player.name, player.health, player.armor, player.hunger,
            player.status, player.position, player.heading);

}

/// Env testing for environment variables
/// This will be used in the future.
fn env_test() {
    // This works, gives an error if the password isn't set.
    // This just crashes the program here.
    // let test_password = env::var("TEST_PASSWORD").expect("Test password must be set");
    // Switched to this, it gives a blank value instead of just crashing.
    // Although I may switch back to the other option later on if i require this.
    let test_password = env::var("TEST_PASSWORD").unwrap_or_default();

    if test_password.is_empty()  {
        info!("Test password is blank.");
    } else {
        info!("Test password: {}", test_password);
    }
}

// I didn't know I could use C in Rust.
// https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#using-extern-functions-to-call-external-code
// Moved C testing into call-c-test.
//

// Async support from Tokio
// https://tokio.rs/tokio/tutorial/async
#[tokio::main]
async fn main() {
    // Setup the logger
    // env_logger::init();

    // The locations file to output and read the list of ReVC game locations from.
    let locations_json_file = "test.json";

    // Setup the new Log4rs logger
    // log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    // Setup the log file with the library.
    logger::init("logging_config.yaml");

    // These will log to the file and the console.
    // TODO Move this into log_handler in the library later.
    // trace!("detailed tracing info");
    // debug!("debug info");
    // info!("relevant general info");
    // warn!("warning this program doesn't do much");
    // error!("error message here");

    // Write to the log file with the library.
    // logger::test();
    //


    // Load the .env file for later use
    // There is a .env.example file in here which can be renamed to .env and used.
    // https://env.dev/guides/rust-env-variables
    dotenvy::dotenv().ok();

    // let number_to_convert = 400;
    let mut rng = rand::rng();

    // Display the working directory
    // https://doc.rust-lang.org/std/env/fn.current_dir.html
    // let path = env::current_dir();
    // println!("The current folder is: {}", path.unwrap().display());

    // Convert a number to hex

    // println!("'{}' to Base 16 (hexadecimal): '0x{:x}'", number_to_convert, number_to_convert);


    // Print a test location from JSON.
    // print_location("test.json").unwrap();

    // Library testing from KCNet-Rust-Lib
    // let random_number = number_generators::generate_random_number(1, 1000);
    // println!("Random number: {}", random_number);


    // json_handler::read_json_file(locations_json_file);

    // json_handler::print_location(locations_json_file, "Location4").expect("Error reading JSON");

    // Write the list of locations to the JSON.
    json_handler::write_locations(locations_json_file);

    // Print a test location from JSON.
    json_handler::print_location(locations_json_file, "Location1").expect("Error reading JSON");

    // Call a C function in Rust.
    // call_c_test();

    // Logging text, currently just outputs a log message to console.
    // log_text("File: test.txt");

    // Env testing
    // env_test();


    // Plaer struct testing
    // player_test()

    // Async testing
    // test_async(rng).await;

    // SHA256 and other hashing
    // let test_hashed = sha256_hash_string("test".to_string());
    // println!("Test hashed: {}", test_hashed);

}
