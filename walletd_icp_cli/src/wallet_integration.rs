use crate::config::WalletDConfig;
use crate::mode_selector::WalletMode;
use anyhow::Result;
use dotenvy;
use monero_real::RealMoneroWallet;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod bitcoin_real;
pub mod ethereum_real;
pub mod monero_real;
pub mod solana_real;
use bitcoin_real::RealBitcoinWallet;
use ethereum_real::RealEthereumWallet;
pub use walletd_hedera::wallet::RealHederaWallet;

pub struct WalletManager {
    pub config: WalletDConfig,
    pub mode: WalletMode,
    pub bitcoin: Option<RealBitcoinWallet>,
    pub ethereum: Option<RealEthereumWallet>,
    pub eth_provider: Option<String>,
    pub solana: Option<solana_real::RealSolanaWallet>,
    pub monero: Option<monero_real::RealMoneroWallet>,
    pub hedera: Option<RealHederaWallet>,
}
#[derive(Debug, Clone)]
pub struct Balance {
    pub confirmed: u64,
    pub unconfirmed: u64,
}

impl WalletManager {
    pub fn new(config: WalletDConfig) -> Self {
        let mode = if config.demo_mode {
            WalletMode::Demo
        } else if config.bitcoin.network == "testnet" {
            WalletMode::Testnet
        } else {
            WalletMode::Mainnet
        };

        Self {
            config,
            mode,
            bitcoin: None,
            ethereum: None,
            eth_provider: None,
            solana: None,
            monero: None,
            hedera: None,
        }
    }
    pub async fn init_bitcoin(&mut self) -> Result<()> {
        println!("🔄 Initializing Bitcoin wallet...");

        let network = match self.config.bitcoin.network.as_str() {
            "testnet" => bitcoin::Network::Testnet,
            "mainnet" => bitcoin::Network::Bitcoin,
            _ => bitcoin::Network::Testnet,
        };

        let wallet = RealBitcoinWallet::new(network)?;

        println!("✅ Bitcoin wallet initialized ({network:?})");
        println!("📍 Address: {}", wallet.address);
        println!("🔑 Private Key (WIF): {}", wallet.private_key.to_wif());

        match wallet.get_balance().await {
            Ok(balance) => {
                let btc = balance as f64 / 100_000_000.0;
                println!("💰 Balance: {btc} BTC ({balance} sats)");
            }
            Err(_) => {
                println!("💰 Balance: Unable to fetch (network issue?)");
            }
        }

        self.bitcoin = Some(wallet);
        Ok(())
    }

    pub async fn init_ethereum(&mut self) -> Result<()> {
        println!("🔄 Initializing Ethereum wallet...");

        let mut wallet = RealEthereumWallet::new(self.config.ethereum.chain_id)?;

        if let Err(e) = wallet.connect().await {
            println!("⚠️  Could not connect to Ethereum network: {e}");
        }

        println!(
            "✅ Ethereum wallet initialized (chain {})",
            self.config.ethereum.chain_id
        );
        println!("📍 Address: 0x{:x}", wallet.address);
        println!("🔑 Private Key: {}", wallet.get_private_key());

        if self.config.ethereum.chain_id == 11155111 {
            println!("💡 Get Sepolia ETH from: https://sepoliafaucet.com/");
        }

        self.ethereum = Some(wallet);
        self.eth_provider = Some("https://eth-sepolia.g.alchemy.com/v2/demo".to_string());

        Ok(())
    }

    pub async fn init_solana(&mut self) -> Result<()> {
        println!("🔄 Initializing Solana wallet...");

        let wallet = solana_real::RealSolanaWallet::new(&self.config.solana.cluster)?;

        println!(
            "✅ Solana wallet initialized ({})",
            self.config.solana.cluster
        );
        println!("📍 Address: {}", wallet.address);
        println!("🔑 Private Key: {}", wallet.get_private_key());

        match wallet.get_balance().await {
            Ok(balance) => {
                let sol = balance as f64 / 1_000_000_000.0;
                println!("💰 Balance: {sol} SOL ({balance} lamports)");

                if balance == 0 && self.config.solana.cluster == "devnet" {
                    println!("\n💡 Your wallet has 0 SOL. You can:");
                    println!("   1. Get airdrop from menu option 3");
                    println!("   2. Use web faucet: https://faucet.solana.com/");
                }
            }
            Err(e) => {
                println!("💰 Balance: Unable to fetch ({e})");
            }
        }

        self.solana = Some(wallet);
        Ok(())
    }

    pub async fn get_bitcoin_wallet(&self, _user_id: &str) -> Result<(String, String)> {
        if let Some(wallet) = &self.bitcoin {
            match wallet.get_balance().await {
                Ok(balance) => {
                    let btc = balance as f64 / 100_000_000.0;
                    Ok((wallet.address.to_string(), format!("{btc:.8}")))
                }
                Err(_) => Ok((wallet.address.to_string(), "0.00000000".to_string())),
            }
        } else {
            Err(anyhow::anyhow!("Bitcoin wallet not initialized"))
        }
    }

    pub async fn get_ethereum_wallet(&self) -> Result<(String, String)> {
        if let Some(wallet) = &self.ethereum {
            match wallet.get_balance().await {
                Ok(balance) => {
                    let eth = ethers::utils::format_units(balance, "ether")
                        .unwrap_or_else(|_| "0.0".to_string());
                    Ok((format!("0x{:x}", wallet.address), eth))
                }
                Err(_) => Ok((format!("0x{:x}", wallet.address), "0.0".to_string())),
            }
        } else {
            Err(anyhow::anyhow!("Ethereum wallet not initialized"))
        }
    }

    pub async fn get_solana_wallet(&self, _user_id: &str) -> Result<(String, String)> {
        if let Some(wallet) = &self.solana {
            match wallet.get_balance().await {
                Ok(balance) => {
                    let sol = balance as f64 / 1_000_000_000.0;
                    Ok((wallet.address.clone(), format!("{sol:.9}")))
                }
                Err(_) => Ok((wallet.address.clone(), "0.000000000".to_string())),
            }
        } else {
            Ok(("SolanaWallet".to_string(), "0.0".to_string()))
        }
    }

    pub async fn send_bitcoin(&self, to_address: &str, amount: f64) -> Result<String> {
        if let Some(wallet) = &self.bitcoin {
            let amount_sats = (amount * 100_000_000.0) as u64;

            println!("📡 Creating real Bitcoin transaction...");
            let txid = wallet
                .create_and_send_transaction(to_address, amount_sats)
                .await?;

            let explorer_url = match wallet.network {
                bitcoin::Network::Testnet => {
                    format!("https://blockstream.info/testnet/tx/{txid}")
                }
                bitcoin::Network::Bitcoin => format!("https://blockstream.info/tx/{txid}"),
                _ => String::new(),
            };

            println!("✅ Transaction broadcast successfully!");
            println!("📍 Transaction ID: {txid}");
            println!("🔍 View on explorer: {explorer_url}");

            Ok(txid)
        } else {
            Err(anyhow::anyhow!("Bitcoin wallet not initialized"))
        }
    }

    pub async fn send_ethereum(&self, to_address: &str, amount_wei: u64) -> Result<String> {
        if let Some(wallet) = &self.ethereum {
            let amount_eth = amount_wei as f64 / 1e18;

            println!("📡 Creating real Ethereum transaction...");
            match wallet.send_transaction(to_address, amount_eth).await {
                Ok(tx_hash) => {
                    println!("✅ Transaction broadcast successfully!");
                    println!("📍 Transaction hash: {tx_hash}");

                    if self.config.ethereum.chain_id == 11155111 {
                        println!(
                            "🔍 View on Sepolia Etherscan: https://sepolia.etherscan.io/tx/{tx_hash}"
                        );
                    }

                    Ok(tx_hash)
                }
                Err(e) => Err(e),
            }
        } else {
            Err(anyhow::anyhow!("Ethereum wallet not initialized"))
        }
    }

    //     pub async fn init_hedera(&mut self) -> Result<()> {
    //         println!("🔄 Initializing Hedera wallet...");
    //
    //         let network = match self.mode {
    //             WalletMode::Testnet => "testnet",
    //             WalletMode::Mainnet => "mainnet",
    //             _ => "testnet",
    //         };
    //
    //         let mut wallet = RealHederaWallet::new(network)?;
    //
    //         println!("✅ Hedera wallet initialized ({})", network);
    //         println!("📍 Public Key: {}", wallet.public_key);
    //         println!("🔑 Private Key: {}", wallet.private_key);
    //
    //
    // Initialize the Hedera client with existing account
    //         Ok(())
    //     }
    //
    pub async fn init_hedera(&mut self) -> Result<()> {
        println!("🔄 Initializing Hedera wallet...");

        // ALWAYS load .env.hedera FIRST
        dotenvy::from_filename(".env.hedera").ok();

        let network = match self.mode {
            WalletMode::Testnet => "testnet",
            WalletMode::Mainnet => "mainnet",
            _ => "testnet",
        };

        let mut wallet = RealHederaWallet::new(network)?;

        println!("✅ Hedera wallet initialized ({network})");
        println!("📍 Public Key: {}", wallet.public_key);

        // CHECK FOR EXISTING CREDENTIALS - DO NOT CREATE SIMULATED ACCOUNTS
        if let (Ok(operator_id), Ok(_operator_key)) = (
            std::env::var("HEDERA_OPERATOR_ID"),
            std::env::var("OPERATOR_PRIVATE_KEY"),
        ) {
            println!("✅ Found REAL testnet account: {operator_id}");
            wallet.account_id = Some(operator_id);
            // Don't print the private key
        } else {
            println!("⚠️  No Hedera account configured");
            println!("   Using the wallet without an account");
            // DO NOT CREATE SIMULATED ACCOUNT
            // wallet.account_id remains None
        }

        // Initialize the Hedera client with existing account
        if wallet.account_id.is_some() {
            match wallet.init_with_existing_account().await {
                Ok(_) => println!("✅ Hedera client initialized successfully"),
                Err(e) => println!("⚠️  Failed to initialize Hedera client: {e}"),
            }
        }

        self.hedera = Some(wallet);
        Ok(())
    }
    pub async fn init_monero(&mut self) -> Result<()> {
        println!("🔄 Initializing Monero wallet...");

        let network = match self.mode {
            WalletMode::Testnet => "stagenet",
            WalletMode::Mainnet => "mainnet",
            _ => "stagenet",
        };

        let wallet = RealMoneroWallet::new(network)?;

        println!("✅ Monero wallet initialized ({network})");
        println!(
            "📍 Address: {}...{}",
            &wallet.address[..12],
            &wallet.address[wallet.address.len() - 12..]
        );

        self.monero = Some(wallet);
        Ok(())
    }
    pub async fn get_hedera_wallet(&self, _user_id: &str) -> Result<(String, String)> {
        if let Some(wallet) = &self.hedera {
            let account_id = wallet
                .account_id
                .clone()
                .unwrap_or_else(|| "0.0.pending".to_string());

            // For testnet, always show 10000 HBAR balance
            let balance = "0.0".to_string();
            Ok((account_id, balance))
        } else {
            Err(anyhow::anyhow!("Hedera wallet not initialized"))
        }
    }
}

pub struct BitcoinManager;

impl BitcoinManager {
    pub async fn get_balance(&self, _user_id: &str) -> Result<Balance> {
        Ok(Balance {
            confirmed: 0,
            unconfirmed: 0,
        })
    }

    pub async fn get_receive_address(
        &self,
        _user_id: &str,
        _address_type: walletd_bitcoin::AddressType,
    ) -> Result<String> {
        Ok("tb1q...".to_string())
    }
}

pub struct EthereumManager {
    pub address: String,
}

impl EthereumManager {
    pub fn address(&self) -> String {
        self.address.clone()
    }
}

pub static WALLET_MANAGER: Lazy<Arc<RwLock<WalletManager>>> = Lazy::new(|| {
    let config = WalletDConfig::load();
    Arc::new(RwLock::new(WalletManager::new(config)))
});
