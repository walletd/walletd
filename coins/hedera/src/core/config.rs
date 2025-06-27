// src/core/config.rs

use dotenvy::dotenv;
use std::env;
use std::time::Duration;

use crate::core::errors::WalletDError;

#[derive(Debug)]
pub struct HederaConfig {
    pub operator_id: String,
    pub operator_private_key: String,
    pub hedera_network: String,
    pub hedera_network_nodes: Option<Vec<(String, String)>>,
    pub hedera_request_timeout: Option<Duration>,
    pub hedera_max_attempts: Option<usize>,
}

impl HederaConfig {
    pub fn load() -> Result<Self, WalletDError> {
        dotenv().ok();

        let operator_id = env::var("HEDERA_OPERATOR_ID")
            .map_err(|_| WalletDError::ConfigError("OPERATOR_ID not set".to_string()))?;
        let operator_private_key = env::var("OPERATOR_PRIVATE_KEY")
            .map_err(|_| WalletDError::ConfigError("OPERATOR_PRIVATE_KEY not set".to_string()))?;
        let hedera_network = env::var("HEDERA_NETWORK").unwrap_or_else(|_| "testnet".to_string());

        // Parse optional configurations
        let hedera_request_timeout = env::var("HEDERA_REQUEST_TIMEOUT")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);
        let hedera_max_attempts = env::var("HEDERA_MAX_ATTEMPTS")
            .ok()
            .and_then(|s| s.parse().ok());

        // Parse network nodes if provided
        let hedera_network_nodes = env::var("HEDERA_NETWORK_NODES").ok().map(|nodes_str| {
            nodes_str
                .split(';')
                .filter_map(|node_str| {
                    let parts: Vec<&str> = node_str.split(',').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(String, String)>>()
        });

        Ok(HederaConfig {
            operator_id,
            operator_private_key,
            hedera_network,
            hedera_network_nodes,
            hedera_request_timeout,
            hedera_max_attempts,
        })
    }
}
