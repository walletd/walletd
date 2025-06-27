// Add this to the menu display (after line with option [10]):
println!("\n[R] Reload Wallet (use after setting up credentials)");

// Add this case to the match statement:
"R" | "r" => {
    println!("\n🔄 Reloading Hedera wallet...");
    
    // Drop the read lock
    drop(manager);
    
    // Get write access to reload
    let mut manager_mut = WALLET_MANAGER.write().await;
    
    // Reload environment
    dotenvy::from_filename(".env.hedera").ok();
    
    // Reinitialize Hedera
    match manager_mut.init_hedera().await {
        Ok(_) => {
            println!("✅ Hedera wallet reloaded successfully");
        }
        Err(e) => {
            println!("❌ Failed to reload: {}", e);
        }
    }
    
    // Re-acquire read lock
    drop(manager_mut);
    manager = WALLET_MANAGER.read().await;
    
    println!("\nPress Enter to continue...");
    let mut _pause = String::new();
    std::io::stdin().read_line(&mut _pause).unwrap();
}
