use anyhow::Result;
use candid::Principal;
use serde::{Deserialize, Serialize};

pub mod authentication;
pub mod did_document;
pub mod resolver;

pub use authentication::DIDAuthentication;
pub use did_document::DIDDocument;
pub use resolver::DIDResolver;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizedIdentity {
    pub did: String,
    pub document: DIDDocument,
    pub keys: Vec<VerificationKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationKey {
    pub id: String,
    pub key_type: String,
    pub controller: String,
    pub public_key_base58: String,
}

impl DecentralizedIdentity {
    pub fn create(principal: Principal) -> Result<Self> {
        let did = format!("did:icp:{principal}");
        let document = DIDDocument::new(&did, principal)?;

        Ok(Self {
            did,
            document,
            keys: vec![],
        })
    }

    pub fn add_verification_key(&mut self, key: VerificationKey) {
        self.keys.push(key.clone());
        self.document.add_verification_method(&key);
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.document)?)
    }
}
