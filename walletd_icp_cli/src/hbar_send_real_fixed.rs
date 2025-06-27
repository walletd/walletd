use crate::wallet_integration::WALLET_MANAGER;
use std::io::{self, Write};

pub async fn handle_send_hedera_real() -> Result<(), String> {
    let mut manager = WALLET_MANAGER.write().await;

    if let Some(wallet) = manager.get_hedera_wallet_mut() {
        // Check if we have a real account
        if wallet.account_id.is_none() || wallet.client.is_none() {
            println!("\n❌ No Hedera account configured!");
            println!("Please set up an account first (option 4.5)");
            return Ok(());
        }
        
        let account_id = wallet.account_id.as_ref().unwrap();
        
        // Get real balance from network
        let balance = match wallet.get_balance().await {
            Ok(bal) => bal,
            Err(e) => {
                println!("❌ Failed to get balance: {}", e);
                return Ok(());
            }
        };
        
        println!("\n=== Send HBAR (REAL TRANSACTION) ===");
        println!("Network: {}", wallet.network);
        println!("From: {}", account_id);
        println!("Balance: {} HBAR", balance);
        
        if balance < 0.01 {
            println!("\n❌ Insufficient balance for transaction");
            println!("Minimum needed: 0.01 HBAR (for fees)");
            return Ok(());
        }
        
        print!("\nRecipient Account ID: ");
        io::stdout().flush().unwrap();
        let mut to_account = String::new();
        io::stdin().read_line(&mut to_account).unwrap();
        let to_account = to_account.trim();
        
        // Validate account format
        if !to_account.starts_with("0.0.") {
            println!("❌ Invalid account format. Use format: 0.0.XXXXX");
            return Ok(());
        }
        
        print!("Amount (HBAR): ");
        io::stdout().flush().unwrap();
        let mut amount_str = String::new();
        io::stdin().read_line(&mut amount_str).unwrap();
        let amount: f64 = match amount_str.trim().parse() {
            Ok(amt) => amt,
            Err(_) => {
                println!("❌ Invalid amount");
                return Ok(());
            }
        };
        
        if amount > balance - 0.01 {
            println!("❌ Amount too high (need to keep 0.01 HBAR for fees)");
            return Ok(());
        }
        
        println!("\n📋 Transaction Summary:");
        println!("From: {}", account_id);
        println!("To: {}", to_account);
        println!("Amount: {} HBAR", amount);
        println!("Network: {}", wallet.network);
        println!("Estimated Fee: ~0.0001 HBAR");
        
        print!("\n⚠️  This is a REAL transaction. Confirm? (yes/no): ");
        io::stdout().flush().unwrap();
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).unwrap();
        
        if confirm.trim().to_lowercase() == "yes" {
            println!("\n🔄 Sending REAL transaction to Hedera {}...", wallet.network);
            
            match wallet.send_hbar(to_account, amount).await {
                Ok(tx_id) => {
                    println!("\n✅ Transaction successful!");
                    println!("Transaction ID: {}", tx_id);
                    
                    let explorer_url = if wallet.network == "testnet" {
                        format!("https://hashscan.io/testnet/transaction/{}", tx_id)
                    } else {
                        format!("https://hashscan.io/mainnet/transaction/{}", tx_id)
                    };
                    
                    println!("🔍 View on HashScan: {}", explorer_url);
                    
                    // Get updated balance
                    if let Ok(new_balance) = wallet.get_balance().await {
                        println!("💰 New balance: {} HBAR", new_balance);
                    }
                }
                Err(e) => {
                    println!("\n❌ Transaction failed: {}", e);
                }
            }
        } else {
            println!("Transaction cancelled");
        }
    } else {
        println!("❌ Hedera wallet not initialized");
    }
    
    println!("\nPress Enter to continue...");
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).unwrap();
    
    Ok(())
}
