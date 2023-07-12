use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use chacha20poly1305::AeadInPlace;
use chacha20poly1305::{aead::{Aead, AeadCore, KeyInit, OsRng}, XChaCha20Poly1305};

#[test]
fn test_streaming_symmetric_encrypt_decrypt_file() {
    // mock data file
    let mut data_file = File::create(Path::new("../test_encrypt_file.txt")).unwrap();
    data_file.write("Hello World!".as_bytes()).unwrap();

    // encrypt 
    let key = XChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let supplied_password = "my_master_password";
    println!("Running test streaming symmetric encrypt file");
    let cipher_text = cipher.encrypt(&nonce, supplied_password.as_bytes());
    assert!(cipher_text.is_ok());
    todo!()
    //let plaintext = cipher.decrypt(&nonce, cipher_text);
    //assert!(plaintext.is_ok());
}







