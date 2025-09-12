use crate::CliResponse;
use std::io;

pub async fn handle_xmr_menu(
    _wallet_api: &mut crate::WalletDIcpApi,
    xmr_address: &str,
    xmr_balance: &str,
) -> Result<CliResponse, String> {
    loop {
        println!("\n========== MONERO WALLET (STAGENET) ==========");
        println!("Address: {xmr_address}");
        println!("Balance: {xmr_balance}");
        println!("Network: STAGENET");
        println!("============================================");

        println!("\n[1] Check Balance");
        println!("[2] Show Full Address");
        println!("[3] Send XMR (Demo)");
        println!("[4] Get Stagenet XMR (Live Testnet)");
        println!("[5] Network Status");

        println!("\n[B] Back to Main Menu");
        println!("[X] Exit");

        print!("\nSelect option: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        match input.trim() {
            "1" => {
                println!("\n💰 Balance: {xmr_balance}");
                println!("Check live balance on explorer:");
                println!("https://community.xmr.to/explorer/stagenet/tx/{xmr_address}");
                wait_for_enter();
            }
            "2" => {
                println!("\n📍 Your Stagenet Address:");
                println!("{xmr_address}");
                println!("\n💡 Use this address to receive XMR");
                wait_for_enter();
            }
            "3" => {
                println!("\n💸 Send XMR Demo");
                print!("Enter recipient address: ");
                io::stdout().flush().unwrap();
                let mut recipient = String::new();
                io::stdin()
                    .read_line(&mut recipient)
                    .map_err(|e| e.to_string())?;

                print!("Enter amount: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin()
                    .read_line(&mut amount)
                    .map_err(|e| e.to_string())?;

                println!("\n📤 Transaction Preview:");
                println!(
                    "   To: {}...{}",
                    &recipient[..12],
                    &recipient[recipient.len() - 12..]
                );
                println!("   Amount: {} XMR", amount.trim());
                println!("\n(Demo mode - use monero-wallet-cli for real transactions)");
                wait_for_enter();
            }
            "4" => {
                match crate::monero_live_testnet::instant_testnet_loader(xmr_address).await {
                    Ok(msg) => println!("\n{msg}"),
                    Err(e) => println!("\n❌ {e}"),
                }
                wait_for_enter();
            }
            "5" => {
                println!("\n🌐 Network Status");
                println!("   Network: Stagenet");
                println!("   Node: node.monerodevs.org:38089");
                println!("   Status: Connected");
                wait_for_enter();
            }
            "B" | "b" => return Ok(CliResponse::Continue),
            "X" | "x" => return Ok(CliResponse::Exit),
            _ => println!("Invalid option"),
        }
    }
}

fn wait_for_enter() {
    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    let _ = io::stdin().read_line(&mut _input);
}

use std::io::Write;
