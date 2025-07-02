# Bitcoin Integration Guide

This guide covers Bitcoin wallet functionality in WalletD, including wallet creation, balance checking, address generation, and transaction handling.

## Quick Start

### Prerequisites

Add the Bitcoin module to your `Cargo.toml`:

```toml
[dependencies]
walletd-bitcoin = { path = "coins/bitcoin" }
bdk = "0.28"
```

### Basic Wallet Setup

```rust
use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::keys::bip39::Mnemonic;
use walletd_bitcoin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), walletd_bitcoin::Error> {
    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let mut btc_wallet = BitcoinWallet::builder()
        .mnemonic(mnemonic)
        .network_type(Network::Testnet)
        .build()?;

    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let blockchain = ElectrumBlockchain::from(client);
    btc_wallet.sync(&blockchain).await?;

    println!("next receive address: {}", btc_wallet.receive_address()?);

    let balance = btc_wallet.balance().await?;
    println!("bitcoin wallet balance: {} satoshi", balance.confirmed);

    Ok(())
}
```

## Core Features

### Wallet Creation

The `BitcoinWallet::builder()` pattern provides flexible wallet configuration:

```rust
let btc_wallet = BitcoinWallet::builder()
    .mnemonic(mnemonic)
    .network_type(Network::Testnet)  // Use Network::Bitcoin for mainnet
    .build()?;
```

### Network Types

- `Network::Bitcoin` - Bitcoin mainnet
- `Network::Testnet` - Bitcoin testnet
- `Network::Signet` - Bitcoin signet
- `Network::Regtest` - Local development network

### Address Generation

```rust
// Get the next receiving address
let receive_addr = btc_wallet.receive_address()?;
println!("Send Bitcoin to: {}", receive_addr);
```

### Balance Checking

```rust
let balance = btc_wallet.balance().await?;
println!("Confirmed balance: {} satoshi", balance.confirmed);
println!("Unconfirmed balance: {} satoshi", balance.trusted_pending);
```

### Blockchain Synchronization

Before checking balances or creating transactions, sync with the blockchain:

```rust
// Connect to Electrum server
let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
let blockchain = ElectrumBlockchain::from(client);

// Sync wallet
btc_wallet.sync(&blockchain).await?;
```

## Advanced Features

### Lightning Network

WalletD Bitcoin module includes Lightning Network support. See `coins/bitcoin/examples/lightning_and_swaps.rs` for implementation details.

### Multisig Wallets

Multi-signature wallet functionality is available through the Bitcoin module. See `coins/bitcoin/examples/bitcoin_usage.rs` for examples.

### Batch Operations

The module supports batch transaction operations for efficiency. See examples in `coins/bitcoin/examples/` directory.

## Development Setup

### Local Testing

For local development, you can use:

1. **Bitcoin Core regtest mode**
2. **Electrum server for testnet**
3. **Local Electrum server**

### Testnet Faucets

For testnet Bitcoin, use these faucets:
- https://testnet-faucet.mempool.co/
- https://bitcoinfaucet.uo1.net/

### Example Commands

```bash
# Run Bitcoin examples
cd coins/bitcoin
cargo run --example btc_quickstart_guide
cargo run --example bitcoin_usage
cargo run --example lightning_and_swaps
```

## Error Handling

The Bitcoin module defines `walletd_bitcoin::Error` for Bitcoin-specific errors:

```rust
match btc_wallet.balance().await {
    Ok(balance) => println!("Balance: {}", balance.confirmed),
    Err(walletd_bitcoin::Error::NetworkError(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Security Considerations

### Private Key Management

- Use hardware wallets for production
- Never log or expose mnemonic phrases
- Use secure random generation for new wallets

### Network Security

- Use SSL/TLS connections to Electrum servers
- Verify server certificates
- Consider running your own Electrum server

### Transaction Verification

- Always verify transaction details before signing
- Check recipient addresses carefully
- Validate fee amounts

## Integration with Other Chains

The Bitcoin module integrates with WalletD's cross-chain functionality:

```rust
// Cross-chain swap from ICP to Bitcoin
walletd.swap_icp_to_btc(
    from_principal,
    btc_wallet.receive_address()?,
    amount
).await?;
```

## Resources

- [Bitcoin Developer Guide](https://developer.bitcoin.org/)
- [BDK Documentation](https://bitcoindevkit.org/)
- [Electrum Protocol](https://electrumx.readthedocs.io/en/latest/protocol.html)
- [Lightning Network Specifications](https://github.com/lightning/bolts)

For more examples and advanced usage, see the `coins/bitcoin/examples/` directory.
