use crate::{CanisterClient, CanisterError};
use candid::Principal;
use ic_agent::{Agent, Identity};

/// Network type for canister connections
#[derive(Debug, Clone)]
pub enum Network {
    Local,
    Mainnet,
    Testnet,
    Custom(String),
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::Local => "http://localhost:8000",
            Network::Mainnet => "https://ic0.app",
            Network::Testnet => "https://testnet.dfinity.network",
            Network::Custom(url) => url,
        }
    }
    
    pub fn should_fetch_root_key(&self) -> bool {
        matches!(self, Network::Local | Network::Testnet)
    }
}

/// Builder for easy canister client creation
pub struct CanisterClientBuilder {
    canister_id: Option<Principal>,
    network: Network,
    identity: Option<Box<dyn Identity>>,
    timeout: Option<std::time::Duration>,
}

impl CanisterClientBuilder {
    pub fn new() -> Self {
        Self {
            canister_id: None,
            network: Network::Mainnet,
            identity: None,
            timeout: None,
        }
    }
    
    /// Set the canister ID
    pub fn with_canister(mut self, canister_id: &str) -> Result<Self, CanisterError> {
        self.canister_id = Some(
            Principal::from_text(canister_id)
                .map_err(|e| CanisterError::InvalidCanisterId(e.to_string()))?
        );
        Ok(self)
    }
    
    /// Use local replica
    pub fn with_local_replica(mut self) -> Self {
        self.network = Network::Local;
        self
    }
    
    /// Use mainnet
    pub fn with_mainnet(mut self) -> Self {
        self.network = Network::Mainnet;
        self
    }
    
    /// Use custom network
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }
    
    /// Set identity for authentication
    pub fn with_identity(mut self, identity: impl Identity + 'static) -> Self {
        self.identity = Some(Box::new(identity));
        self
    }
    
    /// Set request timeout
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    /// Build the canister client
    pub async fn build(self) -> Result<CanisterClient, CanisterError> {
        let mut agent_builder = Agent::builder()
            .with_url(self.network.url());
            
        if let Some(identity) = self.identity {
            agent_builder = agent_builder.with_identity(identity);
        }
        
        if let Some(timeout) = self.timeout {
            agent_builder = agent_builder.with_ingress_expiry(Some(timeout));
        }
        
        let agent = agent_builder.build()
            .map_err(|e| CanisterError::AgentError(e))?;
            
        if self.network.should_fetch_root_key() {
            agent.fetch_root_key().await
                .map_err(|e| CanisterError::AgentError(e))?;
        }
        
        let canister_id = self.canister_id
            .ok_or_else(|| CanisterError::InvalidCanisterId("Canister ID not set".to_string()))?;
            
        Ok(CanisterClient::new(agent, canister_id))
    }
}

impl Default for CanisterClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
