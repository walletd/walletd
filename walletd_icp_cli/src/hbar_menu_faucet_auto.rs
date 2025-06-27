use crate::wallet_integration::WALLET_MANAGER;
use std::io::{self, Write};

pub async fn handle_get_testnet_hbar() -> Result<(), String> {
    println!("\n=== Get Testnet HBAR ===\n");
    
    println!("[1] 🤖 AUTO - Find & Fund Account (recommended)");
    println!("[2] Use pre-configured test account");
    println!("[3] Generate new keys for manual setup");
    println!("[4] Cancel");
    
    print!("\nSelect option: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).map_err(|e| e.to_string())?;
    
    match choice.trim() {
        "1" => handle_auto_fund_account().await?,
        "2" => setup_prefunded_account().await?,
        "3" => generate_new_keys()?,
        _ => println!("Cancelled"),
    }
    
    Ok(())
}

// Include the auto_fund_testnet_account function from the artifact above
// ... (copy the code from the artifact)

async fn setup_prefunded_account() -> Result<(), String> {
    use crate::hedera_testnet_accounts::TESTNET_ACCOUNTS;
    
    println!("\n🔄 Available test accounts:\n");
    
    for (i, account) in TESTNET_ACCOUNTS.iter().enumerate() {
        println!("[{}] Account: {} - {}", i + 1, account.account_id, account.description);
    }
    
    print!("\nSelect account [1-3]: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).map_err(|e| e.to_string())?;
    
    match choice.trim() {
        "1" | "2" | "3" => {
            let index = choice.trim().parse::<usize>().unwrap() - 1;
            let account = &TESTNET_ACCOUNTS[index];
            
            // Same implementation as before but check balance first
            println!("\n🔍 Checking account balance...");
            
            // Query the mirror node for balance
            let url = format!("https://testnet.mirrornode.hedera.com/api/v1/accounts/{}", account.account_id);
            
            match reqwest::get(&url).await {
                Ok(response) => {
                    if let Ok(json) = response.json::<serde_json::Value>().await {
                        if let Some(balance) = json["balance"]["balance"].as_i64() {
                            let hbar = balance as f64 / 100_000_000.0;
                            println!("💰 Current balance: {} HBAR", hbar);
                            
                            if balance == 0 {
                                println!("⚠️  This account has no balance");
                                println!("   Try option 1 for automatic funding");
                                return Ok(());
                            }
                        }
                    }
                }
                Err(_) => {}
            }
            
            // Continue with setup...
            save_and_reload_account(account.account_id, account.private_key).await?;
        }
        _ => println!("Invalid selection"),
    }
    
    Ok(())
}

async fn save_and_reload_account(account_id: &str, private_key: &str) -> Result<(), String> {
    // Save credentials
    let env_content = format!(
        "# Hedera Testnet Configuration\nHEDERA_NETWORK=testnet\nHEDERA_OPERATOR_ID={}\nOPERATOR_PRIVATE_KEY={}\n",
        account_id,
        private_key
    );
    
    std::fs::write(".env.hedera", env_content)
        .map_err(|e| format!("Failed to save: {}", e))?;
    
    println!("📝 Credentials saved");
    
    // Reload
    dotenvy::from_filename(".env.hedera").ok();
    std::env::set_var("HEDERA_OPERATOR_ID", account_id);
    std::env::set_var("OPERATOR_PRIVATE_KEY", private_key);
    
    let mut manager = WALLET_MANAGER.write().await;
    match manager.init_hedera().await {
        Ok(_) => {
            println!("✅ Wallet reloaded!");
            if let Some(wallet) = &manager.hedera {
                if let Ok(balance) = wallet.get_balance().await {
                    println!("💰 Balance: {} HBAR", balance);
                }
            }
        }
        Err(e) => println!("⚠️  Reload warning: {}", e),
    }
    
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).map_err(|e| e.to_string())?;
    Ok(())
}

fn generate_new_keys() -> Result<(), String> {
    println!("\n=== Generate New Keys ===");
    println!("\n1. Visit: https://portal.hedera.com/");
    println!("2. Create a testnet account");
    println!("3. You'll get 10,000 HBAR automatically");
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).map_err(|e| e.to_string())?;
    Ok(())
}
