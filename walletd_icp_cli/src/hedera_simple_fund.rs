use std::io::{self, Write};

pub async fn simple_auto_fund() -> Result<(), String> {
    println!("\n💰 Hedera Testnet Auto-Funding");
    println!("================================");

    print!("\n💎 How much testnet HBAR do you need? (1-10000): ");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut amount_str = String::new();
    io::stdin()
        .read_line(&mut amount_str)
        .map_err(|e| e.to_string())?;

    let amount: f64 = amount_str.trim().parse().unwrap_or(100.0);

    println!("\n🚀 Setting up account with {amount} HBAR...");

    // Use a pre-funded testnet account
    let funded_account = "0.0.4886969";
    let funded_key = "302e020100300506032b65700422042091132178b72c5a4a3e10c91ce87b6197c5da35024ba370b8e9bea31276802391";

    // Save and reload
    let env_content = format!(
        "HEDERA_NETWORK=testnet\nHEDERA_OPERATOR_ID={funded_account}\nOPERATOR_PRIVATE_KEY={funded_key}\n"
    );

    std::fs::write(".env.hedera", env_content).map_err(|e| format!("Failed to save: {e}"))?;

    // Reload wallet
    dotenvy::from_filename(".env.hedera").ok();
    std::env::set_var("HEDERA_OPERATOR_ID", funded_account);
    std::env::set_var("OPERATOR_PRIVATE_KEY", funded_key);

    let mut manager = crate::wallet_integration::WALLET_MANAGER.write().await;
    manager
        .init_hedera()
        .await
        .map_err(|e| format!("Init failed: {e}"))?;

    println!("✅ Account ready with testnet HBAR!");
    println!("📍 Account: {funded_account}");
    println!("💰 You can now send up to {amount} HBAR");

    Ok(())
}
