use crate::hedera_testnet_simulator::HederaTestnetSimulator;
use anyhow::Result;
use std::io::{self, Write};

pub async fn handle_send_hedera_testnet(from_account: &str) -> Result<()> {
    println!("\n=== Send HBAR (Testnet) ===");

    let balance = HederaTestnetSimulator::get_balance(from_account);
    println!("From: {from_account}");
    println!("Balance: {balance} HBAR");

    if balance == 0.0 {
        println!("\n⚠️  Your balance is 0. Getting testnet HBAR...");
        HederaTestnetSimulator::create_account(from_account)?;
        println!("✅ Added 10,000 testnet HBAR to your account!");
        let new_balance = HederaTestnetSimulator::get_balance(from_account);
        println!("New balance: {new_balance} HBAR");
    }

    print!("\nRecipient Account ID: ");
    io::stdout().flush()?;
    let mut to_account = String::new();
    io::stdin().read_line(&mut to_account)?;
    let to_account = to_account.trim();

    // Validate account format
    if !to_account.starts_with("0.0.") {
        println!("⚠️  Invalid account format. Use format: 0.0.XXXXX");
        return Ok(());
    }

    print!("Amount (HBAR): ");
    io::stdout().flush()?;
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str)?;
    let amount: f64 = amount_str
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid amount"))?;

    if amount <= 0.0 {
        println!("⚠️  Amount must be positive");
        return Ok(());
    }

    println!("\n📋 Transaction Summary:");
    println!("From: {from_account}");
    println!("To: {to_account}");
    println!("Amount: {amount} HBAR");
    println!("Fee: 0.001 HBAR");
    println!("Total: {} HBAR", amount + 0.001);

    print!("\nConfirm? (yes/no): ");
    io::stdout().flush()?;
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;

    if confirm.trim().to_lowercase() != "yes" {
        println!("Transaction cancelled.");
        return Ok(());
    }

    // Execute transfer
    match HederaTestnetSimulator::transfer(from_account, to_account, amount) {
        Ok(tx_id) => {
            println!("\n✅ Transaction successful!");
            println!("Transaction ID: {tx_id}");
            println!("View on HashScan: https://hashscan.io/testnet/transaction/{tx_id}");

            let new_balance = HederaTestnetSimulator::get_balance(from_account);
            println!("\nNew balance: {new_balance} HBAR");
        }
        Err(e) => {
            println!("\n❌ Transaction failed: {e}");
        }
    }

    Ok(())
}
