// In wallet_integration.rs, update init_hedera:

if self.mode == WalletMode::Testnet {
    println!("🌐 Connecting to REAL Hedera testnet...");
    
    match wallet.create_testnet_account().await {
        Ok(account_id) => {
            println!("✅ Created REAL testnet account: {}", account_id);
            println!("🔍 Verify on: https://hashscan.io/testnet/account/{}", account_id);
        }
        Err(e) => {
            if e.to_string().contains("InvalidSignature") {
                println!("⚠️  Connected to Hedera testnet but cannot create accounts");
                println!("💡 The operator account credentials don't match");
                println!("📋 To create real accounts, you need:");
                println!("   1. A funded testnet account from https://portal.hedera.com/");
                println!("   2. Update HEDERA_OPERATOR_ID and OPERATOR_PRIVATE_KEY");
                
                // Use a placeholder account for testing UI
                wallet.account_id = Some("0.0.PENDING".to_string());
            } else {
                println!("⚠️  Failed to create testnet account: {}", e);
            }
        }
    }
}
