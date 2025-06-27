use crate::wallet_integration::WALLET_MANAGER;

pub async fn handle_check_real_balance() -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;
    
    if let Some(wallet) = &manager.hedera {
        if let Some(account_id) = &wallet.account_id {
            println!("\n🔄 Checking real balance for {}...", account_id);
            
            match wallet.get_balance().await {
                Ok(balance) => {
                    println!("✅ Real Balance: {} HBAR", balance);
                }
                Err(e) => {
                    println!("❌ Error: {}", e);
                    println!("Using simulated balance");
                }
            }
        }
    }
    
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).unwrap();
    Ok(())
}
