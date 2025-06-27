#![allow(async_fn_in_trait)]
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DIDError {
    #[error("Invalid DID format")]
    InvalidFormat,
    #[error("Principal not found")]
    PrincipalNotFound,
    #[error("Registration failed")]
    RegistrationFailed,
    #[error("Agent error: {0}")]
    Agent(String),
}

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct DIDDocument {
    pub id: String,
    pub principal: Principal,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct IcpDID {
    principal: Principal,
    document: DIDDocument,
}

impl IcpDID {
    pub fn create(principal: Principal) -> Result<Self, DIDError> {
        let document = DIDDocument {
            id: format!("did:icp:{}", principal.to_text()),
            principal,
            public_key: vec![],
        };
        
        Ok(Self {
            principal,
            document,
        })
    }

    pub fn document(&self) -> &DIDDocument {
        &self.document
    }

    pub async fn resolve(did: &str) -> Result<DIDDocument, DIDError> {
        // Extract principal from DID
        let parts: Vec<&str> = did.split(':').collect();
        if parts.len() != 3 || parts[0] != "did" || parts[1] != "icp" {
            return Err(DIDError::InvalidFormat);
        }
        
        let principal = Principal::from_text(parts[2])
            .map_err(|_| DIDError::InvalidFormat)?;
        
        // In production, this would fetch from chain
        let document = DIDDocument {
            id: did.to_string(),
            principal,
            public_key: vec![],
        };
        
        Ok(document)
    }
}

// The rest of the file remains the same...


impl std::fmt::Display for IcpDID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "did:icp:{}", self.principal)
    }
}
