//! Example of object operations on Prasaga Avio

use std::env;
use walletd_prasaga_avio::{PrasagaAvioClient, PrasagaAvioKeypair};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a tokio runtime
    let runtime = tokio::runtime::Runtime::new()?;

    // Run the async main function
    runtime.block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Prasaga Avio Object Operations Example");
    println!("=========================================");

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment
    dotenv::dotenv().ok();

    // Create client
    let endpoint = env::var("PRASAGA_TESTNET_URL")
        .unwrap_or_else(|_| "https://testnet.prasaga.com".to_string());

    println!("📡 Connecting to: {endpoint}");
    let _client = PrasagaAvioClient::new(vec![endpoint]).await?;

    // Create keypair
    let mnemonic = env::var("TEST_MNEMONIC").unwrap_or_else(|_| {
        "test test test test test test test test test test test junk".to_string()
    });
    let keypair = PrasagaAvioKeypair::from_mnemonic(&mnemonic, "", "m/44'/9000'/0'/0/0")?;

    println!("🔑 Account loaded");
    println!(
        "📍 Public Key: 0x{}",
        hex::encode(keypair.public_key_bytes())
    );

    // Example: Create an object (placeholder - will be implemented when connected to testnet)
    println!("\n📦 Object Operations:");
    println!("  - Create Object: TODO (waiting for testnet connection)");
    println!("  - Invoke Method: TODO (waiting for testnet connection)");
    println!("  - Transfer Object: TODO (waiting for testnet connection)");
    println!("  - Query State: TODO (waiting for testnet connection)");

    println!("\n✅ Example completed!");

    Ok(())
}
