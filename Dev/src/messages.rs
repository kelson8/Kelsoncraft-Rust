use const_format::formatcp;

// use dotenv::dotenv;
// use std::env;

const PROGRAM_VERSION: &str = "0.0.1a";

// I cannot define a let variable outside of a function, so maybe I can just return this.
// static program_version = env::var("");

// TODO Figure out how to do this, this should get the env variable
// https://dev.to/francescoxx/3-ways-to-use-environment-variables-in-rust-4eaf
// fn get_program_version() -> Result<String, std::env::VarError> {
//     let program_version = env::var("PROGRAM_VERSION");
//
//     match program_version {
//         Ok(version) => println!("{}", version),
//         Err(e) => println!("Error, program version not found! {}", e),
//     }
//
//     return program_version;
// }

// TODO Figure out how to do this, I didn't know I could use env variables here.
// I think this is only for passing the env variable from the command line.
// const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");
// pub const WELCOME_MESSAGE: &str = "KelsonCraft-Rust v0.0.1a starting";
// pub const WELCOME_MESSAGE: &str = "KelsonCraft-Rust v{} starting";

// Switched to using const_format Crate with Rust
// https://docs.rs/const_format/latest/const_format/
pub const WELCOME_MESSAGE: &str = formatcp!("KelsonCraft-Rust v{PROGRAM_VERSION} starting");


// TODO Figure out how to do this, I cannot use the format specifier with const.
// let PROGRAM_VERSION = env::var("PROGRAM_VERSION");

