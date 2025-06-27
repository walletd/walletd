use crate::wallet_integration::WALLET_MANAGER;

pub async fn handle_check_balance() -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;
    
    if let Some(wallet) = &manager.hedera {
        if wallet.account_id.is_none() {
            println!("\n❌ No Hedera account configured!");
            return Ok(());
        }
        
        let account_id = wallet.account_id.as_ref().unwrap();
        println!("\n🔄 Checking balance for account: {}", account_id);
        println!("Network: {}", wallet.network);
        
        match wallet.get_balance().await {
            Ok(balance) => {
                println!("\n✅ Account Balance:");
                println!("================");
                println!("Account: {}", account_id);
                println!("Balance: {} HBAR", balance);
                println!("Network: {}", wallet.network);
                
                // Also show in tinybars
                let tinybars = (balance * 100_000_000.0) as i64;
                println!("Balance: {} tinybars", tinybars);
                
                let explorer_url = if wallet.network == "testnet" {
                    format!("https://hashscan.io/testnet/account/{}", account_id)
                } else {
                    format!("https://hashscan.io/mainnet/account/{}", account_id)
                };
                
                println!("\n🔍 View on HashScan: {}", explorer_url);
            }
            Err(e) => {
                println!("\n❌ Failed to get balance: {}", e);
                println!("This could mean:");
                println!("- Network connection issues");
                println!("- Invalid account ID");
                println!("- Account doesn't exist");
            }
        }
    } else {
        println!("❌ Hedera wallet not initialized");
    }
    
    println!("\nPress Enter to continue...");
    let mut _pause = String::new();
    std::io::stdin().read_line(&mut _pause).unwrap();
    
    Ok(())
}
