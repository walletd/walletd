pub mod btc_menu;
pub mod btc_send_real;
pub mod btc_simple;
pub mod cli_options;
pub mod config;
pub mod eth_menu;
pub mod eth_send_real;
pub mod eth_simple;
pub mod hardware_wallet;
pub mod hbar_menu;
pub mod icp_network;
pub mod icp_real_menu;
pub mod swap_real;
pub mod swaps;
pub mod traits;
pub mod types;
pub mod wallet_integration;

pub use cli_options::CliOptions;
pub use types::{CliResponse, WalletDIcpApi};

use std::io::{self, Write};

pub async fn icp_overview(
    wallet: &mut WalletDIcpApi,
    address: &str,
) -> Result<CliResponse, String> {
    crate::icp_real_menu::handle_icp_menu(wallet, address, "100").await
}

pub fn display_message(message: &str, icon: &str) {
    println!("\n{icon} {message}");
}

pub fn display_crypto_menu(coins: &[cli_options::CoinType]) -> String {
    println!("\n🪙  Select Cryptocurrency:");
    for (i, coin) in coins.iter().enumerate() {
        println!("[{}] {}", i + 1, coin);
    }
    println!("[X] Exit");

    print!("\nYour choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => "BTC".to_string(),
        "2" => "ETH".to_string(),
        "3" => "HBAR".to_string(),
        "4" => "ICP".to_string(),
        "x" | "X" => std::process::exit(0),
        _ => "BTC".to_string(),
    }
}

pub fn get_mock_data(coin: &str) -> (String, String) {
    match coin {
        "BTC" => (
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            "0.5".to_string(),
        ),
        "ETH" => (
            "0x742d35Cc6634C0532925a3b844Bc9e7595f7e8E".to_string(),
            "1.5".to_string(),
        ),
        "HBAR" => ("0.0.12345".to_string(), "1000".to_string()),
        "ICP" => ("ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), "100".to_string()),
        _ => ("unknown".to_string(), "0".to_string()),
    }
}
pub mod base_menu;
pub mod erc20_menu;
pub mod hbar_menu_faucet;
pub mod hbar_send_real;
pub mod hbar_send_testnet;
pub mod hedera_account_pool;
pub mod hedera_auto_faucet;
pub mod hedera_auto_fund;
pub mod hedera_faucet;
pub mod hedera_funded_wallet;
pub mod hedera_pool_faucet;
pub mod hedera_portal_faucet;
pub mod hedera_real_ops;
pub mod hedera_simple_fund;
pub mod hedera_testnet_accounts;
pub mod hedera_testnet_auto_fund;
pub mod hedera_testnet_simulator;
pub mod hedera_wallet_stub;
pub mod hedera_working_faucet;
pub mod icp_menu;
pub mod mining_helper;
pub mod mode_selector;
pub mod monero_auto_faucet;
pub mod monero_balance_checker;
pub mod monero_integrated;
pub mod monero_live_testnet;
pub mod monero_miner;
pub mod monero_stagenet_autofund;
pub mod monero_testnet_funder;
pub mod monero_testnet_service;
pub mod prasaga_menu;
pub mod sol_menu;
pub mod sol_send_real;
pub mod testnet_integration;
pub mod testnet_integration_real;
pub mod testnet_menu;
pub mod testnet_real_menu;
pub mod xmr_menu;
pub mod xmr_send_real;
