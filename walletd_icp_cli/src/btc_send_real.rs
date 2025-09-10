use crate::wallet_integration::WALLET_MANAGER;
use std::io::{self, Write};

pub async fn handle_send_bitcoin_real(_user_id: &str) -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;

    if let Some(btc_wallet) = &manager.bitcoin {
        // First check the balance
        let balance = btc_wallet
            .get_balance()
            .await
            .map_err(|e| format!("Failed to get balance: {e}"))?;

        if balance == 0 {
            println!("\n⚠️  Your wallet has 0 BTC!");
            println!("To get testnet Bitcoin:");
            println!("1. Copy your address: {}", btc_wallet.address);
            println!("2. Visit: https://coinfaucet.eu/en/btc-testnet/");
            println!("3. Paste your address and get free tBTC");
            println!("4. Wait a few minutes for confirmation");
            return Ok(());
        }

        println!("\n=== Send Bitcoin ===");
        print!("To address: ");
        io::stdout().flush().unwrap();
        let mut to_address = String::new();
        io::stdin().read_line(&mut to_address).unwrap();
        let to_address = to_address.trim();

        print!("Amount (BTC): ");
        io::stdout().flush().unwrap();
        let mut amount_str = String::new();
        io::stdin().read_line(&mut amount_str).unwrap();
        let amount: f64 = amount_str.trim().parse().map_err(|_| "Invalid amount")?;

        let amount_sats = (amount * 100_000_000.0) as u64;

        print!("Fee rate (sat/vB, or Enter for auto): ");
        io::stdout().flush().unwrap();
        let mut fee_str = String::new();
        io::stdin().read_line(&mut fee_str).unwrap();

        println!("\n📋 Transaction Summary:");
        println!("From: {}", btc_wallet.address);
        println!("To: {to_address}");
        println!("Amount: {amount} BTC ({amount_sats} sats)");
        println!(
            "Available: {} BTC ({} sats)",
            balance as f64 / 100_000_000.0,
            balance
        );
        println!("Fee: ~10,000 sats (0.0001 BTC)");

        if amount_sats + 10000 > balance {
            println!("\n❌ Insufficient funds!");
            println!(
                "You need at least {} sats but only have {} sats",
                amount_sats + 10000,
                balance
            );
            return Ok(());
        }

        print!("\nConfirm? (yes/no): ");
        io::stdout().flush().unwrap();
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).unwrap();

        if confirm.trim().to_lowercase() == "yes" {
            println!("\n📡 Creating and broadcasting transaction...");

            // ACTUALLY SEND THE TRANSACTION
            match btc_wallet
                .create_and_send_transaction(to_address, amount_sats)
                .await
            {
                Ok(txid) => {
                    println!("\n✅ TRANSACTION BROADCAST SUCCESSFULLY!");
                    println!("Transaction ID: {txid}");
                    println!("\n🔍 View on Testnet Explorer:");
                    println!("https://blockstream.info/testnet/tx/{txid}");
                    println!("\n📊 Transaction Details:");
                    println!("- From: {}", btc_wallet.address);
                    println!("- To: {to_address}");
                    println!("- Amount: {amount} BTC");
                    println!("- Network: Bitcoin Testnet");
                }
                Err(e) => {
                    println!("\n❌ Transaction failed: {e}");
                    if e.to_string().contains("No UTXOs available") {
                        println!("\n💡 Your wallet needs funding first!");
                        println!(
                            "Get free testnet Bitcoin from: https://coinfaucet.eu/en/btc-testnet/"
                        );
                        println!("Your address: {}", btc_wallet.address);
                    } else if e.to_string().contains("Insufficient funds") {
                        println!("\n💡 Not enough Bitcoin to cover amount + fees");
                    }
                }
            }
        } else {
            println!("Transaction cancelled.");
        }
    } else {
        return Err("Bitcoin wallet not initialized".to_string());
    }

    Ok(())
}
