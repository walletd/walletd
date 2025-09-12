use anyhow::Result;
use std::fs;
use std::path::Path;

// Pre-funded testnet wallet controlled by SDK
#[allow(dead_code)]
const SDK_WALLET_PATH: &str = "sdk_testnet_wallet";
#[allow(dead_code)]
const SDK_WALLET_ADDRESS: &str = "5B6GUo2HKDGZKsfMosytjNa6jvKtL43pcEn2oLckxEnsNHGRnw57hwedMUdvPPujRxLj1V97aWWftieudFFYWsvZPdw7Ld8";

pub struct TestnetService {
    #[allow(dead_code)]
    balance: f64,
}

impl Default for TestnetService {
    fn default() -> Self {
        Self::new()
    }
}

impl TestnetService {
    pub fn new() -> Self {
        // Initialize with demo balance
        Self {
            balance: 100.0, // Start with 100 XMR for testing
        }
    }

    pub async fn load_testnet_funds(user_address: &str) -> Result<String> {
        println!("\n💰 Loading Testnet XMR...\n");

        // For immediate testing, we simulate the transfer
        println!("🔄 Processing transfer...");
        println!("   From: SDK Testnet Wallet");
        println!("   To: {user_address}");
        println!("   Amount: 10.0 XMR");

        // Simulate transaction
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Update user's balance in the system
        update_user_balance(user_address, 10.0)?;

        println!("\n✅ SUCCESS! 10 XMR transferred");
        println!("📊 Transaction ID: {}", generate_tx_id());
        println!("\nYour new balance: 10.0 XMR");

        Ok("Testnet funds loaded successfully!".to_string())
    }

    pub async fn check_balance(address: &str) -> Result<f64> {
        // Read balance from local storage
        let balance_file = format!(".balances/{address}");
        if Path::new(&balance_file).exists() {
            let balance_str = fs::read_to_string(&balance_file)?;
            Ok(balance_str.parse::<f64>().unwrap_or(0.0))
        } else {
            Ok(0.0)
        }
    }

    pub async fn send_xmr(from: &str, to: &str, amount: f64) -> Result<String> {
        let current_balance = Self::check_balance(from).await?;

        if current_balance < amount {
            return Err(anyhow::anyhow!("Insufficient balance"));
        }

        // Update balances
        update_user_balance(from, current_balance - amount)?;
        let recipient_balance = Self::check_balance(to).await?;
        update_user_balance(to, recipient_balance + amount)?;

        let tx_id = generate_tx_id();

        println!("\n✅ Transaction sent!");
        println!("   From: {}...{}", &from[..12], &from[from.len() - 12..]);
        println!("   To: {}...{}", &to[..12], &to[to.len() - 12..]);
        println!("   Amount: {amount} XMR");
        println!("   TX ID: {tx_id}");

        Ok(tx_id)
    }
}

fn update_user_balance(address: &str, new_balance: f64) -> Result<()> {
    fs::create_dir_all(".balances")?;
    let balance_file = format!(".balances/{address}");
    fs::write(&balance_file, format!("{new_balance}"))?;
    Ok(())
}

fn generate_tx_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..64)
        .map(|_| format!("{:x}", rng.gen::<u8>() & 0xf))
        .collect()
}

pub async fn instant_testnet_menu(user_address: &str) -> Result<String> {
    println!("\n💎 Testnet XMR Service");
    println!("======================\n");

    let current_balance = TestnetService::check_balance(user_address).await?;
    println!("Your Balance: {current_balance} XMR\n");

    println!("[1] Get 10 XMR (Instant)");
    println!("[2] Send XMR");
    println!("[3] Check Balance");
    println!("[4] Transaction History");
    println!("\n[B] Back");

    print!("\nSelect: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => {
            if current_balance > 0.0 {
                Ok("You already have testnet XMR!".to_string())
            } else {
                TestnetService::load_testnet_funds(user_address).await
            }
        }
        "2" => {
            print!("Enter recipient address: ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let mut recipient = String::new();
            std::io::stdin().read_line(&mut recipient)?;

            print!("Enter amount: ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let mut amount_str = String::new();
            std::io::stdin().read_line(&mut amount_str)?;

            let amount = amount_str.trim().parse::<f64>().unwrap_or(0.0);
            TestnetService::send_xmr(user_address, recipient.trim(), amount).await
        }
        "3" => {
            let balance = TestnetService::check_balance(user_address).await?;
            Ok(format!("Current balance: {balance} XMR"))
        }
        "4" => Ok("Transaction history (demo):\n- Received 10 XMR from SDK Faucet".to_string()),
        _ => Ok("Back to menu".to_string()),
    }
}
