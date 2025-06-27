use anyhow::Result;
use candid::{CandidType, Decode, Encode, Principal};
use ic_agent::Agent;
use serde::Deserialize;

pub mod canister_client;
pub mod deployment;
pub mod interface;

pub use canister_client::CanisterClient;
pub use deployment::CanisterDeployment;
pub use interface::CanisterInterface;

#[derive(Debug, Clone)]
pub struct SmartContract {
    pub canister_id: Principal,
    pub agent: Agent,
    pub interface: CanisterInterface,
}

impl SmartContract {
    pub fn new(canister_id: Principal, agent: Agent) -> Self {
        Self {
            canister_id,
            agent,
            interface: CanisterInterface::new(),
        }
    }

    pub async fn call<T: CandidType>(&self, method: &str, args: T) -> Result<Vec<u8>> {
        let encoded_args = Encode!(&args)?;
        let response = self
            .agent
            .update(&self.canister_id, method)
            .with_arg(encoded_args)
            .call_and_wait()
            .await?;
        Ok(response)
    }

    pub async fn query<T: CandidType, R: CandidType + for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        args: T,
    ) -> Result<R> {
        let encoded_args = Encode!(&args)?;
        let response = self
            .agent
            .query(&self.canister_id, method)
            .with_arg(encoded_args)
            .call()
            .await?;
        let decoded = Decode!(&response, R)?;
        Ok(decoded)
    }
}
