use crate::CliResponse;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

pub async fn handle_prasaga_menu(
    _api: &mut crate::WalletDIcpApi,
    address: &str,
    balance: &str,
) -> Result<CliResponse, String> {
    loop {
        println!("\n=== 🚀 Prasaga Avio (SAGA) Menu ===");
        println!("Address: {address}");
        println!("Balance: {balance}");
        println!();
        println!("[1] Generate New Avio Address");
        println!("[2] Check Avio Balance");
        println!("[3] Transfer SAGA Tokens");
        println!("[4] Avio Network Status");
        println!("[5] View Transaction History");
        println!("[0] Back to Main Menu");
        println!();
        print!("Select option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => generate_address(),
            "2" => check_balance(),
            "3" => transfer_saga(),
            "4" => show_network_status(),
            "5" => println!("📜 Transaction history not yet available (awaiting Avio testnet)"),
            "0" => break,
            _ => println!("❌ Invalid option"),
        }
    }
    Ok(CliResponse::Continue)
}

fn get_prasaga_binary() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push("../walletd-prasaga-avio/target/release/walletd_prasaga");

    if !path.exists() {
        path = std::env::current_dir().unwrap();
        path.push("../walletd-prasaga-avio/target/debug/walletd_prasaga");
    }

    path
}

fn generate_address() {
    println!("\n🔑 Generating new Prasaga Avio address...");

    let binary = get_prasaga_binary();
    match Command::new(&binary).arg("address").output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if output.status.success() {
                println!("{stdout}");
            } else {
                println!("❌ Error: {stderr}");
            }
        }
        Err(e) => println!("❌ Failed to execute: {e}"),
    }
}

fn check_balance() {
    print!("Enter Prasaga Avio address: ");
    io::stdout().flush().unwrap();

    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap();
    let address = address.trim();

    if !address.starts_with("saga") {
        println!("❌ Invalid address format. Prasaga Avio addresses start with 'saga'");
        return;
    }

    println!("\n💰 Checking Avio balance for {address}...");

    let binary = get_prasaga_binary();
    match Command::new(&binary).arg("balance").arg(address).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{stdout}");
        }
        Err(e) => println!("❌ Failed to check balance: {e}"),
    }
}

fn transfer_saga() {
    println!("\n💸 Transfer SAGA Tokens on Avio Network");

    print!("From Avio address: ");
    io::stdout().flush().unwrap();
    let mut from = String::new();
    io::stdin().read_line(&mut from).unwrap();

    print!("To Avio address: ");
    io::stdout().flush().unwrap();
    let mut to = String::new();
    io::stdin().read_line(&mut to).unwrap();

    print!("Amount (in SAGA): ");
    io::stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).unwrap();

    let binary = get_prasaga_binary();
    match Command::new(&binary)
        .arg("transfer")
        .arg(from.trim())
        .arg(to.trim())
        .arg(amount.trim())
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{stdout}");
        }
        Err(e) => println!("❌ Failed to prepare transfer: {e}"),
    }
}

fn show_network_status() {
    println!("\n🌐 Prasaga Avio Network Status");
    println!("================================");
    println!("Blockchain:  Prasaga Avio");
    println!("Network:     Testnet");
    println!("Chain ID:    9000");
    println!("Token:       SAGA");
    println!("RPC Status:  Awaiting endpoints from Prasaga");
    println!("Explorer:    https://sagascan.prasaga.com");
    println!("Faucet:      Not yet available");
}
