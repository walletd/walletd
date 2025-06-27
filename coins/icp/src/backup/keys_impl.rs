use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use sha2::{Sha256, Digest};

impl IcpKeyManager {
    pub fn sign_transaction(
        &self,
        private_key: &[u8],
        message: &[u8],
    ) -> Result<Vec<u8>, KeyError> {
        if private_key.len() != 32 {
            return Err(KeyError::InvalidKey);
        }
        
        let secret = SecretKey::from_bytes(private_key)
            .map_err(|_| KeyError::InvalidKey)?;
        let public = PublicKey::from(&secret);
        let keypair = Keypair {
            secret,
            public,
        };
        
        let signature: Signature = keypair.sign(message);
        Ok(signature.to_bytes().to_vec())
    }
    
    pub fn verify_signature(
        &self,
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, KeyError> {
        let public = PublicKey::from_bytes(public_key)
            .map_err(|_| KeyError::InvalidKey)?;
        
        let sig = Signature::from_bytes(signature)
            .map_err(|_| KeyError::InvalidSignature)?;
        
        Ok(public.verify(message, &sig).is_ok())
    }
}
