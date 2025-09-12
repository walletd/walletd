use crate::wallet_integration::WALLET_MANAGER;
use std::io::{self, Write};

pub async fn handle_send_solana_real() -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;

    if let Some(sol_wallet) = &manager.solana {
        // Check balance
        let balance_lamports = sol_wallet
            .get_balance()
            .await
            .map_err(|e| format!("Failed to get balance: {e}"))?;
        let balance_sol = balance_lamports as f64 / 1_000_000_000.0;

        println!("\n=== Send Solana ===");
        println!("Network: {} ", sol_wallet.cluster);
        println!("From: {}", sol_wallet.address);
        println!("Balance: {balance_sol} SOL ({balance_lamports} lamports)");

        if balance_lamports == 0 {
            println!("\n⚠️  Your wallet has 0 SOL!");
            println!("Please use option 3 to request an airdrop first.");
            return Ok(());
        }

        print!("To address: ");
        io::stdout().flush().unwrap();
        let mut to_address = String::new();
        io::stdin().read_line(&mut to_address).unwrap();
        let to_address = to_address.trim();

        print!("Amount (SOL): ");
        io::stdout().flush().unwrap();
        let mut amount_str = String::new();
        io::stdin().read_line(&mut amount_str).unwrap();
        let amount: f64 = amount_str.trim().parse().map_err(|_| "Invalid amount")?;

        let amount_lamports = (amount * 1_000_000_000.0) as u64;
        let fee_lamports = 5000; // 0.000005 SOL

        if amount_lamports + fee_lamports > balance_lamports {
            println!("\n❌ Insufficient funds!");
            println!("You have: {balance_sol} SOL");
            println!("You need: {amount} SOL + 0.000005 SOL for fees");
            return Ok(());
        }

        println!("\n📋 Transaction Summary:");
        println!("From: {}", sol_wallet.address);
        println!("To: {to_address}");
        println!("Amount: {amount} SOL ({amount_lamports} lamports)");
        println!("Network: {}", sol_wallet.cluster);
        println!("Fee: ~0.000005 SOL");

        print!("\nConfirm? (yes/no): ");
        io::stdout().flush().unwrap();
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).unwrap();

        if confirm.trim().to_lowercase() == "yes" {
            match sol_wallet.send_transaction(to_address, amount).await {
                Ok(signature) => {
                    println!("\n✅ Transaction prepared!");
                    println!("Signature: {signature}");
                    println!(
                        "\n⚠️  Note: Full transaction signing requires Solana SDK integration."
                    );
                    println!("Your private key and transaction details are ready for use with:");
                    println!(
                        "- Solana CLI: solana transfer {to_address} {amount} --keypair <your-keypair-file>"
                    );
                    println!("- Phantom Wallet: Import your private key");
                    println!("- Solana Web3.js: Use in your own application");
                }
                Err(e) => {
                    println!("\n❌ Transaction failed: {e}");
                }
            }
        } else {
            println!("Transaction cancelled.");
        }
    } else {
        return Err("Solana wallet not initialized".to_string());
    }

    Ok(())
}

pub async fn handle_solana_airdrop() -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;

    if let Some(sol_wallet) = &manager.solana {
        println!("\n=== Request Solana Airdrop ===");
        println!("Address: {}", sol_wallet.address);
        println!("Network: {}", sol_wallet.cluster);

        if sol_wallet.cluster != "devnet" {
            println!("\n⚠️  Airdrop is only available on devnet!");
            return Ok(());
        }

        // Check current balance first
        match sol_wallet.get_balance().await {
            Ok(balance) => {
                let sol = balance as f64 / 1_000_000_000.0;
                println!("Current balance: {sol} SOL");
            }
            Err(_) => {
                println!("Current balance: Unknown");
            }
        }

        print!("\nAmount in SOL (max 2): ");
        io::stdout().flush().unwrap();
        let mut amount_str = String::new();
        io::stdin().read_line(&mut amount_str).unwrap();
        let amount: f64 = amount_str.trim().parse().map_err(|_| "Invalid amount")?;

        if amount > 2.0 {
            println!("⚠️  Maximum airdrop is 2 SOL");
            return Ok(());
        }

        println!("\n💧 Requesting {amount} SOL airdrop...");

        match sol_wallet.request_airdrop(amount).await {
            Ok(signature) => {
                println!("\n✅ Airdrop successful!");
                println!("Transaction signature: {signature}");
                println!("\n🔍 View on Solana Explorer:");
                println!("https://explorer.solana.com/tx/{signature}?cluster=devnet");

                // Check new balance
                println!("\n⏳ Checking new balance...");
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                if let Ok(new_balance) = sol_wallet.get_balance().await {
                    let sol = new_balance as f64 / 1_000_000_000.0;
                    println!("💰 New balance: {sol} SOL");
                }
            }
            Err(e) => {
                println!("\n❌ Airdrop failed: {e}");
                println!("\n💡 Troubleshooting tips:");
                println!("1. Try a smaller amount (0.5 or 1 SOL)");
                println!("2. Wait a few seconds and try again");
                println!("3. Use the web faucet: https://faucet.solana.com/");
                println!("   - Enter your address: {}", sol_wallet.address);
                println!("   - Select 'Devnet'");
                println!("   - Request airdrop");
            }
        }
    } else {
        return Err("Solana wallet not initialized".to_string());
    }

    Ok(())
}
