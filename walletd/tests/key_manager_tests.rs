use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs;
use crypto::aessafe::{AesSafe128Encryptor, AesSafe128Decryptor};
use rand::Rng;
use rand::rngs::OsRng;
use aesstream;



#[test]
fn test_unlocked_file_create_read() {
    println!("Running test unlocked file create read");
    let file_path = "./tests/generated_or_modified_files/test_encrypt_file.txt";
    let mut data_file = File::create(Path::new(file_path)).unwrap();
    assert!(data_file.write("Hello World!".as_bytes()).is_ok());
    let contents = fs::read_to_string(file_path).unwrap();
    println!("File has contents:\n{contents}");
}

#[test]
fn test_streaming_symmetric_encrypt_decrypt_file() {
    println!("Streaming symmetric encrypt/decrypt file");
    // wrap a file with encryption
}

