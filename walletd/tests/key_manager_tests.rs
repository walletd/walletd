use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::prelude::*;
use std::fs;
use rand::{thread_rng, Rng};
use chacha20poly1305::{ChaCha20Poly1305, KeyInit};

#[test]
fn test_unlocked_file_create_read() {
    println!("Running test unlocked file create read");
    let file_path = "./tests/generated_or_modified_files/test_create_file.txt";
    let mut data_file = File::create(Path::new(file_path)).unwrap();
    assert!(data_file.write("Hello World!".as_bytes()).is_ok());
    let contents = fs::read_to_string(file_path).unwrap();
    println!("File has contents:\n{contents}");
}

#[test]
fn test_streaming_symmetric_encrypt_decrypt_file() {
    println!("Streaming symmetric encrypt/decrypt file");
    // wrap a file with encryption
    let key: [u8; 16] = thread_rng().gen();
    let file_path = "./tests/generated_or_modified_files/test_encrypt_file.txt";
    let unencrypted_message = "Hello World, is this encrypted?";
    let mut data_file = File::create(Path::new(file_path)).unwrap();
    // assert!(writer.write_all(unencrypted_message.as_bytes()).is_ok());
    println!("unencrypted message: {}", unencrypted_message);

    let key: [u8; 16] = thread_rng().gen();
    let mut read_file = File::open(Path::new(file_path)).unwrap();
    let mut buffer = Vec::new();
    read_file.read_to_end(&mut buffer).unwrap();
}

fn test_encrypt_with_passphrase() {
    let passphrase = "my_master_passphrase";
    let key = passphrase.as_bytes();
    let cipher = ChaCha20Poly1305::new(&key);
    let rand_for_nonce: [u8; 12] = thread_rng().gen();
    let nonce = rand_for_nonce.to_vec().sort();
    let aad = [1, 2, 3, 4];
    let plaintext = b"{master_seed: 'hexadecimal'}";
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    let tag = chacha20poly1305::encrypt(&key, &nonce, &plaintext, &mut ciphertext).unwrap();

}


