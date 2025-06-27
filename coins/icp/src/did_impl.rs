use candid::Principal;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    pub id: String,
    pub principal: Principal,
    pub public_key: Vec<u8>,
    pub authentication: Vec<String>,
    pub created: u64,
    pub updated: u64,
}

impl IcpDID {
    pub fn create_did_document(
        principal: Principal,
        public_key: Vec<u8>,
    ) -> Result<DIDDocument, DIDError> {
        let did_id = format!("did:icp:{}", principal.to_text());
        
        let doc = DIDDocument {
            id: did_id.clone(),
            principal,
            public_key: public_key.clone(),
            authentication: vec![format!("{}#key-1", did_id)],
            created: ic_cdk::api::time(),
            updated: ic_cdk::api::time(),
        };
        
        Ok(doc)
    }
    
    pub async fn register_did_on_chain(
        &self,
        document: &DIDDocument,
        agent: &Agent,
    ) -> Result<(), DIDError> {
        // In a real implementation, this would call a DID registry canister
        let did_registry = Principal::from_text("suaf3-hqaaa-aaaaf-qaaya-cai").unwrap();
        
        #[derive(CandidType, Serialize)]
        struct RegisterDIDArgs {
            document: String,
        }
        
        let args = RegisterDIDArgs {
            document: serde_json::to_string(document)
                .map_err(|e| DIDError::Serialization(e.to_string()))?,
        };
        
        agent
            .update(&did_registry, "register_did")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| DIDError::Registration(e.to_string()))?;
        
        Ok(())
    }
    
    pub async fn resolve_did(
        did: &str,
        agent: &Agent,
    ) -> Result<DIDDocument, DIDError> {
        if !did.starts_with("did:icp:") {
            return Err(DIDError::InvalidFormat);
        }
        
        let did_registry = Principal::from_text("suaf3-hqaaa-aaaaf-qaaya-cai").unwrap();
        
        #[derive(CandidType, Serialize)]
        struct ResolveDIDArgs {
            did: String,
        }
        
        let args = ResolveDIDArgs {
            did: did.to_string(),
        };
        
        let result = agent
            .query(&did_registry, "resolve_did")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| DIDError::Resolution(e.to_string()))?;
        
        let doc_string: String = candid::decode_one(&result)
            .map_err(|e| DIDError::Decode(e.to_string()))?;
        
        let document: DIDDocument = serde_json::from_str(&doc_string)
            .map_err(|e| DIDError::Deserialization(e.to_string()))?;
        
        Ok(document)
    }
}
