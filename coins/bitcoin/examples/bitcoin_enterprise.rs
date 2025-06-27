use walletd_bitcoin::{
    lightning::LightningNodeManager, multi_wallet::EnterpriseWalletManager, AddressType,
    BitcoinConfig, BitcoinWalletManager, Network,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 WalletD Bitcoin Enterprise Demo");
    println!("==================================\n");

    // Initialize enterprise manager
    let config = EnterpriseConfig {
        security_config: SecurityConfig {
            hsm_config: Some(HsmConfig {
                provider: "SoftHSM".to_string(),
                slot_id: 0,
                pin: "1234".to_string(),
            }),
            kdf_params: KeyDerivationParams {
                iterations: 100_000,
                memory: 65536,
                parallelism: 4,
            },
            key_rotation_days: 90,
        },
        monitoring_config: MonitoringConfig {
            prometheus_port: 9090,
            enable_alerts: true,
        },
        rpc_config: RpcConfig {
            endpoints: vec![
                "https://btc-node1.example.com".to_string(),
                "https://btc-node2.example.com".to_string(),
            ],
        },
        rate_limit_config: RateLimitConfig {
            requests_per_minute: 100,
            burst_size: 20,
        },
        max_concurrent_ops: 1000,
    };

    let manager = EnterpriseWalletManager::new(config).await?;

    // Create wallets for users
    println!("1️⃣ Creating user wallets...");
    for i in 1..=5 {
        let wallet = manager
            .create_wallet_with_compliance(
                &format!("user-{:03}", i),
                KycInfo {
                    user_id: format!("user-{:03}", i),
                    country: "US".to_string(),
                    verified: true,
                    risk_score: 10,
                },
            )
            .await?;
        println!("   ✅ User {}: {}", i, wallet.first_address);
    }

    // Batch transactions
    println!("\n2️⃣ Processing batch transactions...");
    let batch_txs = vec![
        BatchTransaction {
            user_id: "user-001".to_string(),
            to_address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            amount: 100_000,
            priority: TransactionPriority::High,
        },
        // ... more transactions
    ];

    let results = manager.process_batch_transactions(batch_txs).await?;
    println!("   ✅ Processed {} transactions", results.len());

    // Lightning Network
    println!("\n3️⃣ Lightning Network setup...");
    let ln_manager = LightningNodeManager::new(Network::Bitcoin).await?;

    let node = ln_manager.create_node("user-001", [0u8; 32]).await?;
    println!("   ⚡ Lightning node: {}", node.node_id);

    // Open channel
    let channel = ln_manager
        .open_channel(OpenChannelRequest {
            user_id: "user-001".to_string(),
            peer_node_id: "02abc...".to_string(),
            amount_sats: 1_000_000,
            push_msat: Some(100_000_000),
            user_channel_id: 1,
        })
        .await?;
    println!("   ⚡ Channel opened: {}", channel.channel_id);

    // Fee optimization
    println!("\n4️⃣ Fee optimization...");
    let optimization = manager.optimize_fees("user-001").await?;
    match optimization.recommendation {
        FeeRecommendation::Consolidate => {
            println!(
                "   💰 Consolidation recommended, saves: {} sats",
                optimization.estimated_savings
            );
        }
        _ => println!("   ✅ Fees already optimized"),
    }

    // Hot wallet management
    println!("\n5️⃣ Hot wallet management...");
    manager
        .manage_hot_wallet_balance(
            "hot-wallet-001",
            1_000_000_000, // 10 BTC min
            5_000_000_000, // 50 BTC max
        )
        .await?;
    println!("   ✅ Hot wallet balanced");

    println!("\n✅ All systems operational!");
    println!("\nFeatures demonstrated:");
    println!("• Multi-user wallet management");
    println!("• Hardware security module integration");
    println!("• Lightning Network support");
    println!("• Batch transaction processing");
    println!("• Automated fee optimization");
    println!("• Hot/cold wallet management");
    println!("• Enterprise monitoring & alerting");

    Ok(())
}
