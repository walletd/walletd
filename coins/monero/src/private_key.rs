//! A Private Key with 32 bytes of data that should be a valid Curve25519 scalar

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use curve25519_dalek::scalar::Scalar;
use thiserror::Error;

pub const KEY_LEN: usize = 32;

/// A Monero private spend or view key should be a a valid Curve25519 scalar.
/// It can be represented as 32 bytes.
/// This struct can be used to represent either a private view key or a private
/// spend key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrivateKey(Scalar);

#[derive(Debug, Error, PartialEq, Clone)]
pub enum Error {
    /// Data has an invalid length to be a PrivateKey
    #[error("Data has an invalid length to be a PrivateKey, expected length {expected:?}, found length {found:?}")]
    InvalidLength { expected: usize, found: usize },
    /// Data is outside the required range cannot be converted into a Scalar
    /// from canonical bytes
    #[error("Data is outside range, not a canonical representation of a ed25519 scalar")]
    NotCanonicalScalar,
    /// Error in decoding hex
    #[error("Hex string could not be parsed to bytes, Error: {0}")]
    HexError(#[from] hex::FromHexError),
}

impl PrivateKey {
    /// Create a PrivateKey from a slice of a byte array.
    /// Returns an error if the slice is not appropriate to be converted to a
    /// PrivateKey.
    pub fn from_slice(data: &[u8]) -> Result<PrivateKey, Error> {
        if data.len() != KEY_LEN {
            return Err(Error::InvalidLength {
                expected: KEY_LEN,
                found: data.len(),
            });
        }
        let mut bytes = [0u8; KEY_LEN];
        bytes.copy_from_slice(data);
        match Scalar::from_canonical_bytes(bytes) {
            Some(scalar) => return Ok(PrivateKey(scalar)),
            None => return Err(Error::NotCanonicalScalar),
        }
    }

    /// Represent the PrivateKey as a reference to a slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Convert the PrivateKey to an array of bytes.
    pub fn to_bytes(&self) -> [u8; KEY_LEN] {
        self.0.to_bytes()
    }

    /// Returns the underlying Scalar associated with the PrivateKey
    pub fn as_scalar(&self) -> Scalar {
        self.0
    }

    /// Construct a PrivateKey from a Scalar
    pub fn from_scalar(scalar: Scalar) -> Self {
        Self { 0: scalar }
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_bytes()))
    }
}

impl FromStr for PrivateKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        Self::from_slice(&bytes[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::private_key;

    #[test]
    fn test_from_slice() {
        // Test with correct length
        let data = [0u8; KEY_LEN];
        let result = PrivateKey::from_slice(&data);
        assert!(result.is_ok());

        // Test with incorrect length
        let data = [0u8; KEY_LEN + 1];
        let result = PrivateKey::from_slice(&data);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidLength { expected, found } => {
                assert_eq!(expected, KEY_LEN);
                assert_eq!(found, KEY_LEN + 1);
            }
            _ => panic!("Unexpected error"),
        }

        // Test with incorrect data
        let data = [255u8; KEY_LEN];
        let result = PrivateKey::from_slice(&data);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::NotCanonicalScalar => {}
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_as_bytes() {
        let value = [1u8; 32];
        let private_key = PrivateKey(Scalar::from_bytes_mod_order(value));
        let bytes = private_key.as_bytes();
        assert_eq!(bytes.len(), KEY_LEN);
        assert_eq!(bytes, &value)
    }

    #[test]
    fn test_to_bytes() {
        let value = [1u8; 32];
        let private_key = PrivateKey(Scalar::from_bytes_mod_order(value));
        let bytes = private_key.to_bytes();
        assert_eq!(bytes.len(), KEY_LEN);
        assert_eq!(bytes, value)
    }

    #[test]
    fn test_display() {
        let value = [1u8; 32];
        let private_key = PrivateKey(Scalar::from_bytes_mod_order(value));
        let s = format!("{}", private_key);
        assert_eq!(s.len(), KEY_LEN * 2);
        assert_eq!(s, hex::encode(value))
    }

    #[test]
    fn test_from_str() {
        let value = [1u8; 32];
        let hex_string = hex::encode(value);
        let expected = PrivateKey(Scalar::from_bytes_mod_order(value));
        let result = PrivateKey::from_str(&hex_string);
        assert!(result.is_ok());
        let private_key = result.unwrap();
        assert_eq!(private_key, expected);

        let hex_string = "not a hex string";
        let result = PrivateKey::from_str(hex_string);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::HexError(_) => {}
            _ => panic!("Unexpected error"),
        }
    }
}
