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
fn key_manager_test_encrypt_decrypt_file_with_passphrase() {
    todo!()

}

#[test]
fn key_manager_test_streaming_symmetric_encrypt_decrypt_file() {
    todo!()
}





