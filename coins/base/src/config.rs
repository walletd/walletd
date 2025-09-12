use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub chain_id: u64,
    pub name: String,
    pub currency_symbol: String,
    pub decimals: u8,
    pub block_time_ms: u64,
    pub rpc_endpoints: Vec<String>,
    pub explorer: String,
}

pub const BASE_MAINNET: NetworkConfig = NetworkConfig {
    chain_id: 8453,
    name: String::new(), // Will be initialized properly
    currency_symbol: String::new(),
    decimals: 18,
    block_time_ms: 2000,
    rpc_endpoints: Vec::new(),
    explorer: String::new(),
};

pub const BASE_SEPOLIA: NetworkConfig = NetworkConfig {
    chain_id: 84532,
    name: String::new(),
    currency_symbol: String::new(),
    decimals: 18,
    block_time_ms: 2000,
    rpc_endpoints: Vec::new(),
    explorer: String::new(),
};

impl NetworkConfig {
    pub fn mainnet() -> Self {
        NetworkConfig {
            chain_id: 8453,
            name: "Base Mainnet".to_string(),
            currency_symbol: "ETH".to_string(),
            decimals: 18,
            block_time_ms: 2000,
            rpc_endpoints: vec![
                "https://mainnet.base.org".to_string(),
                "https://base.publicnode.com".to_string(),
                "https://rpc.ankr.com/base".to_string(),
            ],
            explorer: "https://basescan.org".to_string(),
        }
    }

    pub fn sepolia() -> Self {
        NetworkConfig {
            chain_id: 84532,
            name: "Base Sepolia".to_string(),
            currency_symbol: "ETH".to_string(),
            decimals: 18,
            block_time_ms: 2000,
            rpc_endpoints: vec![
                "https://sepolia.base.org".to_string(),
                "https://base-sepolia.publicnode.com".to_string(),
            ],
            explorer: "https://sepolia.basescan.org".to_string(),
        }
    }
}
