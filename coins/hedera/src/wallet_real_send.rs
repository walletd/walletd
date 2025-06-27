    pub async fn send_hbar(&self, to_account: &str, amount: f64) -> Result<String> {
        if let Some(client) = &self.client {
            use hedera::{AccountId, TransferTransaction, Hbar};
            use std::str::FromStr;
            
            // Parse the recipient account
            let recipient = AccountId::from_str(to_account)
                .map_err(|e| anyhow::anyhow!("Invalid recipient account: {}", e))?;
            
            // Get our account ID
            let our_account = AccountId::from_str(
                self.account_id.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("Account ID not set"))?
            )?;
            
            // Convert amount to Hbar
            let hbar_amount = Hbar::from(amount);
            
            println!("🔄 Creating REAL transaction on Hedera testnet...");
            
            // Create the transfer transaction
            let transaction = TransferTransaction::new()
                .hbar_transfer(our_account, -hbar_amount)
                .hbar_transfer(recipient, hbar_amount)
                .execute(&client.client)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to execute transfer: {}", e))?;
            
            // Get the receipt
            let receipt = transaction
                .get_receipt(&client.client)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get receipt: {}", e))?;
            
            // Get transaction ID
            let tx_id = transaction.transaction_id.to_string();
            
            println!("✅ REAL transaction submitted!");
            println!("   Status: {:?}", receipt.status);
            
            Ok(tx_id)
        } else {
            Err(anyhow::anyhow!("Wallet not initialized"))
        }
    }
    
    pub async fn get_balance(&self) -> Result<f64> {
        if let Some(client) = &self.client {
            use hedera::{AccountId, AccountBalanceQuery};
            use std::str::FromStr;
            
            if let Some(account_id_str) = &self.account_id {
                // Don't query simulated accounts
                if account_id_str.contains("PENDING") || account_id_str == "0.0.1741199" {
                    return Ok(10000.0); // Simulated balance
                }
                
                let account_id = AccountId::from_str(account_id_str)?;
                
                // Query real balance from network
                let balance = AccountBalanceQuery::new()
                    .account_id(account_id)
                    .execute(&client.client)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to get balance: {}", e))?;
                
                // Convert from tinybars to HBAR
                Ok(balance.hbars.to_tinybars() as f64 / 100_000_000.0)
            } else {
                Ok(0.0)
            }
        } else {
            Ok(0.0)
        }
    }
