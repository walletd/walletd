use crate::wallet_integration::WALLET_MANAGER;
use std::io::{self, Write};

pub async fn handle_send_monero_real() -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;

    if let Some(xmr_wallet) = &manager.monero {
        let balance = xmr_wallet
            .get_balance()
            .await
            .map_err(|e| format!("Failed to get balance: {e}"))?;
        let balance_xmr = balance as f64 / 1_000_000_000_000.0;

        println!("\n=== Send Monero ===");
        println!("Network: {}", xmr_wallet.network);
        println!("From: {}", xmr_wallet.address);
        println!("Balance: {balance_xmr} XMR");

        if balance == 0 {
            println!("\n⚠️  Your wallet has 0 XMR!");
            println!("To get stagenet XMR:");
            println!("1. Visit: https://community.xmr.to/faucet/stagenet/");
            println!("2. Enter your address: {}", xmr_wallet.address);
            println!("3. Wait for XMR (may take 10-20 minutes)");
            return Ok(());
        }

        print!("To address: ");
        io::stdout().flush().unwrap();
        let mut to_address = String::new();
        io::stdin().read_line(&mut to_address).unwrap();
        let to_address = to_address.trim();

        print!("Amount (XMR): ");
        io::stdout().flush().unwrap();
        let mut amount_str = String::new();
        io::stdin().read_line(&mut amount_str).unwrap();
        let amount: f64 = amount_str.trim().parse().map_err(|_| "Invalid amount")?;

        println!("\n📋 Transaction Summary:");
        println!("From: {}", xmr_wallet.address);
        println!("To: {to_address}");
        println!("Amount: {amount} XMR");
        println!("Network: {}", xmr_wallet.network);
        println!("Ring Size: 11 (default)");

        print!("\nConfirm? (yes/no): ");
        io::stdout().flush().unwrap();
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).unwrap();

        if confirm.trim().to_lowercase() == "yes" {
            match xmr_wallet.send_transaction(to_address, amount).await {
                Ok(_) => {}
                Err(e) => {
                    println!("\n{e}");
                }
            }
        } else {
            println!("Transaction cancelled.");
        }
    } else {
        return Err("Monero wallet not initialized".to_string());
    }

    Ok(())
}
