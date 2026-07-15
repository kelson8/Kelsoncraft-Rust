use base64::{Engine as _, engine::general_purpose};

// https://base64.dev/articles/base64-rust
// TODO Add error handling to this.

/// Encode a base64 string
pub fn base64_encode(string_to_encode: &String) -> String {
    // Encode to string
    let encoded = general_purpose::STANDARD.encode(string_to_encode);
    encoded
}

/// Decode a base64 string
pub fn base64_decode(string_to_decode: &String) -> String {
    let decoded = general_purpose::STANDARD.decode(string_to_decode).unwrap();
    let decoded_full = String::from_utf8(decoded).unwrap();

    decoded_full
}

