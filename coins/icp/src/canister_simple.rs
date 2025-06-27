use thiserror::Error;
use candid::Principal;
use ic_agent::Agent;
use serde::Deserialize;

pub mod canisters;

#[derive(Debug, Error)]
pub enum CanisterError {
    #[error("Agent error: {0}")]
    Agent(String),
    #[error("Call failed: {0}")]
    CallFailed(String),
}

pub struct CanisterClient {
    agent: Agent,
    canister_id: Principal,
}

impl CanisterClient {
    pub fn new(agent: Agent, canister_id: Principal) -> Self {
        Self { agent, canister_id }
    }

    pub async fn call<T>(&self, _method: &str, _args: &[u8]) -> Result<T, CanisterError> 
    where T: for<'de> Deserialize<'de> {
        // Implementation placeholder
        Err(CanisterError::CallFailed("Not implemented".to_string()))
    }
}

pub trait SmartContract {
    fn deploy(&self) -> impl std::future::Future<Output = Result<Principal, CanisterError>> + Send;
    fn upgrade(&self, wasm: Vec<u8>) -> impl std::future::Future<Output = Result<(), CanisterError>> + Send;
}
