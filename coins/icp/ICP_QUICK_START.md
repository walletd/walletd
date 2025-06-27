# WalletD ICP - Quick Start Guide

## Installation
```toml
[dependencies]
walletd_icp = { path = "../path/to/walletd/coins/icp" }
1. Create a Wallet
rustuse walletd_icp::{IcpWallet, HDNetworkType};

// From seed phrase
let wallet = IcpWallet::from_seed("your seed phrase", HDNetworkType::MainNet)?;

// From principal
let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);
2. Connect to a Canister
rustuse walletd_icp::CanisterClient;

// Quick local connection
let client = CanisterClient::local("rrkah-fqaaa-aaaaa-aaaaq-cai").await?;

// Quick mainnet connection
let client = CanisterClient::mainnet("ryjl3-tyaaa-aaaaa-aaaba-cai").await?;
3. Make Type-Safe Calls
rust// Query
let balance: u64 = client.query_typed("get_balance", &()).await?;

// Update
let result: String = client.update_typed("transfer", &(to, amount)).await?;
4. Test with Mocks
rustuse walletd_icp::MockCanister;

let mock = MockCanister::new("rrkah-fqaaa-aaaaa-aaaaq-cai")
    .with_query("balance", 1000u64);

// Use in tests - no replica needed!
That's it! You're ready to build with ICP! 🚀
