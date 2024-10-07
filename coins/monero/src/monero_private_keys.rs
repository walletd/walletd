//! Monero Private Keys - a private spend key and a private view key derived
//! from the private spend key

use curve25519_dalek::scalar::Scalar;
use thiserror::Error;

use crate::private_key::KEY_LEN;
use crate::{keccak256, private_key, PrivateKey, SubaddressIndex};

/// A Monero full private key contains both the spend_key and view_key
/// information. The view_key is an optional struct field because it is possible
/// that the private view key is known without knowing the private spend key.
/// However, knowing the private spend key allows one to derive the private view
/// key. The private view key is needed to recognize owned outputs and amounts.
/// The private spend key is needed to spend owned outputs.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct MoneroPrivateKeys {
    spend_key: Option<PrivateKey>,
    view_key: PrivateKey,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum Error {
    /// Data has invalid byte length
    #[error("invalid byte length, expected: {expected:?}, found: {found:?}")]
    InvalidByteLength { expected: usize, found: usize },
    /// Error coming from the private_key module
    #[error("private_key error: {0}")]
    PrivateKeyError(#[from] private_key::Error),
}

impl MoneroPrivateKeys {
    /// Construct the MoneroPrivateKeys struct given a byte slice of the seed
    /// The seed is "reduced" to become the private spend key
    /// The private view key is derived from the private spend key and the
    /// MoneroPrivateKey struct will contain the information for private view
    /// and spend keys
    pub fn from_seed(seed: &[u8]) -> Result<Self, Error> {
        if seed.len() != KEY_LEN {
            return Err(Error::InvalidByteLength {
                expected: KEY_LEN,
                found: seed.len(),
            });
        }
        let mut seed_bytes = [0u8; 32];
        seed_bytes.copy_from_slice(seed);
        let private_spend_key = Scalar::from_bytes_mod_order(seed_bytes).to_bytes();
        Self::from_private_spend_key(&private_spend_key)
    }

    /// Construct a MoneroPrivateKeys (with info on both the private view and
    /// spend keys) the spend key given as a byte slice
    pub fn from_private_spend_key(private_spend_key: &[u8]) -> Result<Self, Error> {
        let spend_key = PrivateKey::from_slice(private_spend_key)?;
        let view_key = PrivateKey::from_slice(
            &Scalar::from_bytes_mod_order(keccak256(private_spend_key)).to_bytes(),
        )?;
        Ok(Self {
            spend_key: Some(spend_key),
            view_key,
        })
    }

    /// Construct a MoneroPrivateKeys struct given a byte slice with info on the
    /// private view key, the private spend key field of the constructed struct
    /// will be None
    pub fn from_private_view_key(private_view_key: &[u8]) -> Result<Self, Error> {
        let view_key = PrivateKey::from_slice(private_view_key)?;
        Ok(Self {
            view_key,
            spend_key: None,
        })
    }

    /// Return the private view key
    pub fn view_key(&self) -> PrivateKey {
        self.view_key
    }

    /// Return the optional private spend key
    pub fn spend_key(&self) -> Option<PrivateKey> {
        self.spend_key
    }

    /// Given the MoneroPrivateKeys of the primary account, calculates the
    /// MoneroPrivateKeys of subaddress index (major and minor indices
    /// specified)
    pub fn to_subaddress_private_keys(
        &self,
        index: SubaddressIndex,
    ) -> Result<MoneroPrivateKeys, Error> {
        if index.is_zero() {
            return Ok(*self);
        }
        let (major, minor) = index.as_tuple();
        let mut derivation: Vec<_> = b"SubAddr\x00"[..].into();
        derivation.extend(self.view_key().to_bytes());
        derivation.extend(major.to_le_bytes());
        derivation.extend(minor.to_le_bytes());

        let view_key = PrivateKey::from_slice(
            &Scalar::from_bytes_mod_order(keccak256(&derivation)).to_bytes(),
        )?;

        Ok(MoneroPrivateKeys {
            spend_key: None,
            view_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    struct KeyInfo<'a> {
        seed: &'a [u8],
        private_spend_key: &'a [u8],
        private_view_key: &'a [u8],
    }

    const VALID_INFO_1: KeyInfo = KeyInfo {
        seed: &hex!("3eb8e283b45559d4d2fb6b3a4f52443b420e6da2b38832ea0eb642100c92d600"),
        private_spend_key: &hex!(
            "3eb8e283b45559d4d2fb6b3a4f52443b420e6da2b38832ea0eb642100c92d600"
        ),
        private_view_key: &hex!("5177c436f032666c572df97ab591cc6ac2da96ab6818a2f38d72b430aebbdc0a"),
    };

    const INVALID_INFO_1: KeyInfo = KeyInfo {
        seed: &hex!("3eb8e283b45559d4d2fb6b3a4f52443b420e6da2b38832ea0eb642100c92d60011"),
        private_spend_key: &hex!(
            "3eb8e283b45559d4d2fb6b3a4f52443b420e6da2b38832ea0eb642100c92d600"
        ),
        private_view_key: &hex!("5177c436f032666c572df97ab591cc6ac2da96ab6818a2f38d72b430aebbdc0a"),
    };

    #[test]
    fn test_from_seed() {
        // Test with invalid length
        let result = MoneroPrivateKeys::from_seed(INVALID_INFO_1.seed);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidByteLength { expected, found } => {
                assert_eq!(expected, KEY_LEN);
                assert_eq!(found, 33);
            }
            _ => panic!("Unexpected error"),
        }

        // Test with correct length
        let result = MoneroPrivateKeys::from_seed(VALID_INFO_1.seed).unwrap();
        let private_spend_key = result.spend_key().unwrap();
        let private_view_key = result.view_key();
        assert_eq!(private_spend_key.as_slice(), VALID_INFO_1.private_spend_key);
        assert_eq!(private_view_key.as_slice(), VALID_INFO_1.private_view_key);
    }
    #[test]
    fn test_from_private_spend_key() {
        let result = MoneroPrivateKeys::from_private_spend_key(VALID_INFO_1.private_spend_key);
        assert!(result.is_ok());
        let private_keys = result.unwrap();
        let private_spend_key = private_keys.spend_key().unwrap();
        let private_view_key = private_keys.view_key();
        assert_eq!(private_spend_key.as_slice(), VALID_INFO_1.private_spend_key);
        assert_eq!(private_view_key.as_slice(), VALID_INFO_1.private_view_key);
    }

    #[test]
    fn test_from_private_view_key() {
        let result = MoneroPrivateKeys::from_private_view_key(VALID_INFO_1.private_view_key);
        assert!(result.is_ok());
        let private_keys = result.unwrap();
        assert!(private_keys.spend_key().is_none());
        let private_view_key = private_keys.view_key();
        assert_eq!(private_view_key.as_slice(), VALID_INFO_1.private_view_key);
    }
}
