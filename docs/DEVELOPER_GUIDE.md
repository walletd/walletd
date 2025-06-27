# WalletD CLI - Developer Guide

## Installation

Prerequisites:
- Rust 1.70+
- Git  
- OpenSSL

Build from source:
cargo build --release

## Usage

Run the CLI:
cargo run --bin walletd-icp-cli

Select testnet mode (option 1) for development.

## Getting Test Tokens

### Hedera - 1000 HBAR instantly
Select blockchain 4 (Hedera) and get 1000 HBAR automatically.

### Monero - Built-in faucet  
Select blockchain 5 (Monero), then Get Stagenet XMR.

## API Usage

Example code:
use walletd::{Hedera, Bitcoin, Monero};

async fn example() {
    let hedera = Hedera::new_testnet().await?;
    let balance = hedera.get_balance().await?;
}
