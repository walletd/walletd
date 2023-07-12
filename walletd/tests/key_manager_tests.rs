#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;
    
    #[test]
    fn test_streaming_symmetric_encrypt_decrypt_file() {
        println!("Running test streaming symmetric encrypt/decrypt file");
        let mut data_file = File::create(Path::new("../test_encrypt_file.txt")).unwrap();
        data_file.write("Hello World!".as_bytes()).unwrap();
        // read file

        // write out the contents of the file
    }
}
