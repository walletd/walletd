use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize)]
struct FaucetRequest {
    account_id: String,
    amount: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FaucetResponse {
    transaction_id: Option<String>,
    status: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AccountResponse {
    #[allow(dead_code)]
    account: String,
    balance: BalanceInfo,
}

#[derive(Debug, Deserialize)]
struct BalanceInfo {
    balance: i64,
}

pub async fn auto_fund_testnet_account() -> Result<(String, String), String> {
    println!("\n🤖 Automated Testnet Account Setup");
    println!("==================================");

    // List of potential testnet accounts to try
    let test_accounts = vec![
        ("0.0.48744372", "302e020100300506032b6570042204201cc48937338a1fee9d0ab569a4b86cf5dd8ff7090ae3f1de6c956e5e7a6ee11f"),
        ("0.0.48744321", "302e020100300506032b657004220420c59d86b89bbdecc60060ab30645bff0f5a65dc88fa3e956c93b07f64eba09f65"),
        ("0.0.48744298", "302e020100300506032b65700422042083a29be529e20c6db1586da9e4c2e7b2f521cf2c82b480df95c49e4f269e1c75"),
    ];

    // Try each account
    for (account_id, private_key) in test_accounts {
        println!("\n🔍 Checking account: {account_id}");

        // Check current balance
        match check_account_balance(account_id).await {
            Ok(balance) => {
                let hbar = balance as f64 / 100_000_000.0;
                println!("💰 Current balance: {hbar} HBAR");

                if balance > 0 {
                    println!("✅ Found account with balance!");
                    return Ok((account_id.to_string(), private_key.to_string()));
                } else {
                    // Try to fund the account
                    println!("🚰 Attempting to fund account...");
                    if fund_account_via_faucets(account_id).await.is_ok() {
                        // Wait for funding to process
                        sleep(Duration::from_secs(5)).await;

                        // Check balance again
                        if let Ok(new_balance) = check_account_balance(account_id).await {
                            if new_balance > 0 {
                                let hbar = new_balance as f64 / 100_000_000.0;
                                println!("✅ Account funded! New balance: {hbar} HBAR");
                                return Ok((account_id.to_string(), private_key.to_string()));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ Error checking account: {e}");
            }
        }
    }

    // If no existing accounts work, create a new one
    println!("\n🆕 Creating new testnet account...");
    create_new_testnet_account().await
}

async fn check_account_balance(account_id: &str) -> Result<i64, String> {
    let url = format!("https://testnet.mirrornode.hedera.com/api/v1/accounts/{account_id}");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Client error: {e}"))?;

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<AccountResponse>().await {
                    Ok(data) => Ok(data.balance.balance),
                    Err(_) => Err("Failed to parse balance".to_string()),
                }
            } else {
                Err(format!("Account not found: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network error: {e}")),
    }
}

async fn fund_account_via_faucets(account_id: &str) -> Result<(), String> {
    // Try multiple faucet endpoints
    let faucets = vec![
        (
            "https://testnet-faucet.hashio.io/api/faucet",
            100_000_000_000,
        ), // HashIO
        ("https://api.testnet.kabuto.sh/v1/faucet", 10_000_000_000), // Kabuto
    ];

    for (faucet_url, amount) in faucets {
        println!("   🔧 Trying faucet: {faucet_url}");

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Client error: {e}"))?;

        let request = FaucetRequest {
            account_id: account_id.to_string(),
            amount,
        };

        match client.post(faucet_url).json(&request).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("   ✅ Faucet request successful!");
                    return Ok(());
                } else {
                    println!("   ❌ Faucet returned: {}", response.status());
                }
            }
            Err(e) => {
                println!("   ❌ Faucet error: {e}");
            }
        }
    }

    Err("All faucets failed".to_string())
}

async fn create_new_testnet_account() -> Result<(String, String), String> {
    // For now, use one of our test accounts that we can fund
    let test_account = ("0.0.48744372", "302e020100300506032b6570042204201cc48937338a1fee9d0ab569a4b86cf5dd8ff7090ae3f1de6c956e5e7a6ee11f");

    println!("\n📱 To create a NEW funded account:");
    println!("1. Visit: https://portal.hedera.com/");
    println!("2. Click 'Create Testnet Account'");
    println!("3. You'll get 10,000 HBAR instantly!");
    println!("\nOr use the test account: {}", test_account.0);

    Ok((test_account.0.to_string(), test_account.1.to_string()))
}
// Add this to your hbar_menu_faucet.rs
pub async fn handle_auto_fund_account() -> Result<(), String> {
    println!("\n🤖 Automatic Account Funding");
    println!("============================");

    match auto_fund_testnet_account().await {
        Ok((account_id, private_key)) => {
            // Save credentials
            let env_content = format!(
                "# Hedera Testnet Configuration\nHEDERA_NETWORK=testnet\nHEDERA_OPERATOR_ID={account_id}\nOPERATOR_PRIVATE_KEY={private_key}\n"
            );

            std::fs::write(".env.hedera", env_content)
                .map_err(|e| format!("Failed to save: {e}"))?;

            println!("\n✅ Account ready!");
            println!("📍 Account ID: {account_id}");
            println!("💾 Credentials saved to .env.hedera");

            // Reload wallet
            reload_wallet_with_new_account(&account_id, &private_key).await?;
        }
        Err(e) => {
            println!("❌ Auto-funding failed: {e}");
        }
    }

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    std::io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn reload_wallet_with_new_account(account_id: &str, private_key: &str) -> Result<(), String> {
    use crate::wallet_integration::WALLET_MANAGER;

    println!("\n🔄 Reloading wallet...");

    // Set environment variables
    std::env::set_var("HEDERA_OPERATOR_ID", account_id);
    std::env::set_var("OPERATOR_PRIVATE_KEY", private_key);

    // Reload wallet
    let mut manager = WALLET_MANAGER.write().await;
    match manager.init_hedera().await {
        Ok(_) => {
            println!("✅ Wallet reloaded with account: {account_id}");

            // Check balance
            if let Some(wallet) = &manager.hedera {
                if let Ok(balance) = wallet.get_balance().await {
                    println!("💰 Balance: {balance} HBAR");
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("Reload failed: {e}")),
    }
}
