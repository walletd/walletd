//! Example of basic transfer operations on Prasaga Avio

use std::env;
use walletd_prasaga_avio::{PrasagaAvioClient, PrasagaAvioKeypair};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a tokio runtime
    let runtime = tokio::runtime::Runtime::new()?;

    // Run the async main function
    runtime.block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment
    dotenv::dotenv().ok();

    println!("🚀 Prasaga Avio Transfer Example");
    println!("=================================");

    // Create client
    let endpoint = env::var("PRASAGA_TESTNET_URL")
        .unwrap_or_else(|_| "https://testnet.prasaga.com".to_string());
    let client = PrasagaAvioClient::new(vec![endpoint.clone()]).await?;

    println!("📡 Connected to: {endpoint}");

    // Create keypair from test mnemonic
    let mnemonic = env::var("TEST_MNEMONIC").unwrap_or_else(|_| {
        "test test test test test test test test test test test junk".to_string()
    });
    let keypair = PrasagaAvioKeypair::from_mnemonic(&mnemonic, "", "m/44'/9000'/0'/0/0")?;

    println!("✅ Wallet initialized");
    println!(
        "📍 Public Key: 0x{}",
        hex::encode(keypair.public_key_bytes())
    );

    // Health check
    match client.health_check().await {
        Ok(healthy) => println!(
            "🏥 Network health: {}",
            if healthy {
                "✅ Healthy"
            } else {
                "⚠️ Unhealthy"
            }
        ),
        Err(e) => println!("⚠️ Health check failed (testnet may be unavailable): {e}"),
    }

    // Example transfer (placeholder - will be implemented when connected to testnet)
    println!("\n💸 Transfer Operations (Coming Soon):");
    println!("  - Native SAGA token transfer");
    println!("  - PSA token transfer");
    println!("  - Multi-asset batch transfer");
    println!("  - Cross-account object transfer");

    Ok(())
}
