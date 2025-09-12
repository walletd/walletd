use crate::types::CliResponse;
use std::io::{self, Write};

pub async fn handle_erc20_menu<T>(
    _wallet_api: &mut T,
    eth_address: &str,
    eth_balance: &str,
) -> Result<CliResponse, String> {
    loop {
        println!("\n========== ERC-20 TOKEN MENU ==========");
        println!("Ethereum Address: {eth_address}");
        println!("ETH Balance: {eth_balance}");

        println!("\n[1] Check Token Balance");
        println!("[2] List Popular Tokens");
        println!("[B] Back");

        print!("\nChoice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => println!("✅ USDC Balance: 1000.00"),
            "2" => println!("📋 Tokens: USDC, DAI, USDT, LINK"),
            "B" | "b" => return Ok(CliResponse::Continue),
            _ => println!("Invalid!"),
        }
    }
}
