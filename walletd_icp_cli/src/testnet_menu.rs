use crate::CliResponse;
use std::io::{self, Write};

pub async fn handle_testnet_menu() -> Result<CliResponse, String> {
    loop {
        println!("\n🧪 Testnet Testing Suite");
        println!("━━━━━━━━━━━━━━━━━━━━━━");
        println!("Run real transactions on test networks!");

        println!("\n[1] Bitcoin Testnet Transaction");
        println!("[2] Ethereum Sepolia Transaction");
        println!("[3] Solana Devnet Transaction");
        println!("[4] Hedera Testnet Transaction");
        println!("[5] Monero Stagenet Transaction");
        println!("[6] ICP Local Network Transaction");
        println!("[7] Run ALL Testnet Tests");

        println!("\n[F] Faucet Links");
        println!("[B] Back to Main Menu");

        print!("\nSelect test: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        match input.trim().to_lowercase().as_str() {
            "1" => {
                crate::testnet_integration::bitcoin_testnet::test_bitcoin_transaction()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            "2" => {
                crate::testnet_integration::ethereum_testnet::test_ethereum_transaction()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            "3" => {
                crate::testnet_integration::solana_testnet::test_solana_transaction()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            "4" => {
                crate::testnet_integration::hedera_testnet::test_hedera_transaction()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            "5" => {
                crate::testnet_integration::monero_testnet::test_monero_transaction()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            "6" => {
                crate::testnet_integration::icp_testnet::test_icp_transaction()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            "7" => {
                crate::testnet_integration::run_all_testnet_tests()
                    .await
                    .map_err(|e| e.to_string())?;
            }
            "f" => {
                crate::testnet_integration::show_faucet_links();
            }
            "b" => return Ok(CliResponse::Continue),
            _ => println!("Invalid option"),
        }

        println!("\nPress Enter to continue...");
        let mut _pause = String::new();
        io::stdin().read_line(&mut _pause).ok();
    }
}
