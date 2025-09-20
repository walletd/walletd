//! Utility functions and helpers

use sha3::{Digest, Keccak256};

/// Calculate Keccak256 hash
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Convert bytes to hex string
pub fn to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Convert hex string to bytes
pub fn from_hex(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}
