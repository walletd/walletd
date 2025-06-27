// In the wallet initialization section, add testnet detection:

println!("\nInitializing wallets...");

// Check if we're in testnet mode
let is_testnet = !config.demo_mode && config.bitcoin.network == "testnet";

if is_testnet {
    println!("🔄 Connecting to test networks...");
    {
        let mut manager = WALLET_MANAGER.write().await;
        
        // Initialize Bitcoin testnet
        if manager.config.bitcoin.network == "testnet" {
            println!("🔄 Initializing Bitcoin testnet...");
            manager.init_bitcoin().await?;
        }
        
        // Initialize other testnets
        manager.init_ethereum().await?;
        manager.init_solana().await?;
    }
} else if config.demo_mode {
    // Demo mode initialization (existing code)
    println!("🔄 Initializing Bitcoin wallet...");
    println!("✅ Bitcoin wallet initialized (Demo mode)");
    println!("🔄 Initializing Ethereum wallet...");
    println!("🔄 Initializing Solana wallet...");
    println!("✅ Solana wallet initialized (Demo mode - not implemented)");
} else {
    // Mainnet initialization
    let mut manager = WALLET_MANAGER.write().await;
    manager.init_bitcoin().await?;
    manager.init_ethereum().await?;
    manager.init_solana().await?;
}
