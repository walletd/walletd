use crate::wallet_integration::WALLET_MANAGER;
use crate::CliResponse;
use std::io::{self, Write};

pub async fn handle_testnet_menu() -> Result<CliResponse, String> {
    // Check if we're in testnet mode
    let manager = WALLET_MANAGER.read().await;

    // Check if we're in testnet mode by looking at the network configurations
    let _is_testnet = manager.config.bitcoin.network == "testnet" || !manager.config.demo_mode;

    if manager.config.demo_mode && manager.config.bitcoin.network != "testnet" {
        println!("\n⚠️  Not in testnet mode!");
        println!(
            "Current config shows demo_mode: {}",
            manager.config.demo_mode
        );
        println!("\nTo use real testnets, your walletd_config.json should have:");
        println!("\"demo_mode\": false");
        println!("And set networks to testnet:");
        println!("\"bitcoin\": {{ \"network\": \"testnet\", ... }}");
        drop(manager); // Release the lock

        println!("\nPress Enter to continue...");
        let mut _pause = String::new();
        io::stdin().read_line(&mut _pause).ok();
        return Ok(CliResponse::Continue);
    }

    drop(manager); // Release the lock before menu

    loop {
        println!("\n🧪 Testnet Testing Suite - LIVE CONNECTIONS");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Connected to real test networks!");

        println!("\n[1] Bitcoin Testnet Wallet");
        println!("[2] Ethereum Sepolia Wallet");
        println!("[3] Solana Devnet Wallet");
        println!("[4] Generate New Address");
        println!("[5] Check Balances");
        println!("[6] Send Test Transaction");

        println!("\n[F] Faucet Links");
        println!("[B] Back to Main Menu");

        print!("\nSelect option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        match input.trim().to_lowercase().as_str() {
            "1" => handle_bitcoin_testnet_wallet().await?,
            "2" => handle_ethereum_testnet_wallet().await?,
            "3" => handle_solana_testnet_wallet().await?,
            "4" => handle_generate_address().await?,
            "5" => handle_check_balances().await?,
            "6" => handle_send_test_transaction().await?,
            "f" => crate::testnet_integration::show_faucet_links(),
            "b" => return Ok(CliResponse::Continue),
            _ => println!("Invalid option"),
        }

        println!("\nPress Enter to continue...");
        let mut _pause = String::new();
        io::stdin().read_line(&mut _pause).ok();
    }
}

async fn handle_bitcoin_testnet_wallet() -> Result<(), String> {
    println!("\n🪙 Bitcoin Testnet Wallet");
    println!("━━━━━━━━━━━━━━━━━━━━━━");

    let manager = WALLET_MANAGER.read().await;

    if let Some(btc_manager) = &manager.bitcoin {
        // Generate a real testnet address
        match btc_manager.get_receive_address() {
            Ok(address) => {
                println!("✅ Your Bitcoin testnet address:");
                println!("📬 {address}");

                // Check balance
                match btc_manager.get_balance().await {
                    Ok(balance) => {
                        let btc = balance as f64 / 100_000_000.0;
                        println!("💰 Balance: {btc} tBTC");

                        if balance == 0 {
                            println!("\n💧 Get free tBTC from:");
                            println!("   https://coinfaucet.eu/en/btc-testnet/");
                        }
                    }
                    Err(e) => println!("⚠️  Balance check failed: {e}"),
                }

                println!("\n🔗 View on explorer:");
                println!("   https://blockstream.info/testnet/address/{address}");
            }
            Err(e) => {
                println!("❌ Failed to generate address: {e}");
            }
        }
    } else {
        println!("❌ Bitcoin testnet not initialized");
        println!("   Check your configuration");
    }

    Ok(())
}

async fn handle_ethereum_testnet_wallet() -> Result<(), String> {
    println!("\n🪙 Ethereum Sepolia Wallet");
    println!("━━━━━━━━━━━━━━━━━━━━━━━");

    let manager = WALLET_MANAGER.read().await;

    if let Some(eth_wallet) = &manager.ethereum {
        let address = eth_wallet.address;
        println!("✅ Your Ethereum Sepolia address:");
        println!("📬 {address}");

        // Note: Balance check would require provider connection
        println!("\n💧 Get free Sepolia ETH from:");
        println!("   https://sepoliafaucet.com/");

        println!("\n🔗 View on explorer:");
        println!("   https://sepolia.etherscan.io/address/{address}");
    } else {
        println!("❌ Ethereum wallet not initialized");
    }

    Ok(())
}

async fn handle_solana_testnet_wallet() -> Result<(), String> {
    println!("\n🪙 Solana Devnet Wallet");
    println!("━━━━━━━━━━━━━━━━━━━━");

    println!("📌 To use Solana devnet:");
    println!("1. Install Solana CLI");
    println!("2. Run: solana-keygen new");
    println!("3. Run: solana airdrop 2");

    Ok(())
}

async fn handle_generate_address() -> Result<(), String> {
    println!("\n🔑 Generate New Testnet Address");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("[1] Bitcoin testnet address");
    println!("[2] Ethereum address");
    println!("[3] Solana address");

    print!("\nSelect blockchain: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    match input.trim() {
        "1" => {
            let manager = WALLET_MANAGER.read().await;
            if let Some(btc) = &manager.bitcoin {
                match btc.get_receive_address() {
                    Ok(addr) => println!("✅ New Bitcoin testnet address: {addr}"),
                    Err(e) => println!("❌ Failed: {e}"),
                }
            }
        }
        "2" => {
            let wallet = walletd_ethereum::EthereumWallet::builder()
                .build()
                .map_err(|e| e.to_string())?;
            println!("✅ New Ethereum address: {}", wallet.address());
        }
        _ => println!("Invalid option"),
    }

    Ok(())
}

async fn handle_check_balances() -> Result<(), String> {
    println!("\n💰 Check Testnet Balances");
    println!("━━━━━━━━━━━━━━━━━━━━━━");

    let manager = WALLET_MANAGER.read().await;

    // Bitcoin
    if let Some(btc) = &manager.bitcoin {
        match btc.get_balance().await {
            Ok(balance) => {
                let btc_balance = balance as f64 / 100_000_000.0;
                println!("🪙 Bitcoin: {btc_balance} tBTC");
            }
            Err(e) => println!("🪙 Bitcoin: Error - {e}"),
        }
    }

    // Ethereum (would need provider)
    if let Some(_eth) = &manager.ethereum {
        println!("🪙 Ethereum: Check on https://sepolia.etherscan.io/");
    }

    Ok(())
}

async fn handle_send_test_transaction() -> Result<(), String> {
    println!("\n📤 Send Test Transaction");
    println!("━━━━━━━━━━━━━━━━━━━━");

    println!("⚠️  Make sure you have testnet tokens first!");
    println!("\n[1] Send Bitcoin testnet transaction");
    println!("[2] Send Ethereum Sepolia transaction");

    print!("\nSelect blockchain: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    match input.trim() {
        "1" => {
            println!("\n📝 Bitcoin testnet transaction:");
            println!("To: tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7");
            println!("Amount: 0.0001 tBTC");
            println!("Fee: ~0.00001 tBTC");

            // In real implementation, would call btc_manager.send_bitcoin()
            println!("\n(Transaction sending not implemented in demo)");
        }
        _ => println!("Not implemented yet"),
    }

    Ok(())
}
