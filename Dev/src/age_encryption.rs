use age::x25519::Recipient;

/// Testing with encryption using Age.
pub fn age_encryption() {
    // TODO Figure out how to use Age for encryption/decryption using Rust.

    // https://docs.rs/age/latest/age/
    // let public_key = Recipient::generate();
    // let plain_text = b"Test";
    // let encrypted = age::encrypt_and_armor(&public_key, plain_text);

    // I got this working, tested decryption on my desktop.
    // Using a random public key for this.
    let encrypt_recipient = Recipient::from("age1zwxszmh5zt55e2p75ckuym6j7cv59nu7dz2gukrjau3k4uwjk47s4nn6na".parse().unwrap());
    // let encrypted_message = age::encrypt_and_armor(&encrypt_recipient, "Test".as_ref()).unwrap();

    // Well this works for these keys, and this is a way to generate Age encryption keys.
    let key = age::x25519::Identity::generate();
    // let pubkey = key.to_public();

    // Recipient.into();

    let plaintext = b"Hello world!";

    let encrypted = age::encrypt_and_armor(&encrypt_recipient, plaintext);
    println!("{:?}", encrypted);
    // let decrypted = age::decrypt(&key, &encrypted.unwrap().parse());
}

/// Testing decryption with Age, not setup yet.
pub fn age_decryption() {

}