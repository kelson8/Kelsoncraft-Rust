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
