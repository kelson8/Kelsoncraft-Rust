
// use std::error::Error;
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash, phc::Error},
    Argon2
};

/// Hash a password with Argon2
///
/// https://docs.rs/argon2/latest/argon2/
///
/// Guide used for learning a bit more about the Question mark operator.
///
/// https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about
pub fn argon2_hash(password: &str) -> Result<PasswordHash, Error> {

    // let password = password;

    // Argon2 with default params (Argon2id v19), generating a random salt
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password.as_ref()).unwrap().to_string();

    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(&password_hash)?;
    assert!(Argon2::default().verify_password(password.as_ref(), &parsed_hash).is_ok());

    Ok(parsed_hash)

}

/// This should work for verifying a Argon2 hash in Rust.
///
/// Adapted this from the guide below
///
/// https://mojoauth.com/security-guides/argon2-in-rust#how-to-hash-and-verify-a-password-with-argon2-in-rust
///
/// Example usage:
///
/// ```rust
/// // WARNING Don't re-use this password or the hash, generate one to use with the argon2_hash function.
/// // The plain text password should never be stored!
///
/// // Some dummy values for testing.
/// let plain_password = "test123";
/// let plain_password_hash = "$argon2id$v=19$m=19456,t=2,p=1$QQ5Bz3c5ju11cDDUbTeqYw$1fUHsyc9LrZStzJbULE+cskVUkF6poAcr9Gj/K7ClwQ";
///
/// let verified_password = hash_util::argon2_hash_verify(plain_password, plain_password_hash).unwrap();
/// if verified_password {
///     println!("Password matches! Logging in.");
/// } else {
///     println!("Password does not match! Cannot login.");
/// }
/// ```
///
pub fn argon2_hash_verify(password: &str, hashed_password: &str) -> Result<bool, Error> {
    let argon2 = Argon2::default();
    // let password_hash = argon2.hash_password(hashed_password.as_bytes()).unwrap();
    // let verify_password = argon2.verify_password(password.as_ref(), &hashed_password).unwrap();
    let verify_password = argon2.verify_password(password.as_ref(), hashed_password).is_ok();

    Ok(verify_password)
}