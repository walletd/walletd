//! WalletD CLI plugin for Prasaga Avio

use std::env;
use walletd_prasaga_avio::{walletd_integration::commands, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    // Parse network from environment or default to testnet
    let network = match env::var("WALLETD_NETWORK").as_deref() {
        Ok("mainnet") => Network::Mainnet,
        Ok("mocknet") => Network::Mocknet,
        _ => Network::Testnet,
    };

    match args[1].as_str() {
        "balance" => {
            if args.len() < 3 {
                println!("Usage: walletd prasaga balance <address>");
                return Ok(());
            }
            let result = commands::balance(&args[2], network).await?;
            println!("{result}");
        }
        "transfer" => {
            if args.len() < 5 {
                println!("Usage: walletd prasaga transfer <from> <to> <amount>");
                return Ok(());
            }
            let amount: u128 = args[4].parse()?;
            let result = commands::transfer(&args[2], &args[3], amount, network).await?;
            println!("{result}");
        }
        "address" => {
            let result = commands::generate_address(None)?;
            println!("New address: {result}");
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("WalletD Prasaga Avio Commands");
    println!("=============================");
    println!();
    println!("Commands:");
    println!("  balance <address>           Get balance");
    println!("  transfer <from> <to> <amt>  Transfer tokens");
    println!("  address                     Generate new address");
    println!();
    println!("Environment:");
    println!("  WALLETD_NETWORK            Network (mainnet/testnet/mocknet)");
}
