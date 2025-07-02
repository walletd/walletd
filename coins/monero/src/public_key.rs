use std::fmt::{self, Display};
use std::str::FromStr;

use curve25519_dalek::{
    constants::ED25519_BASEPOINT_TABLE as G_BASEPOINT,
    edwards::{CompressedEdwardsY, EdwardsPoint},
    traits::Identity,
};
use thiserror::Error;

use crate::{
    monero_serialize::{DoSerialize, SerializedArchive},
    private_key::{PrivateKey, KEY_LEN},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct PublicKey(pub CompressedEdwardsY);

#[derive(Debug, Error, Clone, Copy, PartialEq)]
pub enum Error {
    #[error("Invalid length, expected {expected}, found {found}")]
    InvalidLength { expected: usize, found: usize },
    #[error("Invalid point on Edwards curve")]
    InvalidPoint,
    #[error("Hex decode error: {0}")]
    FromHex(#[from] hex::FromHexError),
}

impl DoSerialize for PublicKey {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.data.extend_from_slice(self.as_slice());
        serialized
            .json_stream
            .push_str(&hex::encode(self.as_slice()));
        Ok(())
    }
}

impl PublicKey {
    pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
        if data.len() != KEY_LEN {
            return Err(Error::InvalidLength {
                expected: KEY_LEN,
                found: data.len(),
            });
        }
        let point = CompressedEdwardsY::from_slice(data).map_err(|_| Error::InvalidPoint)?;
        if point.decompress().is_some() {
            Ok(Self(point))
        } else {
            Err(Error::InvalidPoint)
        }
    }

    pub fn from_private_key(private_key: &PrivateKey) -> Self {
        let point = private_key.as_scalar() * G_BASEPOINT;
        Self(point.compress())
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn to_bytes(&self) -> [u8; KEY_LEN] {
        self.0.to_bytes()
    }

    pub fn to_edwards_point(&self) -> EdwardsPoint {
        self.0.decompress().expect("Valid point")
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_slice()))
    }
}

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        Self::from_slice(&bytes)
    }
}
impl PublicKey {
    /// Decompresses the public key to an EdwardsPoint
    pub fn decompress(&self) -> EdwardsPoint {
        self.0.decompress().unwrap_or_else(|| {
            // Return identity if decompression fails
            EdwardsPoint::identity()
        })
    }
}

impl serde::Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_slice())
    }
}

impl PublicKey {
    pub fn to_monero(&self) -> monero::PublicKey {
        let bytes = self.0.to_bytes();
        monero::PublicKey::from_slice(&bytes).unwrap()
    }

    pub fn from_monero(key: &monero::PublicKey) -> Self {
        let bytes = key.as_bytes();
        PublicKey(CompressedEdwardsY::from_slice(bytes).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_dalek::scalar::Scalar;
    use hex_literal::hex;

    #[test]
    fn test_from_slice() {
        let data = [0u8; KEY_LEN];
        let result = PublicKey::from_slice(&data);
        assert!(result.is_ok());

        let data = [0u8; KEY_LEN + 1];
        let result = PublicKey::from_slice(&data);
        assert!(matches!(
            result.unwrap_err(),
            Error::InvalidLength {
                expected: 32,
                found: 33
            }
        ));
    }

    #[test]
    fn test_as_bytes() {
        let value = [1u8; 32];
        let public_key = PublicKey(CompressedEdwardsY(value));
        let bytes = public_key.as_slice();
        assert_eq!(bytes.len(), KEY_LEN);
        assert_eq!(bytes, &value);
    }

    #[test]
    fn test_to_bytes() {
        let value = [1u8; 32];
        let public_key = PublicKey(CompressedEdwardsY(value));
        let bytes = public_key.to_bytes();
        assert_eq!(bytes.len(), KEY_LEN);
        assert_eq!(bytes, value);
    }

    #[test]
    fn test_from_private_key() {
        let value = [1u8; 32];
        let private_key = PrivateKey(Scalar::from_bytes_mod_order(value));
        let public_key = PublicKey::from_private_key(&private_key);
        let expected_point = private_key.as_scalar() * G_BASEPOINT;
        let expected = PublicKey(expected_point.compress());
        assert_eq!(public_key, expected);

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
        let public_key = PublicKey(CompressedEdwardsY(value));
        let hex_string = format!("{public_key}");
        assert_eq!(hex_string, hex::encode(value));
    }

    #[test]
    fn test_from_str() {
        let value = [1u8; 32];
        let hex_string = hex::encode(value);
        let public_key = PublicKey::from_str(&hex_string).unwrap();
        assert_eq!(public_key.0 .0, value);

        let hex_string = "invalid";
        let result = PublicKey::from_str(hex_string);
        assert!(matches!(result.unwrap_err(), Error::FromHex(_)));
    }

    #[test]
    fn test_check_keys() {
        let key_1 = hex!("c2cb3cf3840aa9893e00ec77093d3d44dba7da840b51c48462072d58d8efd183");
        assert!(PublicKey::from_slice(&key_1).is_err());

        let key_2 = hex!("bd85a61bae0c101d826cbed54b1290f941d26e70607a07fc6f0ad611eb8f70a6");
        assert!(PublicKey::from_slice(&key_2).is_ok());
    }
}
