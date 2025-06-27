    pub async fn get_balance(&self) -> Result<f64> {
        // ALWAYS try to get real balance
        if let Some(account_id) = &self.account_id {
            // Try SDK first
            if let Some(client) = &self.client {
                match client.get_account_balance(account_id).await {
                    Ok(balance) => return Ok(balance),
                    Err(e) => {
                        println!("SDK error, trying mirror node: {}", e);
                    }
                }
            }
            
            // Fallback to mirror node API
            let url = format!("https://testnet.mirrornode.hedera.com/api/v1/accounts/{}", account_id);
            match reqwest::get(&url).await {
                Ok(response) => {
                    if let Ok(json) = response.json::<serde_json::Value>().await {
                        if let Some(balance) = json["balance"]["balance"].as_i64() {
                            return Ok(balance as f64 / 100_000_000.0);
                        }
                    }
                }
                Err(_) => {}
            }
        }
        
        // Only return 0 if we truly can't get balance
        Ok(0.0)
    }
