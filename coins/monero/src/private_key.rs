use std::fmt::{self, Display};
use std::str::FromStr;

use curve25519_dalek::scalar::Scalar;
use rand::{thread_rng, RngCore};
use thiserror::Error;

pub const KEY_LEN: usize = 32;

/// Represents a Monero private key (view or spend).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PrivateKey(pub Scalar);

#[derive(Debug, Error, PartialEq, Clone)]
pub enum Error {
    #[error("Invalid length, expected {expected}, found {found}")]
    InvalidLength { expected: usize, found: usize },
    #[error("Not a canonical scalar")]
    NotCanonicalScalar,
    #[error("Hex decode error: {0}")]
    FromHex(#[from] hex::FromHexError),
}

impl PrivateKey {
    /// Generates a new random private key.
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        Self(Scalar::from_bytes_mod_order(bytes))
    }

    /// Creates a private key from a byte slice.
    pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
        if data.len() != KEY_LEN {
            return Err(Error::InvalidLength {
                expected: KEY_LEN,
                found: data.len(),
            });
        }
        let mut bytes = [0u8; KEY_LEN];
        bytes.copy_from_slice(data);
        let scalar = Scalar::from_canonical_bytes(bytes);
        if scalar.is_some().into() {
            Ok(Self(scalar.unwrap()))
        } else {
            Err(Error::NotCanonicalScalar)
        }
    }

    /// Returns the key as a byte slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Converts the key to a byte array.
    pub fn to_bytes(&self) -> [u8; KEY_LEN] {
        self.0.to_bytes()
    }

    /// Returns the underlying scalar.
    pub fn as_scalar(&self) -> &Scalar {
        &self.0
    }

    /// Constructs a private key from a scalar.
    pub fn from_scalar(scalar: &Scalar) -> Self {
        Self(*scalar)
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
        Self::from_slice(&bytes)
    }
}
impl PrivateKey {
    pub fn to_monero(&self) -> monero::PrivateKey {
        let bytes = self.0.to_bytes();
        monero::PrivateKey::from_slice(&bytes).unwrap()
    }

    pub fn from_monero(key: &monero::PrivateKey) -> Self {
        let bytes = key.as_bytes();
        PrivateKey(Scalar::from_bytes_mod_order(bytes.try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_from_slice() {
        let data = [0u8; KEY_LEN];
        let result = PrivateKey::from_slice(&data);
        assert!(result.is_ok());

        let data = [0u8; KEY_LEN + 1];
        let result = PrivateKey::from_slice(&data);
        assert!(matches!(
            result.unwrap_err(),
            Error::InvalidLength {
                expected: 32,
                found: 33
            }
        ));

        let data = [255u8; KEY_LEN];
        let result = PrivateKey::from_slice(&data);
        assert!(matches!(result.unwrap_err(), Error::NotCanonicalScalar));
    }

    #[test]
    fn test_as_bytes() {
        let value = [1u8; 32];
        let private_key = PrivateKey(Scalar::from_bytes_mod_order(value));
        let bytes = private_key.as_slice();
        assert_eq!(bytes.len(), KEY_LEN);
        assert_eq!(bytes, &value);
    }

    #[test]
    fn test_to_bytes() {
        let value = [1u8; 32];
        let private_key = PrivateKey(Scalar::from_bytes_mod_order(value));
        let bytes = private_key.to_bytes();
        assert_eq!(bytes.len(), KEY_LEN);
        assert_eq!(bytes, value);
    }

    #[test]
    fn test_display() {
        let value = [1u8; 32];
        let private_key = PrivateKey(Scalar::from_bytes_mod_order(value));
        let s = format!("{private_key}");
        assert_eq!(s, hex::encode(value));
    }

    #[test]
    fn test_from_str() {
        let value = [1u8; 32];
        let hex_string = hex::encode(value);
        let result = PrivateKey::from_str(&hex_string);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_bytes(), value);

        let hex_string = "invalid";
        let result = PrivateKey::from_str(hex_string);
        assert!(matches!(result.unwrap_err(), Error::FromHex(_)));
    }

    #[test]
    fn test_scalar_validity() {
        let bytes_1 = hex!("ac10e070c8574ef374bdd1c5dbe9bacfd927f9ae0705cf08018ff865f6092d0f");
        assert!(PrivateKey::from_slice(&bytes_1).is_ok());

        let bytes_2 = hex!("18fd66f7a0874de792f12a1b2add7d294100ea454537ae5794d0abc91dbf098a");
        assert!(matches!(
            PrivateKey::from_slice(&bytes_2).unwrap_err(),
            Error::NotCanonicalScalar
        ));

        let bytes_3 = hex!("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        assert!(matches!(
            PrivateKey::from_slice(&bytes_3).unwrap_err(),
            Error::NotCanonicalScalar
        ));
    }
}
