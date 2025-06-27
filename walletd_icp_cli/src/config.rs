//! Configuration for blockchain connections

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletDConfig {
    pub bitcoin: BitcoinConfig,
    pub ethereum: EthereumConfig,
    pub solana: SolanaConfig,
    pub monero: MoneroConfig,
    pub hedera: HederaConfig,
    pub icp: IcpConfig,
    pub demo_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    pub network: String, // "mainnet", "testnet", "regtest"
    pub rpc_url: String,
    pub rpc_user: Option<String>,
    pub rpc_password: Option<String>,
    pub electrum_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub etherscan_api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    pub cluster: String, // "mainnet-beta", "testnet", "devnet"
    pub rpc_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneroConfig {
    pub network: String,
    pub daemon_url: String,
    pub wallet_rpc_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HederaConfig {
    pub network: String, // "mainnet", "testnet"
    pub operator_id: String,
    pub operator_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcpConfig {
    pub network: String, // "ic", "local"
    pub identity_path: Option<String>,
}

impl Default for WalletDConfig {
    fn default() -> Self {
        Self {
            bitcoin: BitcoinConfig {
                network: "testnet".to_string(),
                rpc_url: "http://localhost:18332".to_string(),
                rpc_user: None,
                rpc_password: None,
                electrum_url: Some("ssl://electrum.blockstream.info:60002".to_string()),
            },
            ethereum: EthereumConfig {
                chain_id: 5, // Goerli
                rpc_url: "https://eth-goerli.g.alchemy.com/v2/demo".to_string(),
                etherscan_api_key: None,
            },
            solana: SolanaConfig {
                cluster: "devnet".to_string(),
                rpc_url: "https://api.devnet.solana.com".to_string(),
            },
            monero: MoneroConfig {
                network: "testnet".to_string(),
                daemon_url: "http://localhost:28081".to_string(),
                wallet_rpc_url: None,
            },
            hedera: HederaConfig {
                network: "testnet".to_string(),
                operator_id: "0.0.0".to_string(),
                operator_key: "".to_string(),
            },
            icp: IcpConfig {
                network: "local".to_string(),
                identity_path: None,
            },
            demo_mode: false,
        }
    }
}

impl WalletDConfig {
    /// Load config from file or create default
    pub fn load() -> Self {
        if let Ok(config_str) = std::fs::read_to_string("walletd_config.json") {
            serde_json::from_str(&config_str).unwrap_or_default()
        } else {
            let config = Self::default();
            let _ = config.save();
            config
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_str = serde_json::to_string_pretty(self)?;
        std::fs::write("walletd_config.json", config_str)?;
        Ok(())
    }
}
