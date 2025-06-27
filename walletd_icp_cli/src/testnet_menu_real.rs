use crate::CliResponse;
use crate::config::{WalletDConfig, WalletMode};
use std::io::{self, Write};

pub async fn handle_testnet_menu() -> Result<CliResponse, String> {
    // Check if we're in testnet mode
    let config = WalletDConfig::load();
    
    if config.mode != WalletMode::Testnet {
        println!("\n⚠️  You're currently in {:?} mode", config.mode);
        println!("To use testnet features:");
        println!("1. Edit walletd_config.json");
        println!("2. Set \"mode\": \"testnet\"");
        println!("3. Restart the application");
        
        println!("\nPress Enter to continue...");
        let mut _pause = String::new();
        io::stdin().read_line(&mut _pause).ok();
        
        return Ok(CliResponse::Continue);
    }
    
    loop {
        println!("\n🧪 Testnet Testing Suite - REAL CONNECTIONS");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Connected to real test networks!");
        
        println!("\n[1] Bitcoin Testnet (Real)");
        println!("[2] Ethereum Sepolia (Real)");
        println!("[3] Solana Devnet (Real)");
        println!("[4] Hedera Testnet");
        println!("[5] Monero Stagenet");
        println!("[6] ICP Local Network");
        
        println!("\n[F] Faucet Links");
        println!("[B] Back to Main Menu");
        
        print!("\nSelect test: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        
        match input.trim() {
            "1" => {
                crate::testnet_integration_real::bitcoin_testnet::test_bitcoin_transaction()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            "2" => {
                crate::testnet_integration_real::ethereum_testnet::test_ethereum_transaction()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            // ... other options ...
            "b" | "B" => return Ok(CliResponse::Continue),
            _ => println!("Invalid option"),
        }
        
        println!("\nPress Enter to continue...");
        let mut _pause = String::new();
        io::stdin().read_line(&mut _pause).ok();
    }
}
