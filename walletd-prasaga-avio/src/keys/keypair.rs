use bip39::Mnemonic;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use std::str::FromStr;

use crate::types::{Error, Result};

#[derive(Debug, Clone)]
pub struct PrasagaAvioKeypair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    derivation_path: Option<String>,
}

impl PrasagaAvioKeypair {
    /// Create keypair from seed phrase
    pub fn from_mnemonic(mnemonic: &str, passphrase: &str, path: &str) -> Result<Self> {
        let mnemonic = Mnemonic::from_str(mnemonic)
            .map_err(|e| Error::Crypto(format!("Invalid mnemonic: {e}")))?;

        // Convert mnemonic to seed (64 bytes)
        let seed = mnemonic.to_seed(passphrase);
        Self::from_seed(&seed, path)
    }

    /// Create keypair from seed bytes
    pub fn from_seed(seed: &[u8], path: &str) -> Result<Self> {
        // For now, we'll use a simple derivation
        // In production, this should use proper BIP32 derivation
        let key_material = if seed.len() == 32 {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(seed);
            bytes
        } else if seed.len() >= 32 {
            // Take first 32 bytes for ed25519
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(&seed[..32]);
            bytes
        } else {
            // Hash the seed to get 32 bytes
            let hash = blake3::hash(seed);
            *hash.as_bytes()
        };

        let signing_key = SigningKey::from_bytes(&key_material);
        let verifying_key = signing_key.verifying_key();

        Ok(Self {
            signing_key,
            verifying_key,
            derivation_path: Some(path.to_string()),
        })
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.signing_key.sign(message);
        signature.to_bytes().to_vec()
    }

    /// Get public key bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.verifying_key.to_bytes().to_vec()
    }

    /// Export private key (be careful!)
    pub fn private_key_bytes(&self) -> Vec<u8> {
        self.signing_key.to_bytes().to_vec()
    }

    /// Get the derivation path
    pub fn derivation_path(&self) -> Option<&str> {
        self.derivation_path.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let seed = b"test seed for prasaga avio chain integration!!!";
        let keypair = PrasagaAvioKeypair::from_seed(seed, "m/44'/9000'/0'/0/0").unwrap();
        assert_eq!(keypair.public_key_bytes().len(), 32);
    }

    #[test]
    fn test_signature() {
        let seed = b"test seed for prasaga avio chain integration!!!";
        let keypair = PrasagaAvioKeypair::from_seed(seed, "m/44'/9000'/0'/0/0").unwrap();
        let message = b"Hello PraSaga!";
        let signature = keypair.sign(message);
        assert_eq!(signature.len(), 64);
    }

    #[test]
    fn test_from_mnemonic() {
        let mnemonic = "test test test test test test test test test test test junk";
        let keypair =
            PrasagaAvioKeypair::from_mnemonic(mnemonic, "", "m/44'/9000'/0'/0/0").unwrap();
        assert_eq!(keypair.public_key_bytes().len(), 32);
    }
}
