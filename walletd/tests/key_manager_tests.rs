use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::prelude::*;
use std::fs;
use std::str;

use rand::{thread_rng, Rng};
use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Key};
use chacha20poly1305::aead::Aead;
use generic_array::GenericArray;

#[test]
fn key_manager_test_unlocked_file_create_read() {
    println!("Running test unlocked file create read");
    let file_path = "./tests/generated_or_modified_files/test_create_file.txt";
    let mut data_file = File::create(Path::new(file_path)).unwrap();
    assert!(data_file.write("Hello World!".as_bytes()).is_ok());
    let contents = fs::read_to_string(file_path).unwrap();
    println!("File has contents:\n{contents}");
}

#[test]
fn key_manager_check_generic_array() {
    println!("Running test key_manager check generic array");
    let bytes: [u8; 4] = [1, 2, 3, 4];
    println!("bytes: {:?}", bytes);
    let generic_array = GenericArray::from(bytes);
    println!("generic_array: {:?}", generic_array);
}

#[test]
fn key_manager_test_encrypt_decrypt_string_with_passphrase() {
    println!("Running test encrypt/decrypt string with passphrase");
    let passphrase = "my_master_passphrase";
    let passphrase_bytes = passphrase.as_bytes();
    println!("passphrase: {} as bytes {:?}", passphrase, passphrase_bytes);
    let key = GenericArray::from_slice(passphrase.as_bytes());
    println!("passphrase as key: {:?}", key);
    let cipher = ChaCha20Poly1305::new(&key);
    let mut rand_for_nonce: [u8; 12] = thread_rng().gen();
    println!("rand_for_nonce: {:?}", rand_for_nonce);
    // sorting the nonce so it's easier to compare if same or different (need to use the same nonce to decrypt the message, should avoid reuse of same nonce for different messages)
    rand_for_nonce.sort();
    // let nonce = GenericArray::from_slice(rand_for_nonce.as_ref());
    // println!("sorted nonce: {:?}", nonce);
    // let plaintext = "hexadecimal";
    // println!("plaintext message: {}", plaintext);
    // let plaintext_bytes = plaintext.as_bytes();
    // println!("plaintext message as bytes: {:?}", plaintext_bytes);
    // let ciphertext_bytes = cipher.encrypt(&nonce, plaintext_bytes).unwrap();
    // println!("encrypted message (bytes): {:?}", ciphertext_bytes);
    // next is the decryption
}

// #[test]
// fn key_manager_test_streaming_symmetric_encrypt_decrypt_file() {
//     todo!()
// }





