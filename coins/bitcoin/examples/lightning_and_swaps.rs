use walletd_bitcoin::{
    lightning::LightningManager,
    swaps::{Chain, SwapCoordinator},
    BitcoinConfig, BitcoinWalletManager, Network,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("⚡ Bitcoin Lightning & Cross-Chain Swaps Demo");
    println!("=============================================\n");

    // Initialize Bitcoin manager
    let btc_config = BitcoinConfig {
        network: Network::Bitcoin,
        rpc_endpoints: vec![],
    };
    let btc_manager = BitcoinWalletManager::new(btc_config).await?;

    // Create a wallet
    let wallet = btc_manager.create_wallet("alice", None).await?;
    println!("✅ Bitcoin wallet created for Alice");
    println!("   Address: {}", wallet.first_address);

    // Initialize Lightning
    let lightning = LightningManager::new(Network::Bitcoin).await?;

    #[cfg(feature = "lightning")]
    {
        println!("\n⚡ Lightning Network: ENABLED");
        match lightning.create_node("alice", [0u8; 32]).await {
            Ok(node) => {
                println!("   Node ID: {}", node.node_id);
                println!("   Port: {}", node.listening_port);
            }
            Err(e) => println!("   Lightning setup error: {e}"),
        }
    }

    #[cfg(not(feature = "lightning"))]
    {
        println!("\n⚡ Lightning Network: DISABLED");
        println!("   To enable: cargo run --example lightning_and_swaps --features lightning");
        match lightning.create_node("alice", [0u8; 32]).await {
            Err(e) => println!("   Expected error: {e}"),
            Ok(_) => println!("   Unexpected success"),
        }
    }

    // Initialize swap coordinator
    println!("\n💱 Cross-Chain Swaps:");
    let swap_coordinator = SwapCoordinator::new();

    // Show available swap routes
    let routes = vec![
        (Chain::Bitcoin, Chain::ICP),
        (Chain::Bitcoin, Chain::Ethereum),
        (Chain::Bitcoin, Chain::Solana),
        (Chain::Bitcoin, Chain::Monero),
    ];

    println!("\nAvailable swap routes:");
    for (from, to) in routes {
        let route = swap_coordinator
            .create_swap_route(from, to, 100_000)
            .await?;
        println!("   {from:?} → {to:?}: {route:?}");
    }

    // Simulate a swap
    println!("\n🔄 Initiating BTC → ICP swap:");
    let swap_id = swap_coordinator
        .initiate_btc_to_icp_swap(
            100_000,   // 0.001 BTC
            1_000_000, // 0.01 ICP
            &wallet.first_address,
            "test-icp-principal",
        )
        .await?;
    println!("   Swap ID: {swap_id}");
    println!("   Status: Initiated");
    println!("   From: {} sats (BTC)", 100_000);
    println!("   To: {} e8s (ICP)", 1_000_000);

    println!("\n✅ Demo completed!");
    println!("\nFeatures demonstrated:");
    println!("• Bitcoin HD wallet creation");
    println!("• Lightning Network node (when enabled)");
    println!("• Cross-chain swap route discovery");
    println!("• BTC ↔ ICP atomic swap initiation");

    Ok(())
}
