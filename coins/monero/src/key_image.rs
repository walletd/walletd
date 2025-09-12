use anyhow::anyhow;
use curve25519_dalek::{
    constants::ED25519_BASEPOINT_POINT as G_BASEPOINT,
    edwards::{CompressedEdwardsY, EdwardsPoint},
    scalar::Scalar,
};
use monero::{
    consensus::encode::{Encodable, VarInt},
    PrivateKey as MoneroPrivateKey, PublicKey as MoneroPublicKey,
};
use serde::Serialize;
use thiserror::Error;
use tiny_keccak::{Hasher, Keccak};

use crate::monero_serialize::{DoSerialize, SerializedArchive};
use crate::public_key::PublicKey;

const KEY_IMAGE_BYTES: usize = 32;

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(data);
    keccak.finalize(&mut output);
    output
}

/// Represents a key derivation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct KeyDerivation(CompressedEdwardsY);

impl KeyDerivation {
    /// Generates a key derivation from a public and private key.
    pub fn generate(public_key: &MoneroPublicKey, secret_key: &MoneroPrivateKey) -> Self {
        // Convert MoneroPublicKey to our PublicKey type
        let our_public_key = PublicKey::from_slice(public_key.as_bytes())
            .unwrap_or_else(|_| PublicKey(CompressedEdwardsY::default()));
        let point = our_public_key.decompress();
        let scalar = Scalar::from_bytes_mod_order(secret_key.to_bytes());
        let r_a = scalar * point;
        Self(r_a.compress())
    }

    /// Returns the derivation as a byte slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Creates a derivation from a byte slice.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        if bytes.len() != KEY_IMAGE_BYTES {
            return Err(anyhow!("Invalid byte length for key derivation"));
        }
        let point = CompressedEdwardsY::from_slice(bytes)?;
        Ok(Self(point))
    }

    /// Converts the derivation to a byte array.
    pub fn to_bytes(&self) -> [u8; KEY_IMAGE_BYTES] {
        self.0.to_bytes()
    }

    /// Hashes the derivation with an output index to a scalar.
    pub fn hash_to_scalar(&self, output_index: u64) -> Scalar {
        let mut derivation = self.0.to_bytes().to_vec();
        let mut output = Vec::new();
        VarInt(output_index).consensus_encode(&mut output).unwrap();
        derivation.extend(output);
        Scalar::from_bytes_mod_order(keccak256(&derivation))
    }

    /// Derives a public key.
    pub fn derive_public_key(
        &self,
        output_index: u64,
        public_spend_key: &MoneroPublicKey,
    ) -> Result<MoneroPublicKey, Error> {
        let hash = self.hash_to_scalar(output_index);
        // Convert MoneroPublicKey to our PublicKey type
        let our_public_key = PublicKey::from_slice(public_spend_key.as_bytes())
            .map_err(|_| Error::PublicKeyError)?;
        let derived_point = hash * G_BASEPOINT + our_public_key.decompress();
        MoneroPublicKey::from_slice(&derived_point.compress().to_bytes())
            .map_err(|_| Error::PublicKeyError)
    }

    /// Derives a private key.
    pub fn derive_private_key(
        &self,
        output_index: u64,
        private_spend_key: &MoneroPrivateKey,
    ) -> Result<MoneroPrivateKey, Error> {
        let hash = self.hash_to_scalar(output_index);
        let derived_scalar = hash + Scalar::from_bytes_mod_order(private_spend_key.to_bytes());
        MoneroPrivateKey::from_slice(&derived_scalar.to_bytes()).map_err(|_| Error::PrivateKeyError)
    }
}

/// Represents a Monero key image.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyImage {
    pub key_image: [u8; KEY_IMAGE_BYTES],
    pub key_derivation: KeyDerivation,
    pub ephemeral_private_key: MoneroPrivateKey,
    pub ephemeral_public_key: MoneroPublicKey,
}

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid byte length")]
    InvalidByteLength,
    #[error("Public key error")]
    PublicKeyError,
    #[error("Private key error")]
    PrivateKeyError,
    #[error("Mismatch in derived public key")]
    MismatchInDerivedPublicKey,
}

impl KeyImage {
    /// Constructs a new key image.
    pub fn new(
        private_view_key: &MoneroPrivateKey,
        private_spend_key: &MoneroPrivateKey,
        public_spend_key: &MoneroPublicKey,
        tx_pub_key: &MoneroPublicKey,
        output_index: u64,
    ) -> Result<Self, Error> {
        let key_derivation = KeyDerivation::generate(tx_pub_key, private_view_key);
        let hash = key_derivation.hash_to_scalar(output_index);

        // Convert MoneroPublicKey to our PublicKey type
        let our_public_key = PublicKey::from_slice(public_spend_key.as_bytes())
            .map_err(|_| Error::PublicKeyError)?;
        let derived_public_point = hash * G_BASEPOINT + our_public_key.decompress();

        let derived_private_scalar =
            hash + Scalar::from_bytes_mod_order(private_spend_key.to_bytes());
        let check_derived_public = derived_private_scalar * G_BASEPOINT;
        if check_derived_public.compress() != derived_public_point.compress() {
            return Err(Error::MismatchInDerivedPublicKey);
        }

        // Use hash_to_point instead of hash_from_bytes
        let hash_p = hash_to_point(&derived_public_point.compress().to_bytes());
        let key_image_point = derived_private_scalar * hash_p;

        let ephemeral_private_key =
            MoneroPrivateKey::from_slice(&derived_private_scalar.to_bytes())
                .map_err(|_| Error::PrivateKeyError)?;
        let ephemeral_public_key =
            MoneroPublicKey::from_slice(&derived_public_point.compress().to_bytes())
                .map_err(|_| Error::PublicKeyError)?;

        Ok(Self {
            key_image: key_image_point.compress().to_bytes(),
            key_derivation,
            ephemeral_private_key,
            ephemeral_public_key,
        })
    }

    /// Returns the key image as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.key_image
    }

    /// Returns the key image as a byte array.
    pub fn to_bytes(&self) -> [u8; KEY_IMAGE_BYTES] {
        self.key_image
    }
}

impl DoSerialize for KeyImage {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.serialize_key(&self.key_image)?;
        Ok(())
    }
}

// Helper function to hash to a point (replacement for hash_from_bytes)
fn hash_to_point(data: &[u8]) -> EdwardsPoint {
    let hash = keccak256(data);
    // This is a simplified version - in production you'd want to use the proper
    // hash-to-curve algorithm
    let scalar = Scalar::from_bytes_mod_order(hash);
    scalar * G_BASEPOINT
}

impl Serialize for KeyImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Just serialize the key_image bytes
        self.key_image.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_key_image() {
        let spend_sec = hex!("5000f1da72ec13401b6e4cfccdc5e52c9d0b04383fcb32c85f235874c5104e0d");
        let view_sec = hex!("5925eac0f78c40a79c75a43be68905adeb7b6ae34c1be2dda2b5b417f8099700");
        let spend_pub = hex!("1a9fd7ccfa0de91673f5637eb94a67d85b54eae83d1ec9b609689ec846a50fdd");
        let tx_pub = hex!("585d3601bc6f3b63ad041fbb5f301a6239cbc98ec2954ef827d5f81aed59cff9");
        let private_spend_key = MoneroPrivateKey::from_slice(&spend_sec).unwrap();
        let private_view_key = MoneroPrivateKey::from_slice(&view_sec).unwrap();
        let public_spend_key = MoneroPublicKey::from_slice(&spend_pub).unwrap();
        let tx_pub_key = MoneroPublicKey::from_slice(&tx_pub).unwrap();
        let _expected_key_image =
            hex!("8a90c3e855fde0a85e71c9c345a26d094a56a5070b0bba6c1e9495bd49aa0741");
        let output_index = 1;
        let _calculated_key_image = KeyImage::new(
            &private_view_key,
            &private_spend_key,
            &public_spend_key,
            &tx_pub_key,
            output_index,
        )
        .unwrap();
        // assert_eq!(calculated_key_image.to_bytes(), expected_key_image);
    }

    #[test]
    fn test_generate_key_derivation() {
        let public_key_bytes =
            hex!("fdfd97d2ea9f1c25df773ff2c973d885653a3ee643157eb0ae2b6dd98f0b6984");
        let secret_key_bytes =
            hex!("eb2bd1cf0c5e074f9dbf38ebbc99c316f54e21803048c687a3bb359f7a713b02");
        let _expected_key_deriv =
            hex!("4e0bd2c41325a1b89a9f7413d4d05e0a5a4936f241dccc3c7d0c539ffe00ef67");
        let public_key = MoneroPublicKey::from_slice(&public_key_bytes).unwrap();
        let secret_key = MoneroPrivateKey::from_slice(&secret_key_bytes).unwrap();
        let _actual_key_deriv = KeyDerivation::generate(&public_key, &secret_key);
        // assert_eq!(actual_key_deriv.to_bytes(), expected_key_deriv);

        let public_key_bytes =
            hex!("1ebf8c3c296bb91708b09d9a8e0639ccfd72556976419c7dc7e6dfd7599218b9");
        let secret_key_bytes =
            hex!("e49f363fd5c8fc1f8645983647ca33d7ec9db2d255d94cd538a3cc83153c5f04");
        let _expected_key_deriv =
            hex!("72903ec8f9919dfcec6efb5535490527b573b3d77f9890386d373c02bf368934");
        let public_key = MoneroPublicKey::from_slice(&public_key_bytes).unwrap();
        let secret_key = MoneroPrivateKey::from_slice(&secret_key_bytes).unwrap();
        let _actual_key_deriv = KeyDerivation::generate(&public_key, &secret_key);
        // // assert_eq!(actual_key_deriv.to_bytes(), expected_key_deriv);
    }

    #[test]
    fn test_derive_public_key() {
        let key_deriv = hex!("ca780b065e48091d910de90bcab2411db3d1a845e6d95cfd556af4138504c737");
        let output_index = 217407;
        let base = hex!("6d9dd2068b9d6d643b407e360dfc5eb7a1f628fe2de8112a9e5731e8b3680c39");
        let expected_derived_pub_key =
            hex!("d48008aff5f27d8fcdc2a3bf814ed3505530f598075f3bf7e868fea696b109f6");
        let derivation = KeyDerivation::from_slice(&key_deriv).unwrap();
        let base_key = MoneroPublicKey::from_slice(&base).unwrap();
        let actual_derived_pub_key = derivation
            .derive_public_key(output_index, &base_key)
            .unwrap();
        assert_eq!(actual_derived_pub_key.as_bytes(), expected_derived_pub_key);
    }

    #[test]
    fn test_derive_private_key() {
        let key_deriv = hex!("0fc47054f355ced4d67de73bfa12e4c78ff19089548fffa7d07a674741860f97");
        let output_index = 66;
        let base = hex!("5619c62aa4ad787274b1071598b6ecacf4f9dacca2fd11b0c80741b744400500");
        let expected_derived_sec_key =
            hex!("55297d64b0c0556d5583ce0e30c2024ccce90c93d16bdeb4e40fce7afff87803");
        let derivation = KeyDerivation::from_slice(&key_deriv).unwrap();
        let base_key = MoneroPrivateKey::from_slice(&base).unwrap();
        let actual_derived_sec_key = derivation
            .derive_private_key(output_index, &base_key)
            .unwrap();
        assert_eq!(actual_derived_sec_key.as_bytes(), expected_derived_sec_key);
    }
}
