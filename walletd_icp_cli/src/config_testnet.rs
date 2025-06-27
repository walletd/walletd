use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WalletMode {
    Demo,
    Testnet,
    Mainnet,
}

impl Default for WalletDConfig {
    fn default() -> Self {
        Self {
            mode: WalletMode::Testnet,  // Default to testnet!
            demo_mode: false,  // For backward compatibility
            bitcoin: BitcoinConfig {
                network: "testnet".to_string(),
                rpc_url: Some("https://blockstream.info/testnet/api".to_string()),
                electrum_url: Some("testnet.aranguren.org:51002".to_string()),
            },
            ethereum: EthereumConfig {
                chain_id: 11155111,  // Sepolia
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
                identity_path: None,
                ic_url: Some("http://localhost:8000".to_string()),
            },
        }
    }
}
