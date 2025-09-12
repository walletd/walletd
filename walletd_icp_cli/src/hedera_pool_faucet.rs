use crate::hedera_account_pool::ACCOUNT_POOL;
use crate::wallet_integration::WALLET_MANAGER;
use std::io::{self, Write};

pub async fn get_testnet_hbar_from_pool() -> Result<(), String> {
    println!("\n💰 Hedera Testnet Account Pool");
    println!("================================");

    println!("\n[1] 🔄 Get account from pool");
    println!("[2] 📊 Check pool status");
    println!("[3] ➕ Add new account to pool");
    println!("[4] 🔍 Check all account balances");

    print!("\nYour choice: ");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .map_err(|e| e.to_string())?;

    match choice.trim() {
        "1" => get_account_from_pool().await?,
        "2" => check_pool_status().await?,
        "3" => add_account_to_pool().await?,
        "4" => check_all_balances().await?,
        _ => println!("Invalid choice"),
    }

    Ok(())
}

async fn get_account_from_pool() -> Result<(), String> {
    let mut pool = ACCOUNT_POOL.write().await;

    println!("\n🔍 Finding available account...");

    match pool.get_next_available_account().await {
        Some(account) => {
            println!("✅ Found account: {}", account.account_id);
            println!("💰 Estimated balance: {} HBAR", account.estimated_balance);

            // Save and reload
            save_and_reload(&account.account_id, &account.private_key).await?;

            // Check actual balance
            let manager = WALLET_MANAGER.read().await;
            if let Some(wallet) = &manager.hedera {
                if let Ok(actual_balance) = wallet.get_balance().await {
                    drop(manager);

                    // Update pool with actual balance
                    let mut pool = ACCOUNT_POOL.write().await;
                    pool.report_balance(&account.account_id, actual_balance)
                        .await;

                    println!("💰 Actual balance: {actual_balance} HBAR");

                    if actual_balance < 10.0 {
                        println!("⚠️  Low balance! Consider adding more accounts to pool");
                    }
                }
            }
        }
        None => {
            println!("❌ No available accounts in pool!");
            println!("   All accounts may be depleted or in use");
            println!("   Try adding new funded accounts to the pool");
        }
    }

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn check_pool_status() -> Result<(), String> {
    let pool = ACCOUNT_POOL.read().await;
    let status = pool.get_pool_status().await;

    println!("\n📊 {status}");

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn add_account_to_pool() -> Result<(), String> {
    println!("\n➕ Add Account to Pool");
    println!("======================");

    print!("Account ID (e.g., 0.0.12345): ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut account_id = String::new();
    io::stdin()
        .read_line(&mut account_id)
        .map_err(|e| e.to_string())?;

    print!("Private Key: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut private_key = String::new();
    io::stdin()
        .read_line(&mut private_key)
        .map_err(|e| e.to_string())?;

    print!("Estimated Balance (HBAR): ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut balance_str = String::new();
    io::stdin()
        .read_line(&mut balance_str)
        .map_err(|e| e.to_string())?;

    let balance: f64 = balance_str.trim().parse().unwrap_or(0.0);

    let mut pool = ACCOUNT_POOL.write().await;
    pool.add_account(
        account_id.trim().to_string(),
        private_key.trim().to_string(),
        balance,
    )
    .await;

    println!("✅ Account added to pool!");

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn check_all_balances() -> Result<(), String> {
    println!("\n🔍 Checking all account balances...");

    let pool = ACCOUNT_POOL.read().await;

    // This would normally check actual balances
    // For now, show estimated balances
    println!("\n📊 Account Pool Status:");
    println!("{}", pool.get_pool_status().await);

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn save_and_reload(account_id: &str, private_key: &str) -> Result<(), String> {
    // Save credentials
    let env_content = format!(
        "HEDERA_NETWORK=testnet\nHEDERA_OPERATOR_ID={account_id}\nOPERATOR_PRIVATE_KEY={private_key}\n"
    );

    std::fs::write(".env.hedera", env_content).map_err(|e| format!("Failed to save: {e}"))?;

    // Reload
    dotenvy::from_filename(".env.hedera").ok();
    std::env::set_var("HEDERA_OPERATOR_ID", account_id);
    std::env::set_var("OPERATOR_PRIVATE_KEY", private_key);

    let mut manager = WALLET_MANAGER.write().await;
    manager
        .init_hedera()
        .await
        .map_err(|e| format!("Init failed: {e}"))?;

    Ok(())
}
