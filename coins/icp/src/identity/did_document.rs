use anyhow::Result;
use candid::Principal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    pub controller: String,
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    pub service: Vec<Service>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub method_type: String,
    pub controller: String,
    #[serde(rename = "publicKeyBase58")]
    pub public_key_base58: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

impl DIDDocument {
    pub fn new(did: &str, principal: Principal) -> Result<Self> {
        Ok(Self {
            context: vec![
                "https://www.w3.org/ns/did/v1".to_string(),
                "https://w3id.org/security/v1".to_string(),
            ],
            id: did.to_string(),
            controller: principal.to_string(),
            verification_method: vec![],
            authentication: vec![],
            service: vec![],
        })
    }

    pub fn add_verification_method(&mut self, key: &super::VerificationKey) {
        let method = VerificationMethod {
            id: format!("{}#{}", self.id, key.id),
            method_type: key.key_type.clone(),
            controller: self.controller.clone(),
            public_key_base58: key.public_key_base58.clone(),
        };

        self.verification_method.push(method.clone());
        self.authentication.push(method.id);
    }
}
