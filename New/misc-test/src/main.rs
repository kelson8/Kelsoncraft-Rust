// https://doc.rust-lang.org/rust-by-example/hello/print.html

// extern crate kcnet_lib

// Now this almost works in here.
// use kcnet_lib::{*};
use kcnet_lib::{json_handler, log_handler::logger, json_handler::Vec2D, json_handler::Vec3D};
use kcnet_lib::json_handler::{MapLocation, Player, PlayerState};

use vector2d::Vector2D;
use rand::RngExt;

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

use log::{debug, info};
// use log4rs;

//

use std::env;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use rand::rngs::ThreadRng;
use serde_json::json;
// https://docs.rs/vector2d/latest/vector2d/

// For logging to file
//

// For AES encryption.

// use aes_gcm::aead::KeyInit;

// use app_error::AppError;

// use argon2::{Argon2, PasswordHasher, password_hash::SaltString};

// List of enviornment variables
// https://doc.rust-lang.org/cargo/reference/environment-variables.html
const PROGRAM_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// The path to the config for Log4rs
const LOG_CONFIG: &'static str = "logging_config.yaml";


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

// List of hashing utilities for Rust
// https://github.com/RustCrypto/hashes
//

// Try to look into Clap for argument handling.
// https://github.com/clap-rs/clap

// Supressing the unused function warnings
// https://stackoverflow.com/questions/32900809/how-to-suppress-function-is-never-used-warning-for-a-function-used-by-tests

/// This generates a random Vector2D position, mostly just for something random.
///
/// TODO How can I make this like a template in C++? So It can return a Vector2D and Vector3D.
///
/// `min_range` The minimum range for the coordinates.
///
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
#[allow(dead_code)]
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

#[allow(dead_code)]
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
///
/// TODO Make this get the program name from the cargo crate.
///
/// TODO Make this log to a file later, for now it just logs to the console.
pub fn log_text(text: &str) {
    info!("[Misc-Test]: {}", text);

    // https://docs.rs/simple-logging/latest/simple_logging/
    // TODO Figure this out for file logging.
    // simple_logging::log_to_file("test.log", LevelFilter::Info).expect("Error logging to file");
}

// https://docs.rust-embedded.org/book/c-tips/index.html
// For feature testing
#[cfg(feature="debug_test")]
fn debug_function() {
    // debug!("");
    // logger::write(logger::LogStatus::INFO, "Test message");
    info!("Test message");
}

/// Test with Vectors
///
/// <https://doc.rust-lang.org/book/ch08-01-vectors.html>
#[allow(dead_code)]
fn vector_test() {
    // let vector: Vec<i32> = Vec::new();
    let mut vector = vec![300.0, 180.0, 14.5];

    vector.push(200.0);
}

//---------------

/// Basic player test for the Player struct in json_handler.rs.
#[allow(dead_code)]
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

/// Env testing for environment variables.
///
/// This will be used in the future.
#[allow(dead_code)]
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

/// Testing actions with a JSON file, mostly reading/ writing to a ReVC locations JSON file. currently.
/// Moved out of the main function.
fn json_test(locations_json_file: &str) {

    // This works in here now!
    // Setup a test location for the JSON file testing.
    let location1 = MapLocation {
        name: "Location1".to_string(),
        // pos_x: 180.0,
        // pos_y: 180.0,
        // pos_z: 14.0,
        pos: Vec3D::new(180.0, 180.0, 14.0),
        heading: 90.0,
    };

    // Parse the location JSON data directly here.
    // So it is adapted to my format.
    // TODO Make this get a list of locations from the CLI or something to write to this with.
    let location_json = json!({
        // TODO Make it to where I can use the name here also.
        // For some reason it just complains about the variable being moved.
        // location1.name:  {
        "Location1":  {
            "name": location1.name,
            "pos_x": location1.pos.x,
            "pos_y": location1.pos.y,
            "pos_z": location1.pos.z
        }
    });

    // TODO Add error handling to this.

    // Reading and printing the JSON file
    // json_handler::read_json_file(locations_json_file);
    // json_handler::print_location(locations_json_file, "Location4").expect("Error reading JSON");

    // Write the list of locations to the JSON.

    // This should not be used anymore, use write_location instead.
    // json_handler::write_locations(locations_json_file);


    // This will write a list of locations that I eventually add with the CLI later on.
    json_handler::write_location(locations_json_file,
                                 // location1);
                                 location_json);

    // Print a test location from JSON.
    json_handler::print_location(locations_json_file, "Location1").expect("Error reading JSON");
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

    // Display the program info message on startup.
    println!("--------------------");
    println!("Running {} v{}", PROGRAM_NAME, VERSION);
    println!("--------------------\n");

    // The locations file to output and read the list of ReVC game locations from.
    let locations_json_file = "test.json";

    // Setup the new Log4rs logger
    // Setup the log file with the library.
    logger::init(LOG_CONFIG);

    // Write to the log file with the library.
    // logger::test();
    //

    // Load the .env file for later use
    // There is a .env.example file in here which can be renamed to .env and used.
    // https://env.dev/guides/rust-env-variables
    dotenvy::dotenv().ok();

    // info!(".env loaded");

    // let number_to_convert = 400;
    // let mut rng = rand::rng();

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


    //-------
    // JSON Testing
    //-------
    json_test(locations_json_file);


    // #[cfg(feature="debug_test")]
    // debug_function()

    //------
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

    // AES256 testing
    // encryption_handler::encryption_test().unwrap();

    //------
    // Argon2 password hash testing
    // Some dummy values for testing.
    // let plain_password = "test123";
    // let plain_password_hash = "$argon2id$v=19$m=19456,t=2,p=1$QQ5Bz3c5ju11cDDUbTeqYw$1fUHsyc9LrZStzJbULE+cskVUkF6poAcr9Gj/K7ClwQ";
    //
    // // let hashed_password = hash_util::argon2_hash(plain_password).unwrap();
    // // println!("Hashed password: {}", hashed_password);
    //
    // let verified_password = hash_util::argon2_hash_verify(plain_password, plain_password_hash).unwrap();
    // if verified_password {
    //     println!("Password matches! Logging in.");
    // } else {
    //     println!("Password does not match! Cannot login.");
    // }

    //--------

}
