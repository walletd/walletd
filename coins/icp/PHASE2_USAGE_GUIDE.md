# Phase 2 Usage Guide

## Quick Start

### 1. Interacting with Tokens (ICRC-1)

```rust
use walletd_icp::canister::{CanisterClient, Icrc1Client, Account};
use ic_agent::Agent;
use candid::Principal;

// Create agent and client
let agent = Agent::builder()
    .with_url("https://ic0.app")
    .build()?;

let canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai")?;
let client = CanisterClient::new(agent, canister_id);
let token = Icrc1Client::new(client);

// Query token info
let name = token.name().await?;
let symbol = token.symbol().await?;
let decimals = token.decimals().await?;

// Check balance
let account = Account {
    owner: my_principal,
    subaccount: None,
};
let balance = token.balance_of(account).await?;
2. Working with NFTs (ICRC-7)
rustuse walletd_icp::canister::{Icrc7Client, TokenId};

let nft = Icrc7Client::new(client);

// Query NFT
let token_id = TokenId::from(1u64);
let owner = nft.owner_of(token_id.clone()).await?;
let metadata = nft.metadata(token_id).await?;
3. Security Best Practices
rustuse walletd_icp::canister::{SecurityValidator};

let mut validator = SecurityValidator::new();

// Check rate limits
validator.check_rate_limit(&principal)?;

// Validate inputs
validator.validate_input(&transfer_args)?;
4. Performance Monitoring
rustuse walletd_icp::canister::PerformanceMonitor;

let mut monitor = PerformanceMonitor::new();
let timer = monitor.start_timer();

// Your canister call here
let result = token.transfer(args).await;

monitor.record_call(
    "icrc1_transfer".to_string(),
    canister_id,
    timer,
    result.is_ok()
);

// Get metrics
let avg_duration = monitor.average_duration("icrc1_transfer");
let success_rate = monitor.success_rate("icrc1_transfer");
Advanced Features

Custom canister interfaces
Batch operations
Cross-canister calls
Error recovery strategies

See examples/ directory for more usage patterns.
