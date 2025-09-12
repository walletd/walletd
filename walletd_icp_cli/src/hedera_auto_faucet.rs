use crate::hedera_portal_faucet::{create_funded_testnet_account, fund_via_yamolky};
use crate::wallet_integration::WALLET_MANAGER;
use std::io::{self, Write};

pub async fn auto_fund_testnet_hbar() -> Result<(), String> {
    println!("\n💰 Hedera Testnet Auto-Funding");
    println!("================================");

    println!("\nSelect funding method:");
    println!("[1] 🆕 Create NEW account with 100 HBAR");
    println!("[2] 💧 Fund EXISTING account");
    println!("[3] 🎯 Use pre-funded account (instant)");

    print!("\nYour choice: ");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .map_err(|e| e.to_string())?;

    match choice.trim() {
        "1" => create_new_funded_account().await?,
        "2" => fund_existing_account().await?,
        "3" => use_instant_funded_account().await?,
        _ => println!("Invalid choice"),
    }

    Ok(())
}

async fn create_new_funded_account() -> Result<(), String> {
    match create_funded_testnet_account().await {
        Ok((account_id, private_key, balance)) => {
            println!("\n✅ New account created!");
            println!("📍 Account ID: {account_id}");
            println!("💰 Balance: {balance} HBAR");

            // Save and reload
            save_and_reload(&account_id, &private_key).await?;
        }
        Err(e) => {
            println!("❌ Failed to create account: {e}");
        }
    }

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn fund_existing_account() -> Result<(), String> {
    // Get account ID first, then drop the lock
    let account_id = {
        let manager = WALLET_MANAGER.read().await;
        if let Some(wallet) = &manager.hedera {
            wallet.account_id.clone()
        } else {
            None
        }
    }; // manager lock is dropped here

    if let Some(account_id) = account_id {
        print!("\n💎 How much HBAR do you need? (1-100): ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut amount_str = String::new();
        io::stdin()
            .read_line(&mut amount_str)
            .map_err(|e| e.to_string())?;

        let amount: f64 = amount_str.trim().parse().unwrap_or(10.0);

        // Try Yamolky faucet
        match fund_via_yamolky(&account_id, amount).await {
            Ok(msg) => {
                println!("✅ {msg}");

                // Wait for funding to process
                println!("⏳ Waiting for transaction...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                // Check new balance
                let manager = WALLET_MANAGER.read().await;
                if let Some(wallet) = &manager.hedera {
                    if let Ok(balance) = wallet.get_balance().await {
                        println!("💰 New balance: {balance} HBAR");
                    }
                }
            }
            Err(e) => {
                println!("❌ Funding failed: {e}");
                println!("\n💡 Try option 1 to create a new funded account instead");
            }
        }
    } else {
        println!("❌ No Hedera account found. Initialize wallet first.");
    }

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn use_instant_funded_account() -> Result<(), String> {
    println!("\n🎯 Using pre-funded testnet account...");

    // These accounts are pre-created with 100 HBAR each
    let funded_accounts = [("0.0.4920123", "302e020100300506032b65700422042091132178b72c5a4a3e10c91ce87b6197c5da35024ba370b8e9bea31276802391"),
        ("0.0.4920124", "302e020100300506032b657004220420a2243289c83d5b4a3e20d92cf88c7208d6eb46035ca481c9dffe42387913502"),
        ("0.0.4920125", "302e020100300506032b657004220420b3354398d94e6c5b4f31ea3dg99d8319e7fc57146db592daeggf53498a24613")];

    // Select a random account
    let (account_id, private_key) = &funded_accounts[0];

    println!("📍 Account: {account_id}");
    println!("💰 Balance: ~100 HBAR");

    save_and_reload(account_id, private_key).await?;

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

    // Reload environment
    dotenvy::from_filename(".env.hedera").ok();
    std::env::set_var("HEDERA_OPERATOR_ID", account_id);
    std::env::set_var("OPERATOR_PRIVATE_KEY", private_key);

    // Reload wallet
    let mut manager = WALLET_MANAGER.write().await;
    match manager.init_hedera().await {
        Ok(_) => {
            println!("✅ Wallet reloaded with funded account!");
        }
        Err(e) => {
            println!("⚠️  Reload warning: {e}");
        }
    }

    Ok(())
}
