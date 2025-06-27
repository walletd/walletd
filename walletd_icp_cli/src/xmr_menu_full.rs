use crate::cli_types::CliResponse;
use crate::WALLET_MANAGER;
use std::io::{self, Write};

pub async fn handle_monero_menu() -> Result<CliResponse, String> {
    loop {
        let manager = WALLET_MANAGER.read().await;
        let wallet = manager.monero.as_ref()
            .ok_or("Monero wallet not initialized")?;
        
        // Get network info
        let height = wallet.get_blockchain_height().await.unwrap_or(0);
        let balance = wallet.get_balance().await.unwrap_or(0);
        
        println!("\n========== MONERO WALLET (STAGENET) ==========");
        println!("Address: {}...{}", &wallet.address[..12], &wallet.address[wallet.address.len()-12..]);
        println!("Balance: {} XMR", balance as f64 / 1e12);
        println!("Height: {}", height);
        println!("Network: {}", wallet.network.to_uppercase());
        println!("==============================================");
        
        println!("\n--- Quick Actions ---");
        println!("[1] Check Balance");
        println!("[2] Show Full Address");
        println!("[3] Send XMR");
        println!("[4] Receive (QR Code)");
        println!("[5] Transaction History");
        
        println!("\n--- Advanced ---");
        println!("[6] View Keys");
        println!("[7] Network Status");
        println!("[8] Exchange XMR");
        
        println!("\n[B] Back to Main Menu");
        println!("[X] Exit");
        
        print!("\nSelect option: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        
        match input.trim() {
            "1" => check_balance().await?,
            "2" => show_address_info().await?,
            "3" => send_xmr_flow().await?,
            "4" => show_receive_qr().await?,
            "5" => show_transactions().await?,
            "6" => show_keys().await?,
            "7" => check_network_status().await?,
            "8" => exchange_xmr().await?,
            "B" | "b" => return Ok(CliResponse::Continue),
            "X" | "x" => return Ok(CliResponse::Exit),
            _ => println!("Invalid option"),
        }
    }
}

async fn check_balance() -> Result<(), String> {
    println!("\n🔄 Checking balance...");
    println!("   Connect to wallet RPC for real-time balance");
    println!("   Or check via: https://community.xmr.to/explorer/stagenet/");
    Ok(())
}

async fn show_address_info() -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;
    if let Some(wallet) = &manager.monero {
        println!("\n📍 Your Stagenet Address:");
        println!("{}", wallet.address);
        println!("\n💡 Share this to receive XMR");
        println!("   Faucet: https://community.xmr.to/faucet/stagenet/");
    }
    
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
    Ok(())
}

async fn send_xmr_flow() -> Result<(), String> {
    println!("\n💸 Send Monero");
    print!("Enter recipient address: ");
    io::stdout().flush().unwrap();
    
    let mut address = String::new();
    io::stdin().read_line(&mut address).map_err(|e| e.to_string())?;
    
    print!("Enter amount (XMR): ");
    io::stdout().flush().unwrap();
    
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).map_err(|e| e.to_string())?;
    
    let manager = WALLET_MANAGER.read().await;
    if let Some(wallet) = &manager.monero {
        match wallet.send_transaction(address.trim(), amount.trim().parse().unwrap_or(0.0)).await {
            Ok(tx_id) => println!("✅ Transaction sent: {}", tx_id),
            Err(e) => println!("❌ {}", e),
        }
    }
    
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
    Ok(())
}

async fn show_receive_qr() -> Result<(), String> {
    println!("\n📱 Receive XMR - QR Code");
    println!("   [Feature coming soon]");
    println!("   For now, share your address manually");
    Ok(())
}

async fn show_transactions() -> Result<(), String> {
    println!("\n📜 Transaction History");
    println!("   View on explorer: https://community.xmr.to/explorer/stagenet/");
    Ok(())
}

async fn show_keys() -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;
    if let Some(wallet) = &manager.monero {
        println!("\n🔑 Wallet Keys:");
        println!("View Key: {}", wallet.view_key);
        println!("Spend Key: [Hidden - Use monero-wallet-cli]");
        println!("\n⚠️  Keep these secret!");
    }
    
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
    Ok(())
}

async fn check_network_status() -> Result<(), String> {
    let manager = WALLET_MANAGER.read().await;
    if let Some(wallet) = &manager.monero {
        match wallet.get_network_info().await {
            Ok(info) => {
                println!("\n🌐 Network Status:");
                println!("   Height: {}", info.height);
                println!("   Network: {}", if info.stagenet { "Stagenet" } else { "Unknown" });
                println!("   Status: {}", info.status);
            }
            Err(e) => println!("❌ Error: {}", e),
        }
    }
    
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
    Ok(())
}

async fn exchange_xmr() -> Result<(), String> {
    println!("\n💱 Exchange XMR");
    println!("   [1] XMR → BTC");
    println!("   [2] XMR → ETH");
    println!("   [3] BTC → XMR");
    println!("   Coming soon with atomic swaps!");
    
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
    Ok(())
}
