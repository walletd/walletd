pub async fn reload_hedera_credentials(wallet_manager: &mut WalletManager) -> Result<(), String> {
    println!("🔄 Reloading Hedera credentials...");
    
    // Reload environment
    dotenvy::from_filename(".env.hedera").ok();
    
    if let (Ok(operator_id), Ok(private_key)) = (
        std::env::var("HEDERA_OPERATOR_ID"),
        std::env::var("OPERATOR_PRIVATE_KEY")
    ) {
        println!("✅ Found credentials for account: {}", operator_id);
        
        // Reinitialize the wallet with new credentials
        if let Some(wallet) = &mut wallet_manager.hedera {
            wallet.account_id = Some(operator_id.clone());
            
            // Reinitialize the client
            let config = walletd_hedera::core::config::HederaConfig::load()
                .map_err(|e| format!("Config error: {}", e))?;
            
            wallet.client = Some(
                walletd_hedera::HederaClient::new(config)
                    .map_err(|e| format!("Client error: {}", e))?
            );
            
            println!("✅ Wallet reloaded with account: {}", operator_id);
            
            // Test by getting real balance
            match wallet.get_balance().await {
                Ok(balance) => println!("💰 Real balance: {} HBAR", balance),
                Err(e) => println!("⚠️  Could not get balance: {}", e),
            }
        }
    } else {
        return Err("No credentials found in .env.hedera".to_string());
    }
    
    Ok(())
}
