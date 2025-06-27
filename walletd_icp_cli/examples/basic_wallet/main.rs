//! Basic wallet implementation example using WalletD SDK
use anyhow::Result;
use walletd_bitcoin::{BitcoinWallet, Network};
use walletd_ethereum::EthereumWallet;
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Bitcoin wallet
    let btc_wallet = BitcoinWallet::new(Network::Mainnet)?;
    // Generate a new address
    let btc_address = btc_wallet.new_address()?;
    println!("Bitcoin address: {}", btc_address);

    // Initialize Ethereum wallet
    let eth_wallet = EthereumWallet::new()?;
    let eth_address = eth_wallet.address();
    println!("Ethereum address: {}", eth_address);

    // Developers implement their own:
    // - RPC connections
    // - Database storage
    // - User interfaces
    // - Security measures

    Ok(())
}
