
// https://doc.rust-lang.org/rust-by-example/hello/print.html

use vector2d::Vector2D;
use rand::RngExt;

// https://docs.rs/base64/latest/base64/
use base64::prelude::*;

// https://docs.rs/sha256/latest/sha256/
use sha256::{digest, try_digest};

// https://docs.rs/argon2/latest/argon2/
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
    Argon2
};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{env, fs};

// https://docs.rs/vector2d/latest/vector2d/

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

// The state for the basic player, where are they currently at.
#[derive(Debug)]
enum PlayerState {
    MainMenu,
    Playing,
    Paused,
    Dead,
}

#[derive(Serialize, Deserialize)]
struct MapLocation {
    // position: Vector2D<f32>,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    heading: f32
}

// Print a test location with JSON.
// Now this prints from a JSON file, I need to figure out how to parse it.
// And write back into this.
// I may build a location reader/writer in Rust for my ReVC Lua scripts.
fn print_location(json_path: &str) -> Result<()> {
    // let location = MapLocation {
    //     pos_x: 200.0,
    //     pos_y: 200.0,
    //     pos_z: 200.0,
    //     heading: 180.0
    // };

    // https://stackoverflow.com/questions/63657897/how-to-read-json-file-with-serde
    let data = fs::read_to_string(json_path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse json");

    // Serialize it to a JSON string
    let j = serde_json::to_string_pretty(&res)?;
    // let j = serde_json::to_string(&location)?;

    println!("{}", j);

    Ok(())
}


// Very basic player for testing structs
struct Player {
    name: String,
    // position: Vector2D::new(0, 0.0);
    health: i32,
    armor: i32,
    hunger: i32,
    status: PlayerState,
    // ipv4_address: String,
    // ipv6_address: String,
    position: Vector2D<f32>,
    // camera_position: Vector2D<f32>,
    heading: f32
}

/// This generates a random Vector2D position, mostly just for something random.
/// TODO How can I make this like a template in C++? So It can return a Vector2D and Vector3D.
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

    return Vector2D::new(random_pos1, random_pos2);
}

/// Hash a string with SHA256.
fn sha256_hash_string(string_to_hash: String) -> String {
    let input = String::from(&string_to_hash);
    let val = digest(input);
    val
}

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

#[cfg(target_os = "windows")]
fn get_config_path() -> String {
    "C:\\ProgramData\\rust-test\\config.toml".to_string()
}

#[cfg(target_os = "linux")]
fn get_config_path() -> String {
    "/etc/rust-test/config.toml".to_string()
}

#[cfg(target_os = "macos")]
fn get_config_path() -> String {
    "/Library/Application Support/rust-test/config.toml".to_string()
}
//


fn main() {
    // println!("Hello, world!");

    let number_to_convert = 400;
    let mut rng = rand::rng();

    // Display the working directory
    // https://doc.rust-lang.org/std/env/fn.current_dir.html
    let path = env::current_dir();
    println!("The current folder is: {}", path.unwrap().display());

    // Convert a number to hex

    // println!("'{}' to Base 16 (hexadecimal): '0x{:x}'", number_to_convert, number_to_convert);

    // Random Vector2D
    let random_position = generate_random_position(0.0, 200.0);
    // println!("Random position: {:?}", random_position);

    let random_health = rng.random_range(1..100);
    let random_armor = rng.random_range(1..100);
    let random_hunger = rng.random_range(1..100);
    let random_heading = rng.random_range(0.0..180.0);

    // let player = Player;
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
    // println!("Player status:\n Name: {}\n Health: {}\n\
    //             Armor: {}\n Hunger: {}\n Status: {:?}\n\
    //             Position: {:?}\n Heading: {}",
    //          player.name, player.health, player.armor, player.hunger,
    //         player.status, player.position, player.heading);

    // Print a test location from JSON.
    print_location("test.json").unwrap();





    // SHA256 and other hashing
    // let test_hashed = sha256_hash_string("test".to_string());
    // println!("Test hashed: {}", test_hashed);

}
