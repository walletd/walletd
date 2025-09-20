use crate::CliResponse;
use std::io::{self, Write};

pub async fn handle_base_menu(
    _api: &mut crate::WalletDIcpApi,
    address: &str,
    balance: &str,
) -> Result<CliResponse, String> {
    loop {
        println!("\n=== 🔷 Base L2 Menu ===");
        println!("Address: {address}");
        println!("Balance: {balance}");
        println!();
        println!("[1] View Balance");
        println!("[2] Send ETH on Base");
        println!("[3] Bridge from Ethereum");
        println!("[4] View Transaction History");
        println!("[0] Back to Main Menu");
        println!();
        print!("Select option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => println!("Balance: {balance} ETH on Base"),
            "2" => todo!("Send function not yet implemented"),
            "3" => todo!("Bridge function not yet implemented"),
            "4" => todo!("History function not yet implemented"),
            "0" => break,
            _ => println!("Invalid option"),
        }
    }
    Ok(CliResponse::Continue)
}
