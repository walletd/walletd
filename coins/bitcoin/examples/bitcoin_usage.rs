use walletd_bitcoin::{BitcoinWalletManager, BitcoinConfig, Network, SendRequest, AddressType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize for multiple users
    let config = BitcoinConfig {
        network: Network::Bitcoin,
        rpc_endpoints: vec![],
    };
    
    let manager = BitcoinWalletManager::new(config).await?;
    
    // Create wallets for multiple users
    let wallet1 = manager.create_wallet("user-001", None).await?;
    println!("User 1 wallet: {}", wallet1.first_address);
    
    let wallet2 = manager.create_wallet("user-002", None).await?;
    println!("User 2 wallet: {}", wallet2.first_address);
    
    // Get balances
    let balance1 = manager.get_balance("user-001").await?;
    println!("User 1 balance: {} sats", balance1.total);
    
    // Create multisig
    let multisig = manager.create_multisig(
        vec!["user-001".to_string(), "user-002".to_string()],
        2
    ).await?;
    println!("2-of-2 multisig: {}", multisig.address);
    
    // Batch operations
    let balances = manager.batch_get_balances(vec![
        "user-001".to_string(),
        "user-002".to_string(),
    ]).await?;
    
    for (user, balance) in balances {
        println!("{}: {} sats", user, balance.total);
    }
    
    Ok(())
}
