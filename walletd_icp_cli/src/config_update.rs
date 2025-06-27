// Add to your existing config or create new file

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletDConfig {
    #[serde(default = "default_mode")]
    pub mode: WalletMode,
    
    #[serde(default = "default_demo_mode")]
    pub demo_mode: bool,  // Keep for backward compatibility
    
    pub bitcoin: BitcoinConfig,
    pub ethereum: EthereumConfig,
    pub solana: SolanaConfig,
    pub monero: MoneroConfig,
    pub hedera: HederaConfig,
    pub icp: IcpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WalletMode {
    Demo,
    Testnet,
    Mainnet,
}

fn default_mode() -> WalletMode {
    WalletMode::Testnet  // Default to testnet for safety!
}

fn default_demo_mode() -> bool {
    false
}

impl WalletDConfig {
    pub fn load() -> Self {
        let config_path = "walletd_config.json";
        
        if Path::new(config_path).exists() {
            let contents = fs::read_to_string(config_path)
                .expect("Failed to read config file");
            
            let mut config: Self = serde_json::from_str(&contents)
                .expect("Failed to parse config file");
            
            // Handle backward compatibility
            if config.demo_mode {
                config.mode = WalletMode::Demo;
            }
            
            config
        } else {
            // Create default testnet config
            let default_config = Self::default_testnet();
            
            // Save it
            let json = serde_json::to_string_pretty(&default_config)
                .expect("Failed to serialize config");
            fs::write(config_path, json)
                .expect("Failed to write config file");
            
            println!("📝 Created default testnet config: {}", config_path);
            default_config
        }
    }
    
    pub fn default_testnet() -> Self {
        Self {
            mode: WalletMode::Testnet,
            demo_mode: false,
            bitcoin: BitcoinConfig {
                network: "testnet".to_string(),
                rpc_url: Some("https://blockstream.info/testnet/api".to_string()),
                electrum_url: Some("testnet.aranguren.org:51002".to_string()),
            },
            ethereum: EthereumConfig {
                chain_id: 11155111, // Sepolia
                rpc_url: Some("https://eth-sepolia.g.alchemy.com/v2/demo".to_string()),
                etherscan_api_key: None,
            },
            solana: SolanaConfig {
                cluster: "devnet".to_string(),
                rpc_url: Some("https://api.devnet.solana.com".to_string()),
            },
            monero: MoneroConfig {
                network: "stagenet".to_string(),
                daemon_url: Some("http://stagenet.xmr-tw.org:38081".to_string()),
            },
            hedera: HederaConfig {
                network: "testnet".to_string(),
                operator_id: None,
                operator_key: None,
            },
            icp: IcpConfig {
                network: "local".to_string(),
                identity_path: Some("~/.config/dfx/identity/default/identity.pem".to_string()),
                ic_url: Some("http://localhost:8000".to_string()),
            },
        }
    }
}

// Keep existing config structs for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    pub network: String,
    pub rpc_url: Option<String>,
    pub electrum_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumConfig {
    pub chain_id: u64,
    pub rpc_url: Option<String>,
    pub etherscan_api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    pub cluster: String,
    pub rpc_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneroConfig {
    pub network: String,
    pub daemon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HederaConfig {
    pub network: String,
    pub operator_id: Option<String>,
    pub operator_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcpConfig {
    pub network: String,
    pub identity_path: Option<String>,
    pub ic_url: Option<String>,
}
