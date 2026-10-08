/// This will be used for encryption later on.
///

// For AES encryption.
// use encryptman::{encrypt, decrypt, generate_master_key, MasterKey};
use encryptman::{encrypt, decrypt, generate_master_key};

// use aes_gcm::{
//     aead::{Aead, AeadCore, KeyInit, OsRng},
//     Aes256Gcm, Key, Nonce,
// };

// use std::error::Error;
// use app_error::AppError;

// use argon2::{Argon2, PasswordHasher};


//-------------
// Taken out of misc-test.
//-------------

/// Generate a random encryption key and return the hex encoded value of it.
pub fn generate_encryption_key() -> String {
    let master_key = generate_master_key().unwrap();
    let hex_key = hex::encode(&master_key.as_bytes());

    hex_key
}

/// Encryption test with encryptman and AES.
/// TODO Make this support custom passwords also.
pub fn encryption_test() -> Result<(), Box<dyn std::error::Error>> {
    let master_key = generate_master_key()?;
    let hex_key = hex::encode(&master_key.as_bytes());

    // TODO Try to figure this part out.
    // Not sure how to convert this hex encoded value back to a MasterKey.
    // let hex_key = generate_encryption_key();
    // let master_key: MasterKey = hex::decode(hex_key)?;

    let encrypted = encrypt(&master_key, "my_database_password")?;
    let decrypted = decrypt(&master_key, &encrypted)?;

    println!("Key (HEX-Encoded): {}", hex_key);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);

    assert_eq!(decrypted, "my_database_password");
    Ok(())
}

//-------------
// Taken from here
// https://rustz2h.com/chapter_13_rust_security_and_cryptography/series_05_project_secure_password_manager/argon2_key_derivation_rust
// Well of course this changes with argon 0.6.0 on Rust...
// https://mojoauth.com/security-guides/argon2-in-rust#frequently-asked-questions
//-------------

//// Derives a 32-byte encryption key from a password using Argon2id.
//// Returns the key and the salt (needed to re-derive the same key later).
// pub fn derive_key(password: &str) -> Result<([u8; 32], [u8; 16]), Box<dyn std::error::Error>> {
//     // Generate a random salt (16 bytes is standard).
//     let salt = SaltString::generate(&mut OsRng);
//
//     // Configure Argon2id with recommended parameters for a personal vault.
//     let params = ParamsString::new()
//         .time_cost(3)      // 3 iterations
//         .mem_cost(19)      // 512 MiB
//         .parallelism(1)    // Single thread
//         .map_err(|e| format!("Invalid params: {}", e))?;
//
//     let argon2 = Argon2::new(
//         argon2::Algorithm::Argon2id,
//         argon2::Version::V0x13,
//         params,
//     );
//
//     // Hash the password.
//     let hash = argon2
//         .hash_password(password.as_bytes(), &salt)
//         .map_err(|e| format!("Failed to hash: {}", e))?
//         .to_string();
//
//     // Extract the 32-byte key from the hash.
//     let hash_bytes = hash.as_bytes();
//     let mut key = [0u8; 32];
//     key.copy_from_slice(&hash_bytes[..32.min(hash_bytes.len())]);
//
//     // Extract salt as 16 bytes.
//     let salt_bytes = salt.as_str().as_bytes();
//     let mut salt_array = [0u8; 16];
//     salt_array.copy_from_slice(&salt_bytes[..16.min(salt_bytes.len())]);
//
//     Ok((key, salt_array))
// }
//
// /// Re-derives the same key given a password and the salt used in the original derivation.
// pub fn verify_and_derive(password: &str, salt_bytes: &[u8; 16]) -> Result<[u8; 32], Box<dyn std::error::Error>> {
//     // Reconstruct the salt string.
//     let salt_str = String::from_utf8(salt_bytes.to_vec())?;
//     // let salt = SaltString::new(&salt_str)
//     //     .map_err(|e| format!("Invalid salt: {}", e))?;
//
//     let salt = &salt_str;
//
//     // Use the same Argon2id parameters.
//     // let params = ParamsString::new()
//     //     .time_cost(3)
//     //     .mem_cost(19)
//     //     .parallelism(1)
//     //     .map_err(|e| format!("Invalid params: {}", e))?;
//     let mut key = [0u8; 32];
//
//     Argon2::default().hash_password_into(password.as_bytes(), salt.as_bytes(), &key).expect("Invalid params");
//
//     let argon2 = Argon2::new(
//         argon2::Algorithm::Argon2id,
//         argon2::Version::V0x13,
//         params,
//     );
//
//     let hash = argon2
//         .hash_password(password.as_bytes(), &salt)
//         .map_err(|e| format!("Failed to hash: {}", e))?
//         .to_string();
//
//     let hash_bytes = hash.as_bytes();
//     let mut key = [0u8; 32];
//     key.copy_from_slice(&hash_bytes[..32.min(hash_bytes.len())]);
//
//     Ok(key)
// }

//-------------
// Taken from here
// https://dev.to/hiyoyok/aes-256-gcm-encryption-in-rust-securing-local-app-data-3gck
//-------------

//
// Aes encryption test
//
// fn aes_encrypt_test(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, AppError> {
//     let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
//
//     // Generate random nonce — never reuse a nonce with the same key
//     let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
//
//     let ciphertext = cipher
//         .encrypt(&nonce, data)
//         .map_err(|_| AppError::Encryption("Encryption failed".into()))?;
//
//     // Prepend nonce to ciphertext — you need it for decryption
//     let mut result = nonce.to_vec();
//     result.extend_from_slice(&ciphertext);
//
//     Ok(result)
// }
//
// /// Test decryption with AES
// // fn aes_decrypt_test(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, AppError> {
// fn aes_decrypt_test(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, bool> {
//     if data.len() < 12 {
//         // return Err(AppError::msg("Data too short".into()));
//         return Err("Data too short".into());
//     }
//
//     let (nonce_bytes, ciphertext) = data.split_at(12);
//     let nonce = Nonce::from_slice(nonce_bytes);
//     let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
//
//     cipher
//         .decrypt(nonce, ciphertext)
//         .expect("Decryption failed - wrong key or corrupted data.");
//         // .map_err(|_| AppError::msg("Decryption failed — wrong key or corrupted data".into()))
// }
//
// /// Derive a AES key from a password
// // pub fn derive_key(password: &str, salt: &[u8; 32]) -> Result<[u8; 32], AppError> {
// pub fn derive_key(password: &str, salt: &[u8; 32]) -> Result<[u8; 32], bool> {
//     let argon2 = Argon2::default();
//     let mut key = [0u8; 32];
//
//     argon2
//         .hash_password_into(password.as_bytes(), salt, &mut key).expect("Key derivation failed");
//         // .map_err(|_| AppError::Encryption("Key derivation failed".into()))?;
//         // .map_err(|_| AppError::msg("Key derivation failed.".into()))?;
//
//     Ok(key)
// }


