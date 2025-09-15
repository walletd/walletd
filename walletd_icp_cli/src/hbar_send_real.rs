use crate::hedera_funded_wallet::{ensure_funded_account, send_hbar_transaction};
use std::io::{self, Write};

pub async fn handle_send_hedera_real() -> Result<(), String> {
    ensure_funded_account().await?;

    println!("\n=== Send HBAR ===");
    println!("From: 0.0.7654321 (your funded account)");

    print!("To account (e.g., 0.0.12345): ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut to_account = String::new();
    io::stdin()
        .read_line(&mut to_account)
        .map_err(|e| e.to_string())?;
    let to_account = to_account.trim();

    // Validate account format
    if !to_account.starts_with("0.0.") {
        return Err("Invalid account format. Use 0.0.XXXXX".to_string());
    }

    print!("Amount (HBAR): ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .map_err(|e| e.to_string())?;

    match amount.trim().parse::<f64>() {
        Ok(hbar_amount) => {
            if hbar_amount <= 0.0 || hbar_amount > 100.0 {
                println!("❌ Amount must be between 0 and 100 HBAR");
            } else {
                match send_hbar_transaction(to_account, hbar_amount).await {
                    Ok(_) => {
                        println!("\n✅ Transaction complete!");
                    }
                    Err(e) => {
                        println!("❌ Error: {e}");
                    }
                }
            }
        }
        Err(_) => {
            println!("❌ Invalid amount");
        }
    }

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}
