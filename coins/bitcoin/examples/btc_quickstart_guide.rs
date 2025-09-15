use walletd_bitcoin::{BitcoinConfig, BitcoinWalletManager, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Bitcoin Quickstart Guide");

    // Create config for testnet
    let config = BitcoinConfig {
        network: Network::Testnet,
        rpc_endpoints: vec![], // Empty for now, would need proper RpcEndpoint structs
    };

    // Create wallet manager
    let manager = BitcoinWalletManager::new(config).await?;

    println!("Bitcoin wallet manager initialized");

    // Create a wallet with optional mnemonic
    let wallet = manager.create_wallet("quickstart-wallet", None).await?;
    println!("Created wallet: {wallet:?}");

    Ok(())
}
