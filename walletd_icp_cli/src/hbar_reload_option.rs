// Add to the menu display:
println!("[R] Reload Wallet (after setting credentials)");

// Add to the match statement:
"R" | "r" => {
    println!("\n🔄 Reloading Hedera wallet...");
    
    // Drop the current manager to get write access
    drop(manager);
    
    // Get write access
    let mut manager_mut = WALLET_MANAGER.write().await;
    
    // Reload environment
    dotenvy::from_filename(".env.hedera").ok();
    
    if let (Ok(operator_id), Ok(private_key)) = (
        std::env::var("HEDERA_OPERATOR_ID"),
        std::env::var("OPERATOR_PRIVATE_KEY")
    ) {
        println!("✅ Found credentials for account: {}", operator_id);
        
        // Create a new wallet with the real account
        let mut new_wallet = RealHederaWallet::new("testnet").unwrap();
        new_wallet.account_id = Some(operator_id.clone());
        
        // Replace the old wallet
        manager_mut.hedera = Some(new_wallet);
        
        println!("✅ Wallet reloaded with account: {}", operator_id);
    } else {
        println!("❌ No credentials found in .env.hedera");
    }
    
    // Re-acquire read lock
    drop(manager_mut);
    manager = WALLET_MANAGER.read().await;
}
