// Add these methods INSIDE the impl HederaClient block, before the closing }

    pub async fn get_account_balance(&self, account_id: &str) -> Result<f64> {
        use hedera::{AccountBalanceQuery, AccountId};
        use std::str::FromStr;
        
        let account = AccountId::from_str(account_id)?;
        
        let balance = AccountBalanceQuery::new()
            .account_id(account)
            .execute(&self.client)
            .await?;
        
        // Convert from tinybars to HBAR
        Ok(balance.hbars.to_tinybars() as f64 / 100_000_000.0)
    }
    
    pub async fn transfer_hbar(&self, from: &str, to: &str, amount: f64) -> Result<String> {
        use hedera::{AccountId, TransferTransaction, Hbar};
        use std::str::FromStr;
        
        let from_account = AccountId::from_str(from)?;
        let to_account = AccountId::from_str(to)?;
        let hbar_amount = Hbar::from_tinybars((amount * 100_000_000.0) as i64);
        
        let transaction = TransferTransaction::new()
            .hbar_transfer(from_account, -hbar_amount)
            .hbar_transfer(to_account, hbar_amount)
            .execute(&self.client)
            .await?;
        
        let receipt = transaction.get_receipt(&self.client).await?;
        
        if receipt.status != hedera::Status::Success {
            return Err(anyhow::anyhow!("Transaction failed: {:?}", receipt.status));
        }
        
        Ok(transaction.transaction_id.to_string())
    }
