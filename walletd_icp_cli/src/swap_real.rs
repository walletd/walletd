use crate::mode_selector::WalletMode;
use crate::swaps::{simple_swap::SimpleSwapProvider, Chain};
use crate::wallet_integration::WALLET_MANAGER;
use std::io::{self, Write};

pub async fn handle_cross_chain_swap() -> Result<(), String> {
    println!("\n=== Cross-Chain Swap ===");

    let manager = WALLET_MANAGER.read().await;

    match &manager.mode {
        WalletMode::Demo => {
            println!("📌 DEMO MODE - No real swaps will be executed");
        }
        _ => {
            println!("📌 REAL MODE - Swaps will be executed on-chain");
        }
    }

    println!("\nAvailable pairs:");
    println!("[1] BTC → ETH");
    println!("[2] ETH → BTC");
    println!("[3] BTC → SOL");
    println!("[4] ETH → SOL");
    println!("[5] Custom pair");

    print!("\nSelect swap pair: ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .map_err(|e| e.to_string())?;

    let (from_chain, to_chain) = match choice.trim() {
        "1" => (Chain::Bitcoin, Chain::Ethereum),
        "2" => (Chain::Ethereum, Chain::Bitcoin),
        "3" => (Chain::Bitcoin, Chain::Solana),
        "4" => (Chain::Ethereum, Chain::Solana),
        _ => {
            println!("Invalid choice");
            return Ok(());
        }
    };

    print!("\nAmount to swap: ");
    io::stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .map_err(|e| e.to_string())?;

    let amount_f64: f64 = amount.trim().parse().map_err(|_| "Invalid amount")?;

    // Get quote
    println!("\n🔄 Getting swap quote...");
    match SimpleSwapProvider::get_swap_quote(
        &from_chain,
        &to_chain,
        &from_chain.to_string(),
        &to_chain.to_string(),
        amount_f64,
    )
    .await
    {
        Ok(quote) => {
            println!("\n📊 Swap Quote:");
            println!("From: {} {}", quote.input_amount, from_chain);
            println!("To: {} {}", quote.output_amount, to_chain);
            println!(
                "Rate: 1 {} = {} {}",
                from_chain, quote.exchange_rate, to_chain
            );
            println!("Fee: {} {}", quote.fee, from_chain);
            println!("Expires in: 5 minutes");

            print!("\nProceed with swap? (yes/no): ");
            io::stdout().flush().unwrap();
            let mut confirm = String::new();
            io::stdin()
                .read_line(&mut confirm)
                .map_err(|e| e.to_string())?;

            if confirm.trim().to_lowercase() == "yes" {
                print!("Enter destination address: ");
                io::stdout().flush().unwrap();
                let mut dest_addr = String::new();
                io::stdin()
                    .read_line(&mut dest_addr)
                    .map_err(|e| e.to_string())?;

                match &manager.mode {
                    WalletMode::Demo => {
                        println!("\n✅ Demo swap executed!");
                        println!("Swap ID: demo_{}", uuid::Uuid::new_v4());
                        println!("Status: Completed (Demo)");
                    }
                    _ => {
                        match SimpleSwapProvider::execute_swap(
                            &from_chain,
                            &to_chain,
                            amount_f64,
                            dest_addr.trim(),
                        )
                        .await
                        {
                            Ok(result) => {
                                println!("\n✅ Swap initiated!");
                                println!("Swap ID: {}", result.swap_id);
                                println!("Status: {:?}", result.status);
                                println!(
                                    "\n⏳ Monitor your swap at: https://app.thorchain.com/swap/{}",
                                    result.swap_id
                                );
                            }
                            Err(e) => println!("❌ Swap failed: {e}"),
                        }
                    }
                }
            }
        }
        Err(e) => println!("❌ Failed to get quote: {e}"),
    }

    Ok(())
}
