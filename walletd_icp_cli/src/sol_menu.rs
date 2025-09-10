use crate::{types::WalletDIcpApi, CliResponse};
use std::io::{self, Write};

pub async fn handle_sol_menu(
    _wallet_api: &mut WalletDIcpApi,
    address: &str,
    balance: &str,
) -> Result<CliResponse, String> {
    loop {
        println!("\n========== SOLANA WALLET FEATURES ==========");
        println!("Address: {address}");
        println!("Balance: {balance} SOL");

        println!("\n--- Wallet Operations ---");
        println!("[1] View Account Info");
        println!("[2] Check Balance");
        println!("[3] Request Airdrop (Devnet only)");

        println!("\n--- Transactions ---");
        println!("[4] Send SOL");
        println!("[5] Send SPL Token");
        println!("[6] Create Token");

        println!("\n--- Programs & NFTs ---");
        println!("[7] Deploy Program");
        println!("[8] Interact with Program");
        println!("[9] Mint NFT");
        println!("[10] View NFT Collection");

        println!("\n--- DeFi ---");
        println!("[11] Stake SOL");
        println!("[12] Swap Tokens (Raydium/Orca)");
        println!("[13] Provide Liquidity");

        println!("\n[B] Back to Main Menu");
        println!("[X] Exit");

        print!("\nSelect option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("\nAccount info functionality coming soon!");
            }
            "2" => {
                println!("\nChecking balance...");
                use crate::wallet_integration::WALLET_MANAGER;
                let manager = WALLET_MANAGER.read().await;
                if let Some(sol_wallet) = &manager.solana {
                    match sol_wallet.get_balance().await {
                        Ok(balance) => {
                            let sol = balance as f64 / 1_000_000_000.0;
                            println!("Current balance: {sol} SOL ({balance} lamports)");
                        }
                        Err(e) => {
                            println!("Error checking balance: {e}");
                        }
                    }
                } else {
                    println!("Solana wallet not initialized");
                }
            }
            "3" => {
                // Use the real airdrop handler
                if let Err(e) = crate::sol_send_real::handle_solana_airdrop().await {
                    println!("Error: {e}");
                }
            }
            "4" => {
                // Use the real send handler
                if let Err(e) = crate::sol_send_real::handle_send_solana_real().await {
                    println!("Error: {e}");
                }
            }
            "5" => {
                println!("\nSPL token functionality coming soon!");
            }
            "6" => {
                println!("\nToken creation functionality coming soon!");
            }
            "7" => {
                println!("\nProgram deployment functionality coming soon!");
            }
            "8" => {
                println!("\nProgram interaction functionality coming soon!");
            }
            "9" => {
                println!("\nNFT minting functionality coming soon!");
            }
            "10" => {
                println!("\nNFT collection viewing functionality coming soon!");
            }
            "11" => {
                println!("\nStaking functionality coming soon!");
            }
            "12" => {
                println!("\nSwap functionality coming soon!");
            }
            "13" => {
                println!("\nLiquidity provision functionality coming soon!");
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
