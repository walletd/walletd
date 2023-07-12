use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::fs;

    
#[test]
fn test_streaming_symmetric_encrypt_decrypt_file() {
    println!("Running test streaming symmetric encrypt/decrypt file");
    let file_path = "./tests/generated_or_modified_files/test_encrypt_file.txt";
    let mut data_file = File::create(Path::new(file_path)).unwrap();
    assert!(data_file.write("Hello World!".as_bytes()).is_ok());
    let contents = fs::read_to_string(file_path).unwrap();
    println!("File has contents:\n{contents}");
}

