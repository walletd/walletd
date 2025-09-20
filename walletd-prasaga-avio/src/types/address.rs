use crate::types::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrasagaAvioAddress {
    bytes: Vec<u8>,
    prefix: String,
}

impl PrasagaAvioAddress {
    /// Create from public key bytes
    pub fn from_public_key(public_key: &[u8]) -> Result<Self> {
        if public_key.len() != 32 {
            return Err(Error::InvalidAddress("Public key must be 32 bytes".into()));
        }

        // Use similar approach to other chains: hash the public key
        let hash = blake3::hash(public_key);
        let hash_bytes = hash.as_bytes();

        // Take first 20 bytes like Ethereum
        let mut address_bytes = vec![0u8; 20];
        address_bytes.copy_from_slice(&hash_bytes[..20]);

        Ok(Self {
            bytes: address_bytes,
            prefix: "saga".to_string(), // Assumed prefix
        })
    }

    /// Create from string representation
    pub fn parse_address(s: &str) -> Result<Self> {
        // Expected format: saga1xxxxx... (bech32-like)
        if !s.starts_with("saga") {
            return Err(Error::InvalidAddress(
                "Address must start with 'saga'".into(),
            ));
        }

        // For now, decode as hex after prefix
        let hex_part = s
            .strip_prefix("saga")
            .ok_or_else(|| Error::InvalidAddress("Invalid address format".into()))?;

        let bytes = hex::decode(hex_part)
            .map_err(|e| Error::InvalidAddress(format!("Invalid hex in address: {e}")))?;

        Ok(Self {
            bytes,
            prefix: "saga".to_string(),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Display for PrasagaAvioAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.prefix, hex::encode(&self.bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_from_public_key() {
        let public_key = [0u8; 32];
        let address = PrasagaAvioAddress::from_public_key(&public_key).unwrap();
        assert_eq!(address.as_bytes().len(), 20);
        assert!(address.to_string().starts_with("saga"));
    }
}
