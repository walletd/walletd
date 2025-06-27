// Add this function to hbar_menu.rs or as a separate module

use crate::wallet_integration::WALLET_MANAGER;

pub async fn reload_hedera_wallet() -> Result<(), String> {
    println!("\n🔄 Reloading Hedera wallet with new credentials...");
    
    // Reload environment variables
    dotenvy::from_filename(".env.hedera").ok();
    
    let mut manager = WALLET_MANAGER.write().await;
    
    if let Some(wallet) = &mut manager.hedera {
        if let (Ok(operator_id), Ok(_)) = (
            std::env::var("HEDERA_OPERATOR_ID"),
            std::env::var("OPERATOR_PRIVATE_KEY")
        ) {
            // Update account ID
            wallet.account_id = Some(operator_id.clone());
            
            // Try to reinitialize the client
            match walletd_hedera::core::config::HederaConfig::load() {
                Ok(config) => {
                    match walletd_hedera::HederaClient::new(config) {
                        Ok(client) => {
                            wallet.client = Some(client);
                            println!("✅ Wallet reloaded with account: {}", operator_id);
                            
                            // Test with real balance
                            match wallet.get_balance().await {
                                Ok(balance) => {
                                    println!("💰 Real balance: {} HBAR", balance);
                                }
                                Err(e) => {
                                    println!("⚠️  Balance check failed: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("⚠️  Failed to create client: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("⚠️  Failed to load config: {}", e);
                }
            }
        } else {
            println!("❌ No credentials found in environment");
        }
    }
    
    Ok(())
}
