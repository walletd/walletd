//! A Public Key with 32 bytes of data which should be a valid point on the
//! Twisted Edwards curve Ed25519

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use curve25519_dalek::constants::ED25519_BASEPOINT_TABLE as G_BASEPOINT;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use thiserror::Error;

use crate::private_key::{PrivateKey, KEY_LEN};
use crate::{DoSerialize, SerializedArchive};

/// A Monero public spend or view key should be a valid Ed25519 point on the
/// Edwards elliptic curve. It can be represented as 32 bytes.
/// This struct can be used to represent either a public view key or a public
/// spend key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
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
impl DoSerialize for PublicKey {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.data.extend_from_slice(self.as_slice());
        serialized
            .json_stream
            .push_str(&hex::encode(self.as_slice().to_vec()));
        Ok(())
    }
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
    pub fn as_slice(&self) -> &[u8] {
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
        write!(f, "{}", hex::encode(self.as_slice()))
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
        let bytes = public_key.as_slice();
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
        let private_key = PrivateKey::from_scalar(&Scalar::from_bytes_mod_order(value));
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

    #[test]
    fn test_check_keys() {
        // check_key c2cb3cf3840aa9893e00ec77093d3d44dba7da840b51c48462072d58d8efd183
        // false
        let key_1 = hex!("c2cb3cf3840aa9893e00ec77093d3d44dba7da840b51c48462072d58d8efd183");
        let actual_1 = PublicKey::from_slice(&key_1);
        assert!(actual_1.is_err());

        // check_key bd85a61bae0c101d826cbed54b1290f941d26e70607a07fc6f0ad611eb8f70a6
        // true
        let key_2 = hex!("bd85a61bae0c101d826cbed54b1290f941d26e70607a07fc6f0ad611eb8f70a6");
        let actual_2 = PublicKey::from_slice(&key_2);
        assert!(actual_2.is_ok());

        // check_key 328f81cad4eba24ab2bad7c0e56b1e2e7346e625bcb06ae649aef3ffa0b8bef3
        // false
        let key_3 = hex!("328f81cad4eba24ab2bad7c0e56b1e2e7346e625bcb06ae649aef3ffa0b8bef3");
        let actual_3 = PublicKey::from_slice(&key_3);
        assert!(actual_3.is_err());

        // check_key 6016a5463b9e5a58c3410d3f892b76278883473c3f0b69459172d3de49e85abe
        // true
        let key_4 = hex!("6016a5463b9e5a58c3410d3f892b76278883473c3f0b69459172d3de49e85abe");
        let actual_4 = PublicKey::from_slice(&key_4);
        let expected_4 = PublicKey::from_slice(&key_4);
        assert!(actual_4.is_ok());

        // check_key 4c71282b2add07cdc6898a2622553f1ca4eb851e5cb121181628be5f3814c5b1
        // false
        let key_5 = hex!("4c71282b2add07cdc6898a2622553f1ca4eb851e5cb121181628be5f3814c5b1");
        let actual_5 = PublicKey::from_slice(&key_5);
        assert!(actual_5.is_err());
    }

    #[test]
    fn test_check_from_private() {
        // secret_key_to_public_key
        // b2f420097cd63cdbdf834d090b1e604f08acf0af5a3827d0887863aaa4cc4406 true
        // d764c19d6c14280315d81eb8f2fc777582941047918f52f8dcef8225e9c92c52
        let secret_key_1 = hex!("b2f420097cd63cdbdf834d090b1e604f08acf0af5a3827d0887863aaa4cc4406");
        let expected_pub_key_1 =
            hex!("d764c19d6c14280315d81eb8f2fc777582941047918f52f8dcef8225e9c92c52");
        let actual_pub_key_1 =
            PublicKey::from_private_key(&PrivateKey::from_slice(&secret_key_1).unwrap());
        assert_eq!(actual_pub_key_1.to_bytes(), expected_pub_key_1);

        // secret_key_to_public_key
        // f264699c939208870fecebc013b773b793dd18ea39dbe1cb712a19a692fdb000 true
        // bcb483f075d37658b854d4b9968fafae976e5532ca99879479c85ef5da1deead
        let secret_key_2 = hex!("f264699c939208870fecebc013b773b793dd18ea39dbe1cb712a19a692fdb000");
        let expected_pub_key_2 =
            hex!("bcb483f075d37658b854d4b9968fafae976e5532ca99879479c85ef5da1deead");
        let actual_pub_key_2 =
            PublicKey::from_private_key(&PrivateKey::from_slice(&secret_key_2).unwrap());
        assert_eq!(actual_pub_key_2.to_bytes(), expected_pub_key_2);

        // secret_key_to_public_key
        // bd65eb76171bb9b9542a6e06b9503c09fd4a9290fe51828ed766e5aeb742dc02 true
        // 1dec6cc63ff1984ee46a70a46687877a87fcc1e790562da73b33b1a8fd8cad37
        let secret_key_3 = hex!("bd65eb76171bb9b9542a6e06b9503c09fd4a9290fe51828ed766e5aeb742dc02");
        let expected_pub_key_3 =
            hex!("1dec6cc63ff1984ee46a70a46687877a87fcc1e790562da73b33b1a8fd8cad37");
        let actual_pub_key_3 =
            PublicKey::from_private_key(&PrivateKey::from_slice(&secret_key_3).unwrap());
        assert_eq!(actual_pub_key_3.to_bytes(), expected_pub_key_3);

        // secret_key_to_public_key
        // 37621ebd8de6ca022419fd083066285da76ada9bae6d2b7c1a3847d78a726b0b true
        // 25255c26721758456545bcaea3a99407cd3df7c8208eeb49bd80450627138fab
        let secret_key_4 = hex!("37621ebd8de6ca022419fd083066285da76ada9bae6d2b7c1a3847d78a726b0b");
        let expected_pub_key_4 =
            hex!("25255c26721758456545bcaea3a99407cd3df7c8208eeb49bd80450627138fab");
        let actual_pub_key_4 =
            PublicKey::from_private_key(&PrivateKey::from_slice(&secret_key_4).unwrap());
        assert_eq!(actual_pub_key_4.to_bytes(), expected_pub_key_4);
    }
}
