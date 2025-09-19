use walletd_prasaga_avio::PrasagaAvioClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Prasaga Avio Testnet Connection...");

    // Replace with actual testnet URL when available
    let endpoints = vec![
        "https://testnet.prasaga.com".to_string(),
        // Add backup endpoints if available
    ];

    let client = PrasagaAvioClient::new(endpoints).await?;

    // Try a basic RPC call
    match client.health_check().await {
        Ok(healthy) => {
            println!("✅ Connection successful!");
            println!(
                "Network status: {}",
                if healthy { "Healthy" } else { "Degraded" }
            );
        }
        Err(e) => {
            println!("❌ Connection failed: {e}");
            println!("Please verify the testnet URL and try again.");
        }
    }

    Ok(())
}
