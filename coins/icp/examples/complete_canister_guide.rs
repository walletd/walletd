//! Complete guide for using WalletD with ICP canisters

use candid::{CandidType, Nat, Principal};
use serde::Deserialize;
use walletd_icp::{
    testing::helpers, CanisterClient, HDNetworkType, IcpWallet, MockCanister, Network,
};

// Example: Define your canister interface
#[derive(CandidType, Deserialize)]
struct TokenBalance {
    e8s: u64,
}

#[derive(CandidType)]
struct TransferArgs {
    to: Principal,
    amount: Nat,
    memo: Option<Vec<u8>>,
}

/// Example 1: Simple connection patterns
async fn connection_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Connection Examples ===\n");

    // 1a. Quick local connection
    let local_client = CanisterClient::local("rrkah-fqaaa-aaaaa-aaaaq-cai").await?;
    println!("✅ Connected to local canister");

    // 1b. Quick mainnet connection
    let mainnet_client = CanisterClient::mainnet("ryjl3-tyaaa-aaaaa-aaaba-cai").await?;
    println!("✅ Connected to mainnet canister");

    // 1c. Custom configuration with builder
    let custom_client = CanisterClient::builder()
        .with_canister("rrkah-fqaaa-aaaaa-aaaaq-cai")?
        .with_network(Network::Testnet)
        .with_timeout(std::time::Duration::from_secs(30))
        .build()
        .await?;
    println!("✅ Connected with custom configuration");

    Ok(())
}

/// Example 2: Making canister calls
async fn canister_call_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Canister Call Examples ===\n");

    let client = CanisterClient::local("rrkah-fqaaa-aaaaa-aaaaq-cai").await?;

    // 2a. Simple query
    let name: String = client.query_typed("get_name", &()).await?;
    println!("Name: {}", name);

    // 2b. Query with arguments
    let balance: TokenBalance = client
        .query_typed("get_balance", &Principal::from_text("aaaaa-aa")?)
        .await?;
    println!("Balance: {} e8s", balance.e8s);

    // 2c. Update call
    let transfer_args = TransferArgs {
        to: Principal::from_text("aaaaa-aa")?,
        amount: Nat::from(1_000_000u64),
        memo: Some(vec![1, 2, 3, 4]),
    };

    let result: Result<Nat, String> = client.update_typed("transfer", &transfer_args).await?;

    match result {
        Ok(block_height) => println!("Transfer successful at block: {}", block_height),
        Err(e) => println!("Transfer failed: {}", e),
    }

    Ok(())
}

/// Example 3: Working with wallet integration
async fn wallet_integration_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Wallet Integration Example ===\n");

    // Create wallet from seed
    let wallet = IcpWallet::from_seed(
        "your twelve word mnemonic phrase goes here for wallet creation",
        HDNetworkType::Local,
    )?;

    // Get wallet's principal
    let principal = wallet.principal();
    println!("Wallet principal: {}", principal);

    // Create canister client with wallet's identity
    let client = CanisterClient::builder()
        .with_canister("rrkah-fqaaa-aaaaa-aaaaq-cai")?
        .with_local_replica()
        .with_identity(wallet.get_identity()?)
        .build()
        .await?;

    // Now calls will be authenticated with the wallet's identity
    let my_balance: u64 = client.query_typed("get_my_balance", &()).await?;
    println!("My balance: {}", my_balance);

    Ok(())
}

/// Example 4: Testing with mock canisters
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_mock_canister() {
        // Create a mock token canister
        let mock_token = helpers::mock_icrc1_token(
            "mxzaz-hqaaa-aaaar-qaada-cai",
            "Internet Computer",
            "ICP",
            8,
            1_000_000_000_000_000, // 10M ICP
        );

        // Test balance query
        let balance_result = tokio_test::block_on(async {
            mock_token
                .call(
                    "icrc1_balance_of",
                    &encode_args((Account {
                        owner: Principal::from_text("aaaaa-aa").unwrap(),
                        subaccount: None,
                    },))?,
                )
                .await
        });

        assert!(balance_result.is_ok());
    }

    #[test]
    fn test_mock_defi_canister() {
        let mock_defi = helpers::mock_defi_canister("be2us-64aaa-aaaaa-qaabq-cai");

        // Test price query
        let price_result = tokio_test::block_on(async {
            mock_defi
                .call(
                    "get_price",
                    &encode_args((("ICP".to_string(), "USD".to_string()),))?,
                )
                .await
        });

        assert!(price_result.is_ok());
    }
}

/// Example 5: Error handling patterns
async fn error_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Error Handling Example ===\n");

    let client = CanisterClient::local("invalid-canister-id")
        .await
        .map_err(|e| {
            println!("Failed to connect: {}", e);
            e
        })?;

    // Handle query errors
    match client
        .query_typed::<String>("non_existent_method", &())
        .await
    {
        Ok(result) => println!("Unexpected success: {}", result),
        Err(e) => println!("Expected error: {}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("WalletD ICP Canister SDK - Complete Guide");
    println!("========================================\n");

    // Run all examples
    if let Err(e) = connection_examples().await {
        eprintln!("Connection examples failed: {}", e);
    }

    if let Err(e) = canister_call_examples().await {
        eprintln!("Call examples failed: {}", e);
    }

    if let Err(e) = wallet_integration_example().await {
        eprintln!("Wallet integration failed: {}", e);
    }

    if let Err(e) = error_handling_example().await {
        eprintln!("Error handling example failed: {}", e);
    }
}
