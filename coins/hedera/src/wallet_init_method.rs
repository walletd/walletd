// Add this method to RealHederaWallet impl

pub async fn init_with_credentials(&mut self) -> Result<()> {
    // Load config
    let config = HederaConfig::load()?;
    
    // Create and set client
    self.client = Some(HederaClient::new(config)?);
    
    // If we have operator credentials, the account_id should already be set
    if let Some(account_id) = &self.account_id {
        println!("✅ Initialized with account: {}", account_id);
        
        // Test by getting balance
        match self.get_balance().await {
            Ok(balance) => {
                println!("💰 Real balance: {} HBAR", balance);
            }
            Err(e) => {
                println!("⚠️  Balance check failed: {}", e);
            }
        }
    }
    
    Ok(())
}
