extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};

// https://github.com/Keats/rust-bcrypt

// Hash a password with Bcrypt
pub fn hash_password(password: &String) -> String {
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    hashed_password
    // let hashed = hash("hunter2", DEFAULT_COST)?;
    // let valid = verify("hunter2", &hashed)?;
}

/// Verify that a hashed password is valid
pub fn verify_password(password: &String, hashed_password: &String) -> bool {
    // https://doc.rust-lang.org/std/keyword.return.html
    // I didn't know return wasn't needed in cases like this when it is the last expression in the function.
    verify(password, hashed_password).unwrap()
}

