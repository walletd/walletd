use walletd_prasaga_avio::PrasagaAvioClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WalletD Prasaga Network Examples\n");

    // 1. Mocknet (local development)
    println!("1. Mocknet (Local Development):");
    let mocknet_client = PrasagaAvioClient::mocknet().await?;
    match mocknet_client.health_check().await {
        Ok(_) => println!("   ✅ Mocknet ready (using mock responses)"),
        Err(e) => println!("   ❌ Mocknet error: {e}"),
    }

    // 2. Testnet (public test network)
    println!("\n2. Testnet (Public Test Network):");
    let testnet_client = PrasagaAvioClient::testnet().await?;
    println!("   Chain ID: {}", testnet_client.chain_id());
    println!("   Network: {:?}", testnet_client.network());
    println!("   (Waiting for Prasaga to provide endpoints)");

    // 3. Mainnet (production)
    println!("\n3. Mainnet (Production):");
    let mainnet_client = PrasagaAvioClient::mainnet().await?;
    println!("   Chain ID: {}", mainnet_client.chain_id());
    println!("   Network: {:?}", mainnet_client.network());
    println!("   (Not yet launched)");

    Ok(())
}
