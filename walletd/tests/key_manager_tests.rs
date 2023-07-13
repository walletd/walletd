use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs;
use crypto::aessafe::{AesSafe128Encryptor, AesSafe128Decryptor};
use rand::{thread_rng, Rng};
use aesstream::{AesWriter, AesReader};



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
    
    let mut data_file = File::create(Path::new(file_path)).unwrap();
    let encryptor = AesSafe128Encryptor::new(&key);
    let mut writer = AesWriter::new(data_file, encryptor).unwrap();
    assert!(writer.write_all("Hello World, is this encrypted?".as_bytes()).is_ok());

    let key: [u8; 16] = thread_rng().gen();
    let read_file = File::open(Path::new(file_path)).unwrap();
    let decryptor = AesSafe128Decryptor::new(&key);
    let mut reader = AesReader::new(read_file, decryptor).unwrap();
    let mut decrypted = String::new();
    // assert!(reader.read_to_string(&mut decrypted).is_ok());
    // assert_eq!(decrypted, "Hello World, is this encrypted?");

    
}

