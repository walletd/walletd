use crate::wallet_integration::WALLET_MANAGER;

const FUNDED_ACCOUNT: &str = "0.0.7654321";

pub async fn ensure_funded_account() -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;

    if let Some(wallet) = &manager.hedera {
        if let Some(account_id) = &wallet.account_id {
            if account_id != FUNDED_ACCOUNT {
                drop(manager);
                println!("🔄 Switching to funded account...");
                reload_funded_account().await?;
            }
        }
    }

    Ok(())
}

async fn reload_funded_account() -> Result<(), String> {
    dotenvy::from_filename(".env.hedera").ok();

    let mut manager = WALLET_MANAGER.write().await;
    manager
        .init_hedera()
        .await
        .map_err(|e| format!("Init failed: {e}"))?;

    Ok(())
}

pub async fn check_real_balance() -> Result<f64, String> {
    ensure_funded_account().await?;

    let manager = WALLET_MANAGER.read().await;

    if let Some(wallet) = &manager.hedera {
        match wallet.get_balance().await {
            Ok(balance) => Ok(balance),
            Err(e) => Err(format!("Failed to get balance: {e}")),
        }
    } else {
        Err("Wallet not initialized".to_string())
    }
}

pub async fn send_hbar_transaction(to: &str, amount: f64) -> Result<String, String> {
    ensure_funded_account().await?;

    let manager = WALLET_MANAGER.read().await;

    if let Some(wallet) = &manager.hedera {
        println!("📤 Sending {amount} HBAR from {FUNDED_ACCOUNT} to {to}");

        match wallet.send_hbar(to, amount).await {
            Ok(tx_id) => {
                println!("✅ Transaction successful!");
                println!("🔗 Transaction ID: {tx_id}");
                println!("🌐 View on HashScan: https://hashscan.io/testnet/transaction/{tx_id}");
                Ok(tx_id)
            }
            Err(e) => Err(format!("Transaction failed: {e}")),
        }
    } else {
        Err("Wallet not initialized".to_string())
    }
}

pub async fn create_token() -> Result<String, String> {
    ensure_funded_account().await?;

    let manager = WALLET_MANAGER.read().await;

    if let Some(_wallet) = &manager.hedera {
        println!("🪙 Creating new token on Hedera testnet...");

        // Token creation would go here
        // For now, return a mock token ID
        Ok("0.0.7654322".to_string())
    } else {
        Err("Wallet not initialized".to_string())
    }
}

pub async fn swap_tokens() -> Result<(), String> {
    ensure_funded_account().await?;

    println!("\n🔄 Token Swap (Coming Soon)");
    println!("This will integrate with Hedera DEXs like:");
    println!("- SaucerSwap");
    println!("- HeliSwap");
    println!("- Pangolin");

    Ok(())
}
