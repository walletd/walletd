#![allow(async_fn_in_trait)]
#![allow(async_fn_in_trait)]// coins/icp/src/canister.rs
use candid::{CandidType, Deserialize, Principal, encode_args, decode_args};
use ic_cdk::api::call::{call_raw};
use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error)]
pub enum CanisterError {
    #[error("Call failed: {0}")]
    CallFailed(String),
    #[error("Encoding error: {0}")]
    EncodingError(String),
    #[error("Decoding error: {0}")]
    DecodingError(String),
    #[error("Invalid canister ID")]
    InvalidCanisterId,
}

/// Generic canister client for interacting with any ICP canister
pub struct CanisterClient {
    canister_id: Principal,
}

impl CanisterClient {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }
    
    /// Call a canister method with typed arguments and return value
    pub async fn call<T, R>(&self, method: &str, args: T) -> Result<R, CanisterError>
    where
        T: CandidType,
        R: CandidType + for<'de> Deserialize<'de>,
    {
        let encoded_args = encode_args((args,))
            .map_err(|e| CanisterError::EncodingError(e.to_string()))?;
        
        let result = call_raw(self.canister_id, method, encoded_args, 0).await
            .map_err(|(code, msg)| CanisterError::CallFailed(
                format!("Error {}: {}", code as u8, msg)
            ))?;
        
        let (response,): (R,) = decode_args(&result)
            .map_err(|e| CanisterError::DecodingError(e.to_string()))?;
        
        Ok(response)
    }
    
    /// Query a canister method (read-only)
    pub async fn query<T, R>(&self, method: &str, args: T) -> Result<R, CanisterError>
    where
        T: CandidType,
        R: CandidType + for<'de> Deserialize<'de>,
    {
        // For queries, we use the same call mechanism
        // In production, you might want to use query calls for better performance
        self.call(method, args).await
    }
}

/// Common canister interfaces

/// Token standard interface (DIP20/ICRC1)
#[derive(CandidType, Deserialize, Serialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
}

pub trait TokenCanister {
    async fn name(&self) -> Result<String, CanisterError>;
    async fn symbol(&self) -> Result<String, CanisterError>;
    async fn decimals(&self) -> Result<u8, CanisterError>;
    async fn total_supply(&self) -> Result<u64, CanisterError>;
    async fn balance_of(&self, owner: Principal) -> Result<u64, CanisterError>;
    async fn transfer(&self, to: Principal, amount: u64) -> Result<bool, CanisterError>;
}

/// NFT standard interface
#[derive(CandidType, Deserialize, Serialize)]
pub struct NFTMetadata {
    pub token_id: u64,
    pub owner: Principal,
    pub metadata: Vec<u8>,
}

pub trait NFTCanister {
    async fn mint(&self, to: Principal, metadata: Vec<u8>) -> Result<u64, CanisterError>;
    async fn transfer(&self, to: Principal, token_id: u64) -> Result<bool, CanisterError>;
    async fn get_metadata(&self, token_id: u64) -> Result<NFTMetadata, CanisterError>;
    async fn balance_of(&self, owner: Principal) -> Result<u64, CanisterError>;
}

/// DAO/Governance canister interface
#[derive(CandidType, Deserialize, Serialize)]
pub struct Proposal {
    pub id: u64,
    pub proposer: Principal,
    pub title: String,
    pub description: String,
    pub votes_yes: u64,
    pub votes_no: u64,
    pub status: ProposalStatus,
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum ProposalStatus {
    Open,
    Accepted,
    Rejected,
    Executed,
}

pub trait GovernanceCanister {
    async fn create_proposal(&self, title: String, description: String) -> Result<u64, CanisterError>;
    async fn vote(&self, proposal_id: u64, vote: bool) -> Result<(), CanisterError>;
    async fn get_proposal(&self, proposal_id: u64) -> Result<Proposal, CanisterError>;
    async fn execute_proposal(&self, proposal_id: u64) -> Result<(), CanisterError>;
}

/// Example implementation for a token canister
pub struct ICPTokenCanister {
    client: CanisterClient,
}

impl ICPTokenCanister {
    pub fn new(canister_id: Principal) -> Self {
        Self {
            client: CanisterClient::new(canister_id),
        }
    }
}

impl TokenCanister for ICPTokenCanister {
    async fn name(&self) -> Result<String, CanisterError> {
        self.client.query("name", ()).await
    }
    
    async fn symbol(&self) -> Result<String, CanisterError> {
        self.client.query("symbol", ()).await
    }
    
    async fn decimals(&self) -> Result<u8, CanisterError> {
        self.client.query("decimals", ()).await
    }
    
    async fn total_supply(&self) -> Result<u64, CanisterError> {
        self.client.query("totalSupply", ()).await
    }
    
    async fn balance_of(&self, owner: Principal) -> Result<u64, CanisterError> {
        self.client.query("balanceOf", (owner,)).await
    }
    
    async fn transfer(&self, to: Principal, amount: u64) -> Result<bool, CanisterError> {
        self.client.call("transfer", (to, amount)).await
    }
}

/// Helper to create typed canister clients
#[macro_export]
macro_rules! create_canister_client {
    ($name:ident, $canister_id:expr, {
        $(
            $(#[$attr:meta])*
            async fn $method:ident($($arg:ident: $arg_ty:ty),*) -> Result<$ret:ty, CanisterError>;
        )*
    }) => {
        pub struct $name {
            client: CanisterClient,
        }
        
        impl $name {
            pub fn new() -> Self {
                Self {
                    client: CanisterClient::new($canister_id),
                }
            }
            
            $(
                $(#[$attr])*
                pub async fn $method(&self, $($arg: $arg_ty),*) -> Result<$ret, CanisterError> {
                    self.client.call(stringify!($method), ($($arg,)*)).await
                }
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canister_client_creation() {
        let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let client = CanisterClient::new(canister_id);
        assert_eq!(client.canister_id, canister_id);
    }
}