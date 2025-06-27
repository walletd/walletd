// Add this case to the menu handler
"4.5" => {
    crate::hbar_menu_faucet::handle_get_testnet_hbar().await?;
    
    // After setting up credentials, reload the wallet
    println!("\n🔄 Reloading wallet with new credentials...");
    
    // Reload environment
    dotenvy::from_filename(".env.hedera").ok();
    
    if let Some(wallet) = manager.get_hedera_wallet_mut() {
        if let (Ok(operator_id), Ok(_)) = (
            std::env::var("HEDERA_OPERATOR_ID"),
            std::env::var("OPERATOR_PRIVATE_KEY")
        ) {
            // Reinitialize with new credentials
            match walletd_hedera::core::config::HederaConfig::load() {
                Ok(config) => {
                    match walletd_hedera::HederaClient::new(config) {
                        Ok(client) => {
                            wallet.client = Some(client);
                            wallet.account_id = Some(operator_id.clone());
                            
                            println!("✅ Wallet reloaded with account: {}", operator_id);
                            
                            // Show real balance
                            if let Ok(balance) = wallet.get_balance().await {
                                println!("💰 Real balance: {} HBAR", balance);
                            }
                        }
                        Err(e) => println!("⚠️  Client init failed: {}", e),
                    }
                }
                Err(e) => println!("⚠️  Config load failed: {}", e),
            }
        }
    }
}
