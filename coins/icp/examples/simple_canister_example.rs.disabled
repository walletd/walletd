//! Simple example showing canister SDK usage

use walletd_icp::MockCanister;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WalletD Canister SDK Example");
    println!("===========================\n");

    // Example 1: Create a mock for testing
    let mock = MockCanister::new("test-canister")
        .with_query("get_name", "Test Canister".to_string())
        .with_query("get_value", 42u64);

    println!("✅ Created mock canister for testing");

    // Example 2: Builder pattern
    println!("\n✅ CanisterClient supports:");
    println!("  - Local connection: CanisterClient::local(\"canister-id\")");
    println!("  - Mainnet connection: CanisterClient::mainnet(\"canister-id\")");
    println!("  - Custom configuration with builder pattern");
    println!("  - Type-safe queries and updates");
    println!("  - Mock canisters for testing");

    Ok(())
}
