use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Network {
    /// Production mainnet
    Mainnet,
    /// Public testnet
    Testnet,
    /// Local mock network for development
    Mocknet,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub network: Network,
    pub endpoints: Vec<String>,
    pub chain_id: u32,
    pub native_symbol: String,
}

impl NetworkConfig {
    pub fn mainnet() -> Self {
        Self {
            network: Network::Mainnet,
            endpoints: vec![
                "https://api.prasaga.com".to_string(),
                "https://rpc.prasaga.com".to_string(),
            ],
            chain_id: 1, // Will be updated with actual chain ID
            native_symbol: "SAGA".to_string(),
        }
    }

    pub fn testnet() -> Self {
        Self {
            network: Network::Testnet,
            endpoints: vec![
                "https://testnet-api.prasaga.com".to_string(),
                "https://testnet-rpc.prasaga.com".to_string(),
            ],
            chain_id: 9000, // Assumed testnet chain ID
            native_symbol: "tSAGA".to_string(),
        }
    }

    pub fn mocknet() -> Self {
        Self {
            network: Network::Mocknet,
            endpoints: vec![
                "http://localhost:8545".to_string(),
                "http://127.0.0.1:8545".to_string(),
            ],
            chain_id: 31337, // Local dev chain ID
            native_symbol: "mSAGA".to_string(),
        }
    }

    pub fn from_network(network: Network) -> Self {
        match network {
            Network::Mainnet => Self::mainnet(),
            Network::Testnet => Self::testnet(),
            Network::Mocknet => Self::mocknet(),
        }
    }

    pub fn explorer_url(&self) -> Option<String> {
        match self.network {
            Network::Mainnet => Some("https://sagascan.prasaga.com".to_string()),
            Network::Testnet => Some("https://testnet.sagascan.prasaga.com".to_string()),
            Network::Mocknet => None,
        }
    }

    pub fn faucet_url(&self) -> Option<String> {
        match self.network {
            Network::Mainnet => None,
            Network::Testnet => Some("https://faucet.prasaga.com".to_string()),
            Network::Mocknet => Some("http://localhost:8080/faucet".to_string()),
        }
    }
}
