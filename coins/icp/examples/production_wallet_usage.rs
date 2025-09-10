use ic_agent::identity::BasicIdentity;
use walletd_icp::{HDNetworkType, IcpWallet};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ICP Production Wallet Example");

    // Create a basic identity from key material
    // In production, load actual PEM data
    let pem_data = b""; // Empty for example
    let identity = BasicIdentity::from_pem(&pem_data[..]).unwrap();

    // Create wallet
    let _wallet = IcpWallet::new(Box::new(identity))?;

    println!("Wallet created successfully");
    println!("Network type: {:?}", HDNetworkType::MainNet);

    Ok(())
}
