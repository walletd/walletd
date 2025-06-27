use crate::HederaClient;
use crate::core::config::HederaConfig;
use anyhow::Result;
use hedera::{PrivateKey, Hbar};

pub struct RealHederaWallet {
    pub network: String,
    pub account_id: Option<String>,
    pub public_key: String,
    pub private_key: String,
    pub client: Option<HederaClient>,  // Make this public
}

impl RealHederaWallet {
    pub fn new(network: &str) -> Result<Self> {
        let private_key = PrivateKey::generate_ed25519();
        let public_key = private_key.public_key();
        
        Ok(Self {
            network: network.to_string(),
            account_id: None,
            public_key: public_key.to_string(),
            private_key: private_key.to_string(),
            client: None,
        })
    }
    
    // New method to initialize with existing credentials
    pub async fn init_with_existing_account(&mut self) -> Result<()> {
        // Try to load config and create client
        match HederaConfig::load() {
            Ok(config) => {
                match HederaClient::new(config) {
                    Ok(client) => {
                        self.client = Some(client);
                        
                        // If we have an account ID, validate it exists
                        if let Some(account_id) = &self.account_id {
                            println!("✅ Initialized client for account: {}", account_id);
                            
                            // Try to get balance to verify account works
                            match self.get_balance().await {
                                Ok(balance) => {
                                    println!("💰 Account balance: {} HBAR", balance);
                                }
                                Err(e) => {
                                    println!("⚠️  Could not verify balance: {}", e);
                                }
                            }
                        }
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Failed to create client: {}", e))
                }
            }
            Err(e) => Err(anyhow::anyhow!("Failed to load config: {}", e))
        }
    }

    pub async fn create_testnet_account(&mut self, initial_balance: Hbar) -> Result<String> {
        // Initialize client if not already done
        if self.client.is_none() {
            let config = HederaConfig::load()?;
            self.client = Some(HederaClient::new(config)?);
        }
        
        let client = self.client.as_ref().unwrap();
        
        let account_info = client.create_new_account(initial_balance).await?;
        self.account_id = Some(account_info.account_id.to_string());
        
        Ok(account_info.account_id.to_string())
    }
    
    pub async fn get_balance(&self) -> Result<f64> {
        // If we have a client and account ID, get real balance
        if let (Some(client), Some(account_id)) = (&self.client, &self.account_id) {
            // Use the client to get real balance
            let balance = client.get_account_balance(account_id).await?;
            Ok(balance)
        } else {
            Ok(0.0)
        }
    }
    
    pub async fn send_hbar(&self, to_account: &str, amount: f64) -> Result<String> {
        if let (Some(client), Some(from_account)) = (&self.client, &self.account_id) {
            // Use the client to send real transaction
            let tx_id = client.transfer_hbar(from_account, to_account, amount).await?;
            Ok(tx_id)
        } else {
            Err(anyhow::anyhow!("Wallet not properly initialized"))
        }
    }
}

    // Convenience method with default balance

    // Add method to ensure client is initialized
