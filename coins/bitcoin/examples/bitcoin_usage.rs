use walletd_bitcoin::{BitcoinConfig, BitcoinWalletManager, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create config
    let config = BitcoinConfig {
        network: Network::Testnet,
        rpc_endpoints: vec![], // Empty for now, would need proper RpcEndpoint structs
    };

    // Create wallet manager
    let manager = BitcoinWalletManager::new(config).await?;

    // Create a new wallet with optional mnemonic
    let wallet_info = manager.create_wallet("user-001", None).await?;
    println!("Created wallet: {wallet_info:?}");

    // Get balance
    let balance = manager.get_balance("user-001").await?;
    println!("Balance: {balance:?}");

    // Create another wallet
    let wallet_info2 = manager.create_wallet("user-002", None).await?;
    println!("Created second wallet: {wallet_info2:?}");

    let balance2 = manager.get_balance("user-002").await?;
    println!("Second wallet balance: {balance2:?}");

    Ok(())
}
