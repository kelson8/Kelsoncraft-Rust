//! # KCNet Lib
//!
//! `kcnet_lib` contains some useful basic utilities.
//!
//! **Utility functions and libraries**
//!
//! * Hashing passwords with [Argon2](https://docs.rs/argon2/latest/argon2/).
//! * Reading/writing to JSON files with [Serde](https://docs.rs/serde/latest/serde/).
//! * Log handling with [Log4rs](https://docs.rs/log4rs/latest/log4rs/).
//! * Random number generators.
//!
//! ## Library Info
//!
//! This is a very early alpha version and I may change everything or move functions around.
//! Nothing is final or complete just yet in this library.
//!
//! # License
//! This is licensed under MIT, the same as the other projects in this repo.
//!

// https://doc.rust-lang.org/rust-by-example/crates/lib.html

pub mod json_handler;
pub mod log_handler;
pub mod number_generators;
pub mod encryption_handler;
pub mod hash_util;
// All functions are now in the files that should state what they do.
// json_handler.rs - A basic JSON system for my ReVC location system that I will use in the future.
// encryption_handler.rs - This will contain AES 256 encryption along with deriving a key from a password and other encryption util.
// log_handler.rs - A log handler for Log4rs, I will setup more in this later.
// number_generators.rs - Set of random number generators for my program uses.
