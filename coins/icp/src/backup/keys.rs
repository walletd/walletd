// coins/icp/src/keys.rs
use ic_cdk::api::management_canister::ecdsa::{
    EcdsaKeyId, EcdsaPublicKeyArgument, EcdsaPublicKeyResponse,
    SignWithEcdsaArgument, SignWithEcdsaResponse, EcdsaCurve,
};
use candid::Principal;
use sha2::{Sha256, Digest};
use thiserror::Error;
use walletd_hd_key::{HDKey, HDNetworkType};

#[derive(Debug, Error)]
pub enum KeyError {
    #[error("ECDSA error: {0}")]
    Ecdsa(String),
    #[error("Invalid derivation path")]
    InvalidDerivationPath,
    #[error("HD key error: {0}")]
    HdKey(#[from] walletd_hd_key::Error),
}

pub struct IcpKeyManager {
    key_id: EcdsaKeyId,
}

impl IcpKeyManager {
    pub fn new(network: HDNetworkType) -> Self {
        let key_name = match network {
            HDNetworkType::MainNet => "key_1",
            HDNetworkType::TestNet => "test_key_1",
        };
        
        Self {
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name.to_string(),
            },
        }
    }
    
    /// Derive a public key for a specific derivation path
    pub async fn derive_public_key(
        &self,
        derivation_path: Vec<Vec<u8>>,
        canister_id: Option<Principal>,
    ) -> Result<Vec<u8>, KeyError> {
        let request = EcdsaPublicKeyArgument {
            canister_id,
            derivation_path,
            key_id: self.key_id.clone(),
        };
        
        let (response,): (EcdsaPublicKeyResponse,) = 
            ic_cdk::call(Principal::management_canister(), "ecdsa_public_key", (request,))
                .await
                .map_err(|e| KeyError::Ecdsa(format!("{:?}", e)))?;
        
        Ok(response.public_key)
    }
    
    /// Sign a message with ECDSA
    pub async fn sign_with_ecdsa(
        &self,
        message_hash: Vec<u8>,
        derivation_path: Vec<Vec<u8>>,
    ) -> Result<Vec<u8>, KeyError> {
        let request = SignWithEcdsaArgument {
            message_hash,
            derivation_path,
            key_id: self.key_id.clone(),
        };
        
        let (response,): (SignWithEcdsaResponse,) = 
            ic_cdk::call(Principal::management_canister(), "sign_with_ecdsa", (request,))
                .await
                .map_err(|e| KeyError::Ecdsa(format!("{:?}", e)))?;
        
        Ok(response.signature)
    }
    
    /// Create ICP Principal from HD key
    pub fn principal_from_hd_key(hd_key: &HDKey) -> Result<Principal, KeyError> {
        let private_key = hd_key.extended_private_key()?;
        let public_key = private_key.to_public_key();
        
        // ICP uses SHA-224 hash of the public key for Principal
        let mut hasher = Sha256::new();
        hasher.update(b"\x0Aaccount-id");
        hasher.update(&public_key.to_bytes());
        hasher.update(&[0u8; 32]); // Subaccount
        let hash = hasher.finalize();
        
        // Take first 28 bytes and add checksum
        let mut principal_bytes = vec![0x00]; // Version byte
        principal_bytes.extend_from_slice(&hash[..28]);
        
        Ok(Principal::from_slice(&principal_bytes))
    }
    
    /// Get derivation path for account
    pub fn get_derivation_path(account: u32) -> Vec<Vec<u8>> {
        vec![
            b"m".to_vec(),
            b"44'".to_vec(),
            b"223'".to_vec(), // ICP coin type
            b"0'".to_vec(),
            b"0".to_vec(),
            account.to_string().as_bytes().to_vec(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_derivation_path() {
        let path = IcpKeyManager::get_derivation_path(0);
        assert_eq!(path.len(), 6);
        assert_eq!(path[0], b"m");
        assert_eq!(path[1], b"44'");
        assert_eq!(path[2], b"223'");
    }
}