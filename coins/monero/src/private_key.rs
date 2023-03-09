//! A Private Key with 32 bytes of data that should be a valid Curve25519 scalar

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use curve25519_dalek::scalar::Scalar;
use thiserror::Error;

pub const KEY_LEN: usize = 32;
use rand::{thread_rng, Rng};

/// A Monero private spend or view key should be a a valid Curve25519 scalar.
/// It can be represented as 32 bytes.
/// This struct can be used to represent either a private view key or a private
/// spend key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrivateKey(pub Scalar);

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
    // Generates a new random PrivateKey
    pub fn new() -> Self {
        PrivateKey(Scalar::from_bytes_mod_order(thread_rng().gen()))
    }

    /// Create a PrivateKey from a slice of a byte array.
    /// First convers the byte array to a valid Scalar using
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
    pub fn as_slice(&self) -> &[u8] {
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
    pub fn from_scalar(scalar: &Scalar) -> Self {
        Self { 0: scalar.clone() }
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_slice()))
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
    use hex_literal::hex;

    use super::*;

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
        let bytes = private_key.as_slice();
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

    #[test]
    fn test_check_scalar_validity() {
        // check_scalar ac10e070c8574ef374bdd1c5dbe9bacfd927f9ae0705cf08018ff865f6092d0f
        // true
        let bytes_1 = hex!("ac10e070c8574ef374bdd1c5dbe9bacfd927f9ae0705cf08018ff865f6092d0f");
        let actual_1 = PrivateKey::from_slice(&bytes_1);
        assert!(actual_1.is_ok());
        // check_scalar 18fd66f7a0874de792f12a1b2add7d294100ea454537ae5794d0abc91dbf098a
        // false
        let bytes_2 = hex!("18fd66f7a0874de792f12a1b2add7d294100ea454537ae5794d0abc91dbf098a");
        let actual_2 = PrivateKey::from_slice(&bytes_2);
        assert!(actual_2.is_err());
        // check_scalar ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
        // false
        let bytes_3 = hex!("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        let actual_3 = PrivateKey::from_slice(&bytes_3);
        assert!(actual_3.is_err());
    }
}
