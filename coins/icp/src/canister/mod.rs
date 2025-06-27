#![allow(async_fn_in_trait)]
pub mod builder;
pub mod canisters;
pub mod testing;

use candid::{CandidType, Principal, encode_args, decode_args, Deserialize};
use ic_agent::{Agent, AgentError};
use thiserror::Error;

pub use builder::{CanisterClientBuilder, Network};
pub use testing::{MockCanister, TestEnvironment};

#[derive(Debug, Error)]
pub enum CanisterError {
    #[error("Agent error: {0}")]
    AgentError(#[from] AgentError),
    
    #[error("Invalid canister ID: {0}")]
    InvalidCanisterId(String),
    
    #[error("Candid error: {0}")]
    CandidError(#[from] candid::Error),
    
    #[error("Call failed: {0}")]
    CallFailed(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Missing replica transport")]
    MissingReplicaTransport,
}

/// Main canister client for interacting with ICP canisters
pub struct CanisterClient {
    pub(crate) agent: Agent,
    pub(crate) canister_id: Principal,
}

impl CanisterClient {
    /// Create a new canister client
    pub fn new(agent: Agent, canister_id: Principal) -> Self {
        Self { agent, canister_id }
    }
    
    /// Create a builder for easier configuration
    pub fn builder() -> CanisterClientBuilder {
        CanisterClientBuilder::new()
    }
    
    /// Quick connection to local canister
    pub async fn local(canister_id: &str) -> Result<Self, CanisterError> {
        Self::builder()
            .with_canister(canister_id)?
            .with_local_replica()
            .build()
            .await
    }
    
    /// Quick connection to mainnet canister
    pub async fn mainnet(canister_id: &str) -> Result<Self, CanisterError> {
        Self::builder()
            .with_canister(canister_id)?
            .with_mainnet()
            .build()
            .await
    }
    
    /// Get the canister ID
    pub fn canister_id(&self) -> &Principal {
        &self.canister_id
    }
    
    /// Get the agent
    pub fn agent(&self) -> &Agent {
        &self.agent
    }
    
    /// Perform a query call (read-only)
    pub async fn query(&self, method: &str, args: &[u8]) -> Result<Vec<u8>, CanisterError> {
        self.agent
            .query(&self.canister_id, method)
            .with_arg(args)
            .call()
            .await
            .map_err(|e| CanisterError::CallFailed(e.to_string()))
    }
    
    /// Perform an update call (state-changing)
    pub async fn update(&self, method: &str, args: &[u8]) -> Result<Vec<u8>, CanisterError> {
        self.agent
            .update(&self.canister_id, method)
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| CanisterError::CallFailed(e.to_string()))
    }
    
    /// Perform a query call with Candid encoding
    pub async fn call(&self, method: &str, args: &impl CandidType) -> Result<Vec<u8>, CanisterError> {
        let encoded_args = encode_args((args,))?;
        self.query(method, &encoded_args).await
    }
    
    /// Type-safe query
    pub async fn query_typed<T: CandidType + for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        args: &impl CandidType,
    ) -> Result<T, CanisterError> {
        let encoded_args = encode_args((args,))?;
        let bytes = self.query(method, &encoded_args).await?;
        let (result,): (T,) = decode_args(&bytes)?;
        Ok(result)
    }
    
    /// Type-safe update
    pub async fn update_typed<T: CandidType + for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        args: &impl CandidType,
    ) -> Result<T, CanisterError> {
        let encoded_args = encode_args((args,))?;
        let bytes = self.update(method, &encoded_args).await?;
        let (result,): (T,) = decode_args(&bytes)?;
        Ok(result)
    }
}

/// Extension trait for wallet integration
pub trait WalletCanisterExt {
    /// Create a canister client from wallet
    fn create_canister_client(&self, canister_id: &str) -> impl std::future::Future<Output = Result<CanisterClient, CanisterError>> + Send;
}

// Re-export commonly used types
pub use ic_agent::Identity;
