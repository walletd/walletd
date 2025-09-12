use crate::hedera_funded_wallet::check_real_balance;

pub async fn handle_get_testnet_hbar() -> Result<(), String> {
    println!("\n💰 Hedera Testnet Status");
    println!("========================");

    match check_real_balance().await {
        Ok(balance) => {
            println!("✅ Using account: 0.0.7654321");
            println!("💰 Current balance: {balance} HBAR");

            if balance < 10.0 {
                println!("\n⚠️  Low balance! Visit https://portal.hedera.com to refill");
            }
        }
        Err(e) => {
            println!("❌ Error: {e}");
            println!("\n📝 Make sure you've added your private key to .env.hedera");
        }
    }

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    std::io::stdin()
        .read_line(&mut _input)
        .map_err(|e| e.to_string())?;
    Ok(())
}
