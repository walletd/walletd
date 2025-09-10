use crate::{btc_send_real::handle_send_bitcoin_real, types::WalletDIcpApi, CliResponse};
use std::io::{self, Write};

pub async fn handle_btc_menu(
    _wallet_api: &mut WalletDIcpApi,
    address: &str,
    balance: &str,
) -> Result<CliResponse, String> {
    loop {
        println!("\n========== BITCOIN WALLET FEATURES ==========");
        println!("Address: {address}");
        println!("Balance: {balance} BTC");

        println!("\n--- Wallet Operations ---");
        println!("[1] Generate New Address");
        println!("[2] Show Extended Public Key");
        println!("[3] Check Balance");
        println!("[4] View Transaction History");

        println!("\n--- Transactions ---");
        println!("[5] Send Bitcoin");
        println!("[6] Create Multi-Signature Address");
        println!("[7] Sign Transaction");
        println!("[8] Broadcast Transaction");

        println!("\n--- Advanced Features ---");
        println!("[9] UTXO Management");
        println!("[10] Fee Estimation");
        println!("[11] Import/Export Wallet");
        println!("[12] Lightning Network");
        println!("[13] Swap Bitcoin");
        println!("[14] Cross-Chain Bridge");
        println!("[15] Hardware Wallet");

        println!("\n[B] Back to Main Menu");
        println!("[X] Exit");

        print!("\nSelect option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("\nGenerating new address...");
                println!("New address functionality coming soon!");
            }
            "2" => {
                println!("\nExtended public key functionality coming soon!");
            }
            "3" => {
                println!("\nChecking balance...");
                use crate::wallet_integration::WALLET_MANAGER;
                let manager = WALLET_MANAGER.read().await;
                if let Some(btc) = &manager.bitcoin {
                    match btc.get_balance().await {
                        Ok(balance_sats) => {
                            let balance_btc = balance_sats as f64 / 100_000_000.0;
                            println!("Current balance: {balance_btc} BTC ({balance_sats} sats)");
                        }
                        Err(e) => {
                            println!("Error checking balance: {e}");
                        }
                    }
                } else {
                    println!("Bitcoin wallet not initialized");
                }
            }
            "4" => {
                println!("\nTransaction history functionality coming soon!");
            }
            "5" => {
                // CALL THE REAL SEND FUNCTION
                if let Err(e) = handle_send_bitcoin_real("user").await {
                    println!("Error: {e}");
                }
            }
            "6" => {
                println!("\nMulti-signature functionality coming soon!");
            }
            "7" => {
                println!("\nSign transaction functionality coming soon!");
            }
            "8" => {
                println!("\nBroadcast transaction functionality coming soon!");
            }
            "9" => {
                println!("\nUTXO management functionality coming soon!");
            }
            "10" => {
                println!("\nFee estimation functionality coming soon!");
            }
            "11" => {
                println!("\nImport/Export functionality coming soon!");
            }
            "12" => {
                println!("\nLightning Network functionality coming soon!");
            }
            "13" => {
                println!("\nSwap functionality coming soon!");
            }
            "14" => {
                println!("\nCross-chain bridge functionality coming soon!");
            }
            "15" => {
                println!("\nHardware wallet functionality coming soon!");
            }
            "B" | "b" => return Ok(CliResponse::Continue),
            "X" | "x" => return Ok(CliResponse::Exit),
            _ => println!("Invalid option. Please try again."),
        }

        print!("\nPress Enter to continue...");
        io::stdout().flush().unwrap();
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
}
