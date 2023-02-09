//! A Public Key with 32 bytes of data which should be a valid point on the
//! Twisted Edwards curve Ed25519

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use curve25519_dalek::constants::ED25519_BASEPOINT_TABLE as G_BASEPOINT;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use thiserror::Error;

use crate::private_key::{PrivateKey, KEY_LEN};

/// A Monero public spend or view key should be a valid Ed25519 point on the
/// Edwards elliptic curve. It can be represented as 32 bytes.
/// This struct can be used to represent either a public view key or a public
/// spend key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicKey(pub CompressedEdwardsY);

#[derive(Debug, Error, Clone, Copy, PartialEq)]
pub enum Error {
    /// Data has an invalid length to be a PublicKey
    #[error("Data has an invalid length to be a PublicKey, expected length {expected:?}, found length {found:?}")]
    InvalidLength { expected: usize, found: usize },
    /// Invalid point on the Edwards elliptic curve
    #[error("Invalid point on the Edwards elliptic curve")]
    InvalidPoint,
    /// Error in decoding hex
    #[error("Hex string could not be parsed to bytes, Error: {0}")]
    HexError(#[from] hex::FromHexError),
}

impl PublicKey {
    /// Create a PrivateKey from a slice of a byte array
    /// Returns an error if the slice is not appropriate to be converted to a
    /// PublicKey
    pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
        if data.len() != KEY_LEN {
            return Err(Error::InvalidLength {
                expected: KEY_LEN,
                found: data.len(),
            });
        }
        let point = CompressedEdwardsY::from_slice(data);
        match point.decompress() {
            Some(_) => return Ok(PublicKey(point)),
            None => return Err(Error::InvalidPoint),
        }
    }

    /// Generates a PublicKey derived from a given PrivateKey
    pub fn from_private_key(private_key: &PrivateKey) -> Self {
        let point = &private_key.as_scalar() * &G_BASEPOINT;
        PublicKey(point.compress())
    }

    /// Represent the PublicKey as a reference to a slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Convert the PublicKey to an array of bytes.
    pub fn to_bytes(&self) -> [u8; KEY_LEN] {
        self.0.to_bytes()
    }

    /// Returns the underlying EdwardsPoint
    pub fn to_edwards_point(&self) -> EdwardsPoint {
        self.0
            .decompress()
            .expect("public key should be a valid point")
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_bytes()))
    }
}

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        Self::from_slice(&bytes[..])
    }
}

#[cfg(test)]
mod tests {
    use curve25519_dalek::scalar::Scalar;
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_from_slice() {
        // Test with correct length
        let data = [0u8; KEY_LEN];
        let result = PublicKey::from_slice(&data);
        assert!(result.is_ok());

        // Test with incorrect length
        let data = [0u8; KEY_LEN + 1];
        let result = PublicKey::from_slice(&data);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidLength { expected, found } => {
                assert_eq!(expected, KEY_LEN);
                assert_eq!(found, KEY_LEN + 1);
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_as_bytes() {
        let value = [1u8; 32];
        let public_key = PublicKey(CompressedEdwardsY::from_slice(&value));
        let bytes = public_key.as_bytes();
        assert_eq!(bytes.len(), KEY_LEN);
        assert_eq!(bytes, &value)
    }

    #[test]
    fn test_to_bytes() {
        let value = [1u8; 32];
        let public_key = PublicKey(CompressedEdwardsY::from_slice(&value));
        let bytes = public_key.to_bytes();
        assert_eq!(bytes.len(), KEY_LEN);
        assert_eq!(bytes, value);
    }

    #[test]
    fn test_from_private_key() {
        // Checking with underlying math
        let value = [1u8; 32];
        let private_key = PrivateKey::from_scalar(Scalar::from_bytes_mod_order(value));
        let public_key = PublicKey::from_private_key(&private_key);
        let expected_point = &private_key.as_scalar() * &G_BASEPOINT;
        let expected = PublicKey::from_slice(&expected_point.compress().to_bytes());
        assert!(expected.is_ok());
        let expected_pub_key = expected.unwrap();
        assert_eq!(public_key, expected_pub_key);

        // Checking with a known value
        let value = hex!("77916d0cd56ed1920aef6ca56d8a41bac915b68e4c46a589e0956e27a7b77404");
        let private_key = PrivateKey::from_slice(&value).unwrap();
        let public_key = PublicKey::from_private_key(&private_key);
        let expected_pub_key =
            hex!("eac2cc96e0ae684388e3185d5277e51313bff98b9ad4a12dcd9205f20d37f1a3");
        assert_eq!(public_key.to_bytes(), expected_pub_key);
    }

    #[test]
    fn test_display() {
        let value = [1u8; 32];
        let public_key = PublicKey(CompressedEdwardsY::from_slice(&value));
        let hex_string = format!("{}", public_key);
        assert_eq!(hex_string, hex::encode(value));
    }

    #[test]
    fn test_from_str() {
        let value = [1u8; 32];
        let hex_string = hex::encode(value);
        let public_key = PublicKey::from_str(&hex_string).unwrap();
        let expected = CompressedEdwardsY::from_slice(&value);
        assert_eq!(public_key.0, expected);

        let hex_string = "not a hex string";
        let result = PublicKey::from_str(hex_string);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::HexError(_) => {}
            _ => panic!("Unexpected error"),
        }
    }
}
