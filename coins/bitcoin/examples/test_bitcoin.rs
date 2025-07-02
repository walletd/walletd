use walletd_bitcoin::{AddressType, BitcoinConfig, BitcoinWalletManager, Network};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 Testing WalletD Bitcoin SDK");
    println!("==============================\n");

    // Initialize Bitcoin manager
    let config = BitcoinConfig {
        network: Network::Bitcoin,
        rpc_endpoints: vec![], // No RPC for this test
    };

    let manager = BitcoinWalletManager::new(config).await?;

    // Create a wallet
    println!("1️⃣ Creating Bitcoin wallet...");
    let wallet = manager.create_wallet("test-user", None).await?;
    println!("✅ Wallet created!");
    println!("   Mnemonic: {}", wallet.mnemonic);
    println!("   xPub: {}", wallet.xpub);
    println!("   First address: {}", wallet.first_address);

    // Generate different address types
    println!("\n2️⃣ Generating addresses...");

    let legacy = manager
        .get_receive_address("test-user", AddressType::Legacy)
        .await?;
    println!("   Legacy (P2PKH): {legacy}");

    let segwit_p2sh = manager
        .get_receive_address("test-user", AddressType::SegwitP2SH)
        .await?;
    println!("   SegWit (P2SH): {segwit_p2sh}");

    let native_segwit = manager
        .get_receive_address("test-user", AddressType::NativeSegwit)
        .await?;
    println!("   Native SegWit: {native_segwit}");

    // Check balance
    println!("\n3️⃣ Checking balance...");
    let balance = manager.get_balance("test-user").await?;
    println!("   Confirmed: {} sats", balance.confirmed);
    println!("   Unconfirmed: {} sats", balance.unconfirmed);
    println!("   Total: {} sats", balance.total);

    println!("\n✅ Bitcoin SDK test completed!");

    Ok(())
}
