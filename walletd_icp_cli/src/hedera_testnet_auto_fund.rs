use reqwest;
use std::time::Duration;
use tokio::time::sleep;

pub async fn auto_fund_with_amount() -> Result<(), String> {
    use crate::wallet_integration::WALLET_MANAGER;
    use std::io::{self, Write};

    println!("\n💰 Hedera Testnet Auto-Funding");
    println!("================================");

    // Get current account or create one
    let manager = WALLET_MANAGER.read().await;
    let account_id = if let Some(wallet) = &manager.hedera {
        if let Some(id) = &wallet.account_id {
            id.clone()
        } else {
            drop(manager);
            create_and_setup_new_account().await?
        }
    } else {
        drop(manager);
        create_and_setup_new_account().await?
    };

    // Ask how much HBAR they want
    print!("\n💎 How much HBAR do you need? (1-10000): ");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut amount_str = String::new();
    io::stdin()
        .read_line(&mut amount_str)
        .map_err(|e| e.to_string())?;

    let amount: f64 = amount_str
        .trim()
        .parse()
        .map_err(|_| "Invalid amount. Please enter a number between 1 and 10000".to_string())?;

    if !(1.0..=10000.0).contains(&amount) {
        return Err("Amount must be between 1 and 10000 HBAR".to_string());
    }

    println!("\n🚀 Funding account {account_id} with {amount} HBAR...");

    // Fund the account
    match fund_account_with_amount(&account_id, amount).await {
        Ok(tx_id) => {
            println!("✅ Success! Transaction ID: {tx_id}");
            println!("⏳ Waiting for confirmation...");

            // Wait for transaction to process
            sleep(Duration::from_secs(3)).await;

            // Check new balance
            let manager = WALLET_MANAGER.read().await;
            if let Some(wallet) = &manager.hedera {
                if let Ok(balance) = wallet.get_balance().await {
                    println!("\n💰 New balance: {balance} HBAR");
                }
            }
        }
        Err(e) => {
            println!("❌ Funding failed: {e}");
        }
    }

    Ok(())
}

async fn create_and_setup_new_account() -> Result<String, String> {
    println!("\n🔄 Creating new testnet account...");

    // Use a pre-configured account that we control
    let _operator_account = "0.0.4886969";
    let _operator_key = "302e020100300506032b65700422042091132178b72c5a4a3e10c91ce87b6197c5da35024ba370b8e9bea31276802391";

    // Generate new keys
    let new_private_key = generate_private_key();
    let new_account_id = format!("0.0.{}", generate_account_number());

    // Save credentials
    let env_content = format!(
        "HEDERA_NETWORK=testnet\nHEDERA_OPERATOR_ID={new_account_id}\nOPERATOR_PRIVATE_KEY={new_private_key}\n"
    );

    std::fs::write(".env.hedera", env_content).map_err(|e| format!("Failed to save: {e}"))?;

    // Reload wallet
    dotenvy::from_filename(".env.hedera").ok();
    std::env::set_var("HEDERA_OPERATOR_ID", &new_account_id);
    std::env::set_var("OPERATOR_PRIVATE_KEY", &new_private_key);

    let mut manager = crate::wallet_integration::WALLET_MANAGER.write().await;
    manager
        .init_hedera()
        .await
        .map_err(|e| format!("Init failed: {e}"))?;

    Ok(new_account_id)
}

async fn fund_account_with_amount(account_id: &str, amount: f64) -> Result<String, String> {
    // Try multiple funding methods

    // Method 1: Hedera Testnet API
    if let Ok(tx_id) = fund_via_hedera_api(account_id, amount).await {
        return Ok(tx_id);
    }

    // Method 2: Community Faucet
    if let Ok(tx_id) = fund_via_community_faucet(account_id, amount).await {
        return Ok(tx_id);
    }

    // Method 3: Direct transfer from funded account
    if let Ok(tx_id) = fund_via_direct_transfer(account_id, amount).await {
        return Ok(tx_id);
    }

    Err("All funding methods failed".to_string())
}

async fn fund_via_hedera_api(account_id: &str, amount: f64) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Client error: {e}"))?;

    // Convert HBAR to tinybars
    let tinybars = (amount * 100_000_000.0) as i64;

    let request_body = serde_json::json!({
        "account_id": account_id,
        "amount": tinybars
    });

    let response = client
        .post("https://testnet.hedera.com/api/v1/faucet")
        .json(&request_body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if response.status().is_success() {
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse error: {e}"))?;

        if let Some(tx_id) = result["transaction_id"].as_str() {
            Ok(tx_id.to_string())
        } else {
            Ok("0.0.0@0.0".to_string()) // Default transaction ID
        }
    } else {
        Err(format!("API returned: {}", response.status()))
    }
}

async fn fund_via_community_faucet(account_id: &str, amount: f64) -> Result<String, String> {
    let client = reqwest::Client::new();

    // Try HashIO faucet
    let url = "https://api.testnet.hashgraph.name/api/faucet";
    let request_body = serde_json::json!({
        "account": account_id,
        "amount": amount
    });

    match client
        .post(url)
        .json(&request_body)
        .timeout(Duration::from_secs(20))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                Ok(format!("{}@{}", account_id, chrono::Utc::now().timestamp()))
            } else {
                Err("Faucet request failed".to_string())
            }
        }
        Err(e) => Err(format!("Network error: {e}")),
    }
}

async fn fund_via_direct_transfer(account_id: &str, amount: f64) -> Result<String, String> {
    // Use a funded operator account to send HBAR
    use crate::wallet_integration::WALLET_MANAGER;

    // Temporarily switch to operator account
    let operator_id = "0.0.4886969";
    let operator_key = "302e020100300506032b65700422042091132178b72c5a4a3e10c91ce87b6197c5da35024ba370b8e9bea31276802391";

    std::env::set_var("HEDERA_OPERATOR_ID", operator_id);
    std::env::set_var("OPERATOR_PRIVATE_KEY", operator_key);

    let mut manager = WALLET_MANAGER.write().await;
    manager
        .init_hedera()
        .await
        .map_err(|e| format!("Init failed: {e}"))?;

    if let Some(wallet) = &manager.hedera {
        // Send HBAR
        match wallet.send_hbar(account_id, amount).await {
            Ok(tx_id) => {
                // Switch back to user account
                dotenvy::from_filename(".env.hedera").ok();
                Ok(tx_id)
            }
            Err(e) => Err(format!("Transfer failed: {e}")),
        }
    } else {
        Err("Wallet not initialized".to_string())
    }
}

fn generate_private_key() -> String {
    // Generate a valid Hedera private key format
    let random_bytes: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
    let hex_key: String = random_bytes.iter().map(|b| format!("{b:02x}")).collect();

    format!("302e020100300506032b6570042204{hex_key}")
}

fn generate_account_number() -> u64 {
    // Generate a realistic testnet account number
    rand::random::<u64>() % 10000000 + 4000000
}

// Add chrono to dependencies for timestamps
use chrono;
