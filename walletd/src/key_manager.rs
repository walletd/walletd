use crate::HDKey;
use crate::KeyPair;
use crate::{KeyStore, LockState};
use crate::keystore::{Unlocked, Locked};
use crate::Error;
use std::fs::File;
use chacha20poly1305;

pub trait KeyManager {}

impl KeyManager for KeyStore<Unlocked> {
    /// Import from keystore
    fn from_keystore(key_store: KeyStore<Unlocked>) {
        todo!()
    }

    /// Export to keystore
    fn to_keystore(&self) -> KeyStore<Unlocked> {
        todo!()
    }
}

impl KeyManager for KeyStore<Locked> {
    /// Import from keystore, decrypt first
    fn from_keystore(key_store: KeyStore<Locked>) {
        todo!()

    }

    /// Export to keystore, encrypt first
    fn to_keystore(&self) -> KeyStore<Locked> {
        todo!()
    }
}



impl KeyManager for KeyPair {
    fn from_keystore(&self) -> Result<KeyStore<Locked>, Error> {
        todo!()
    }

    fn to_keystore(&self) -> Result<KeyStore<Locked>, Error> {
        todo!()
    }
}





#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Write;


    #[test]
    fn test_streaming_symmetric_encrypt_file() -> {

    }

    #[test]
    fn test_streaming_symmetric_decrypt_file() -> {

    }
}
