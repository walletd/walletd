# WalletD ICP Canister SDK

Complete SDK for interacting with Internet Computer canisters.

## Quick Start

### 1. Connect to a Local Canister
```rust
use walletd_icp::CanisterClient;

// Quick connection to local replica
let client = CanisterClient::local("rrkah-fqaaa-aaaaa-aaaaq-cai").await?;

// Make a query
let result: String = client.query_typed("hello", &()).await?;
2. Connect to Mainnet
rust// Quick mainnet connection
let client = CanisterClient::mainnet("ryjl3-tyaaa-aaaaa-aaaba-cai").await?;
3. Advanced Configuration
rustlet client = CanisterClient::builder()
    .with_canister("your-canister-id")?
    .with_network(Network::Local)
    .with_identity(your_identity)
    .with_timeout(Duration::from_secs(30))
    .build()
    .await?;
4. Type-Safe Calls
rust#[derive(CandidType, Deserialize)]
struct MyResponse {
    value: u64,
    message: String,
}

let response: MyResponse = client
    .query_typed("get_info", &())
    .await?;
5. Testing with Mocks
rustuse walletd_icp::{MockCanister, testing::helpers};

let mock = MockCanister::new("test-canister")
    .with_query("balance", 1000u64)
    .with_method("transfer", |args: TransferArgs| {
        Ok(args.amount)
    });

// Use mock in tests
let result = mock.call("balance", &encode_args(&())?).await?;
Features

✅ Easy connection to local/mainnet/testnet
✅ Type-safe query and update calls
✅ Wallet integration
✅ Mock canisters for testing
✅ Comprehensive error handling
✅ Builder pattern for configuration
✅ Support for all ICP canister types

Examples
See the examples/ directory for complete examples:

complete_canister_guide.rs - Full SDK demonstration
developer_canister_guide.rs - Basic usage patterns
testing_guide.rs - Testing with mocks
