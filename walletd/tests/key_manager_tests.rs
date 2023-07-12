use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use chacha20poly1305::{aead::{Aead, AeadCore, KeyInit, OsRng}, XChaCha20Poly1305, XNonce};

#[test]
fn test_streaming_symmetric_encrypt_file() {
    let mut write_file = File::create(Path::new("../test_encrypt_file.txt")).unwrap();
    write_file.write("Hello World!".as_bytes()).unwrap();

    // encrypt 

    let key = XChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let supplied_password = "my_master_password";
    let ciphertext = cipher.encrypt(&nonce, supplied_password.as_bytes()).unwrap();
}







