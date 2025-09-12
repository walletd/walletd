use crate::types::CliResponse;
use std::io::{self, Write};

pub async fn handle_hbar_menu(
    _wallet_api: &mut crate::types::WalletDIcpApi,
    _hbar_address: &str,
    _hbar_balance: &str,
) -> Result<CliResponse, String> {
    use crate::wallet_integration::WALLET_MANAGER;

    loop {
        // Get real data from WALLET_MANAGER - re-acquire lock each iteration
        let manager = WALLET_MANAGER.read().await;

        let (account_id, real_balance) = if let Some(wallet) = &manager.hedera {
            let account = wallet
                .account_id
                .as_ref()
                .unwrap_or(&"Not configured".to_string())
                .clone();

            // Get real balance
            println!(
                "DEBUG: Calling get_balance for account {:?}",
                wallet.account_id
            );
            let balance = (wallet.get_balance().await).unwrap_or(0.0);

            (account, balance)
        } else {
            ("Not initialized".to_string(), 0.0)
        };

        // Drop the lock before menu interaction
        drop(manager);

        let is_mock = std::env::var("HEDERA_MOCK_MODE").is_ok();
        if is_mock {
            println!("\n⚠️  MOCK MODE - Not connected to real testnet");
        }
        println!("\n========== HEDERA WALLET (TESTNET) ==========");
        println!("Account: {account_id}");
        println!("Balance: {real_balance:.8} HBAR");
        println!("============================================");

        println!("\n[1] View Account Info");
        println!("[2] Check Balance (Real-time)");
        println!("[3] Get Testnet HBAR");
        println!("[4] Send HBAR");
        println!("[E] Exchange/Swap Tokens");
        println!("[W] Open Hedera Portal (Web)");
        println!("[R] Reload Wallet (after changing credentials)");
        println!("\n[B] Back to Main Menu");
        println!("[X] Exit");

        print!("\nSelect option: ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .map_err(|e| e.to_string())?;

        match choice.trim() {
            "1" => view_info().await?,
            "2" => check_real_balance().await?,
            "3" => get_testnet_hbar().await?,
            "4" => send_hbar().await?,
            "E" | "e" => handle_exchange().await?,
            "5" => view_history().await?,
            "W" | "w" => open_hedera_portal()?,
            "R" | "r" => reload_wallet().await?,
            "B" | "b" => return Ok(CliResponse::Continue),
            "X" | "x" => return Ok(CliResponse::Exit),
            _ => println!("Invalid option"),
        }
    }
}

async fn view_info() -> Result<(), String> {
    use crate::wallet_integration::WALLET_MANAGER;
    let manager = WALLET_MANAGER.read().await;

    if let Some(wallet) = &manager.hedera {
        println!("\n=== Account Information ===");
        println!(
            "Account: {}",
            wallet.account_id.as_ref().unwrap_or(&"Not set".to_string())
        );
        println!("Public Key: {}", wallet.public_key);

        if let Some(account_id) = &wallet.account_id {
            println!("\nView on HashScan:");
            println!("https://hashscan.io/testnet/account/{account_id}");
        }
    }

    println!("\nPress Enter to continue...");
    io::stdin()
        .read_line(&mut String::new())
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn check_real_balance() -> Result<(), String> {
    use crate::wallet_integration::WALLET_MANAGER;
    let manager = WALLET_MANAGER.read().await;

    if let Some(wallet) = &manager.hedera {
        println!("\n🔄 Checking real balance on Hedera testnet...");

        println!(
            "DEBUG: Calling get_balance for account {:?}",
            wallet.account_id
        );
        match wallet.get_balance().await {
            Ok(balance) => {
                println!("✅ Balance: {balance} HBAR");

                // Also check via API
                if let Some(account_id) = &wallet.account_id {
                    let url = format!(
                        "https://testnet.mirrornode.hedera.com/api/v1/accounts/{account_id}"
                    );
                    println!("\n🔍 Verifying via mirror node...");
                    println!("Check: {url}");
                }
            }
            Err(e) => {
                println!("❌ Error: {e}");
            }
        }
    }

    println!("\nPress Enter to continue...");
    io::stdin()
        .read_line(&mut String::new())
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn get_testnet_hbar() -> Result<(), String> {
    // Call the existing faucet handler
    crate::hbar_menu_faucet::handle_get_testnet_hbar().await
}

async fn send_hbar() -> Result<(), String> {
    // Call the existing send handler
    crate::hbar_send_real::handle_send_hedera_real().await
}

async fn view_history() -> Result<(), String> {
    use crate::wallet_integration::WALLET_MANAGER;
    let manager = WALLET_MANAGER.read().await;

    if let Some(wallet) = &manager.hedera {
        if let Some(account_id) = &wallet.account_id {
            println!("\n=== Transaction History ===");
            println!("View on HashScan:");
            println!("https://hashscan.io/testnet/account/{account_id}/transactions");
        }
    }

    println!("\nPress Enter to continue...");
    io::stdin()
        .read_line(&mut String::new())
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn reload_wallet() -> Result<(), String> {
    use crate::wallet_integration::WALLET_MANAGER;

    println!("\n🔄 Reloading wallet...");

    // Force reload environment
    dotenvy::from_filename(".env.hedera").ok();

    // Get write access to update wallet
    let mut manager = WALLET_MANAGER.write().await;

    // Reinitialize Hedera wallet
    match manager.init_hedera().await {
        Ok(_) => {
            println!("✅ Wallet refreshed successfully");

            // Show new balance if available
            if let Some(wallet) = &manager.hedera {
                if let Ok(balance) = wallet.get_balance().await {
                    println!("💰 Current balance: {balance} HBAR");
                }
            }
        }
        Err(e) => println!("❌ Failed to refresh: {e}"),
    }

    println!("\nPress Enter to continue...");
    io::stdin()
        .read_line(&mut String::new())
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn open_hedera_portal() -> Result<(), String> {
    println!("\n🌐 Opening Hedera Portal...");

    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg("https://portal.hedera.com/")
        .spawn()
        .map_err(|e| format!("Failed to open browser: {e}"))?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg("https://portal.hedera.com/")
        .spawn()
        .map_err(|e| format!("Failed to open browser: {}", e))?;

    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(&["/C", "start", "https://portal.hedera.com/"])
        .spawn()
        .map_err(|e| format!("Failed to open browser: {}", e))?;

    println!("✅ Opened portal.hedera.com in your browser");
    println!("   Create account there for 10,000 HBAR");

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn handle_exchange() -> Result<(), String> {
    use crate::hedera_funded_wallet::swap_tokens;

    println!("\n💱 Hedera Token Exchange");
    println!("========================");

    println!("\nAvailable options:");
    println!("[1] Swap HBAR for tokens");
    println!("[2] Create new token");
    println!("[3] View DEX integrations");

    print!("\nYour choice: ");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .map_err(|e| e.to_string())?;

    match choice.trim() {
        "1" => swap_tokens().await?,
        "2" => {
            use crate::hedera_funded_wallet::create_token;
            match create_token().await {
                Ok(token_id) => println!("✅ Token created: {token_id}"),
                Err(e) => println!("❌ Error: {e}"),
            }
        }
        "3" => {
            println!("\n🏪 Hedera DEX Integrations:");
            println!("- SaucerSwap: https://saucerswap.finance");
            println!("- HeliSwap: https://heliswap.io");
            println!("- Pangolin: https://pangolin.hedera.com");
        }
        _ => println!("Invalid choice"),
    }

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}
