#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// https://codezup.com/rust-interoperability-c-cpp-2/

// Well this works, I might be able to make a C API for ReVC that can be used with Rust code in the future.
// I could easily adapt functions from my C++ Lua scripts and just make a few for testing in Rust.

//
// unsafe extern "C" {
//     fn add(a: i32, b: i32) -> i32;
// }
//
// fn main() {
//     let sum = unsafe { add(2, 3) };
//     println!("add(2, 3) = {sum}");
// }

// This method is even better and easier to use.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Taken from kcnet-rust

// unsafe extern "C" {
//     fn abs(input: i32) -> i32;
// }
//
// // Call a C function in Rust.
// fn call_c_test()
// {
//     unsafe {
//         println!("Absolute value of -3 according to C: {}", abs(-3));
//     }
// }
//
// // I can also write functions that work in C
// #[unsafe(no_mangle)]
// pub extern "C" fn call_from_c() {
//     println!("Just called a Rust function from C!");
// }

//

fn main() {
    let sum = unsafe { add(4, 5) };
    println!("add(4, 5) = {sum}");
}