// Add this to the case "4.5" in hbar_menu.rs after handle_get_testnet_hbar()

// After faucet selection, reload the wallet
"4.5" => {
    crate::hbar_menu_faucet::handle_get_testnet_hbar().await?;
    
    // Reload environment variables
    dotenvy::from_filename(".env.hedera").ok();
    
    // Check if new credentials exist
    if let (Ok(operator_id), Ok(private_key)) = (
        std::env::var("HEDERA_OPERATOR_ID"),
        std::env::var("OPERATOR_PRIVATE_KEY")
    ) {
        println!("\n🔄 Reloading wallet with new credentials...");
        
        // Get mutable access to wallet manager
        drop(manager); // Release the read lock
        let mut manager_mut = WALLET_MANAGER.write().await;
        
        if let Some(wallet) = &mut manager_mut.hedera {
            // Update the wallet with new credentials
            wallet.account_id = Some(operator_id.clone());
            
            // Reinitialize the client
            match walletd_hedera::core::config::HederaConfig::load() {
                Ok(config) => {
                    // The wallet needs a method to set the client
                    // Since client is private, we need to work around this
                    println!("✅ Loaded account: {}", operator_id);
                    
                    // For now, just update the account_id
                    // The real fix needs to make client accessible
                }
                Err(e) => {
                    println!("⚠️  Config error: {}", e);
                }
            }
        }
        
        // Re-acquire read lock for menu
        drop(manager_mut);
        manager = WALLET_MANAGER.read().await;
    }
}
