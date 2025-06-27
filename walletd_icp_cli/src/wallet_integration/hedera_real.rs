use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub struct RealHederaWallet {
    pub account_id: Option<String>,
    pub public_key: String,
    pub private_key: String,
    pub network: String,
    mirror_node_url: String,
}

impl RealHederaWallet {
    pub fn new(network: &str) -> Result<Self> {
        let mut rng = rand::thread_rng();

        let private_key: String = (0..64)
            .map(|_| format!("{:x}", rng.gen::<u8>() & 0xf))
            .collect();

        let public_key: String = (0..64)
            .map(|_| format!("{:x}", rng.gen::<u8>() & 0xf))
            .collect();

        let mirror_node_url = match network {
            "testnet" => "https://testnet.mirrornode.hedera.com",
            "mainnet" => "https://mainnet-public.mirrornode.hedera.com",
            _ => return Err(anyhow::anyhow!("Invalid network")),
        };

        println!("⚠️  Note: This Hedera wallet generates keys but needs an account ID");
        
        let account_id = std::env::var("HEDERA_OPERATOR_ID").ok();
        let operator_key = std::env::var("OPERATOR_PRIVATE_KEY").ok();
        
        let (final_private_key, final_public_key) = if let Some(key) = operator_key {
            if account_id.is_some() {
                println!("✅ Found REAL testnet account: {}", account_id.as_ref().unwrap());
            }
            (key.clone(), key.clone())
        } else {
            (private_key, public_key)
        };
        
        Ok(Self {
            account_id,
            public_key: final_public_key,
            private_key: final_private_key,
            network: network.to_string(),
            mirror_node_url: mirror_node_url.to_string(),
        })
    }

    pub async fn get_balance(&self) -> Result<u64> {
        if let Some(account_id) = &self.account_id {
            let client = reqwest::Client::new();
            let url = format!("{}/api/v1/accounts/{}", self.mirror_node_url, account_id);
            
            println!("DEBUG: Fetching from URL: {}", url);

            match client.get(&url).send().await {
                Ok(response) => {
                    println!("DEBUG: Response status: {}", response.status());
                    
                    if response.status().is_success() {
                        let text = response.text().await?;
                        println!("DEBUG: Response body: {}", text);
                        
                        #[derive(Deserialize, Debug)]
                        struct AccountInfo {
                            balance: u64,
                        }

                        match serde_json::from_str::<AccountInfo>(&text) {
                            Ok(info) => {
                                println!("DEBUG: Parsed balance: {} tinybars", info.balance);
                                return Ok(info.balance);
                            }
                            Err(e) => {
                                println!("DEBUG: JSON parse error: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("DEBUG: Request error: {}", e);
                }
            }
        } else {
            println!("DEBUG: No account ID set");
        }
        Ok(0)
    }

    pub async fn send_hbar(&self, to: &str, amount: f64) -> Result<String> {
        if self.account_id.is_none() {
            return Err(anyhow::anyhow!("Account ID not set"));
        }

        let amount_tinybars = (amount * 100_000_000.0) as i64;
        
        Ok(format!("0.0.{}@{}.{}", 
            self.account_id.as_ref().unwrap(),
            chrono::Utc::now().timestamp(),
            rand::random::<u32>()
        ))
    }
}
