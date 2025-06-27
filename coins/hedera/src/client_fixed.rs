use crate::core::config::HederaConfig;
use anyhow::Result;
use hedera::{Client, AccountCreateTransaction, Hbar, PrivateKey, PublicKey};

pub struct HederaClient {
    pub client: Client,
    pub operator_id: String,
    pub operator_key: PrivateKey,
}

pub struct AccountInfo {
    pub account_id: hedera::AccountId,
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

impl HederaClient {
    pub fn new(config: HederaConfig) -> Result<Self> {
        let client = match config.hedera_network.as_str() {
            "mainnet" => Client::for_mainnet(),
            "testnet" => Client::for_testnet(),
            "previewnet" => Client::for_previewnet(),
            _ => return Err(anyhow::anyhow!("Invalid network: {}", config.hedera_network)),
        };
        
        let operator_key = config.operator_private_key.parse::<PrivateKey>()?;
        client.set_operator(config.operator_id.parse()?, operator_key.clone());
        
        Ok(Self {
            client,
            operator_id: config.operator_id,
            operator_key,
        })
    }
    
    pub async fn create_new_account(&self, initial_balance: Hbar) -> Result<AccountInfo> {
        let new_private_key = PrivateKey::generate_ed25519();
        let new_public_key = new_private_key.public_key();
        
        let transaction = AccountCreateTransaction::new()
            .key(new_public_key.clone())
            .initial_balance(initial_balance)
            .execute(&self.client)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to execute account creation: {}", e))?;
        
        let receipt = transaction
            .get_receipt(&self.client)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get receipt: {}", e))?;
        
        let account_id = receipt.account_id
            .ok_or_else(|| anyhow::anyhow!("No account ID in receipt"))?;
        
        Ok(AccountInfo {
            account_id,
            public_key: new_public_key,
            private_key: new_private_key,
        })
    }
    
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
}
