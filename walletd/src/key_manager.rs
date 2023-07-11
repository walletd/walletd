use crate::HDKey;

/// Manages a wallet's keys
pub trait KeyManager {
    /// Import from keystore
    fn from_keystore(); 

    /// Export to keystore
    fn to_keystore();
}


