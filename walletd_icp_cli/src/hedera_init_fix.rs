// This should replace the init_hedera method in wallet_integration.rs

pub async fn init_hedera(&mut self) -> Result<()> {
    println!("🔄 Initializing Hedera wallet...");
    
    // First, check if we have existing credentials
    dotenvy::from_filename(".env.hedera").ok();
    
    let network = match self.mode {
        WalletMode::Testnet => "testnet",
        WalletMode::Mainnet => "mainnet",
        _ => "testnet",
    };
    
    let mut wallet = RealHederaWallet::new(network)?;
    
    println!("✅ Hedera wallet initialized ({})", network);
    println!("📍 Public Key: {}", wallet.public_key);
    println!("🔑 Private Key: {}", wallet.private_key);
    
    // Check if we already have operator credentials
    if let (Ok(operator_id), Ok(_)) = (
        std::env::var("HEDERA_OPERATOR_ID"),
        std::env::var("OPERATOR_PRIVATE_KEY")
    ) {
        println!("✅ Found existing operator account: {}", operator_id);
        wallet.account_id = Some(operator_id.clone());
        
        // Try to initialize the client with existing credentials
        match wallet.create_testnet_account().await {
            Ok(_) => {
                println!("✅ Connected to Hedera testnet with account: {}", operator_id);
            }
            Err(e) => {
                println!("⚠️  Using existing account {} (client init failed: {})", operator_id, e);
            }
        }
    } else if self.mode == WalletMode::Testnet {
        // No existing credentials, try to create new account
        println!("🌐 No existing account found, attempting to create new testnet account...");
        
        match wallet.create_testnet_account().await {
            Ok(account_id) => {
                println!("✅ Created REAL testnet account: {}", account_id);
                println!("🔍 Verify on: https://hashscan.io/testnet/account/{}", account_id);
            }
            Err(e) => {
                println!("❌ Could not create new account: {}", e);
                println!("\n💡 Use option 4.5 in Hedera menu to set up an account");
                
                // Don't use simulation - just leave account_id as None
                println!("⚠️  No account configured. Please set up an account to use Hedera.");
            }
        }
    }
    
    self.hedera = Some(wallet);
    println!("✅ Hedera wallet initialized");
    Ok(())
}
