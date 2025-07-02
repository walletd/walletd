# WalletD Developer Guide

> **What you'll learn**: Set up your development environment, build WalletD, and start developing multi-chain applications.

## Prerequisites

- **Rust**: 1.70+ (verified in [RELEASES.md](../RELEASES.md#minimum-supported-rust-version))
- **Git**: For cloning repositories
- **OpenSSL**: Required for cryptographic operations
- **DFX**: Optional, for ICP local development

### System Dependencies

**macOS:**
```bash
brew install openssl
```

**Ubuntu/Debian:**
```bash
sudo apt-get install libssl-dev pkg-config
```

## Installation

### 1. Clone Repository

```bash
git clone https://github.com/walletd/walletd
cd walletd
```

### 2. Build from Source

```bash
# Build all components
cargo build --release

# Build specific component
cargo build --package walletd-bitcoin --release
```

### 3. Run Development CLI

```bash
cargo run --bin walletd-icp-cli
```

## Development Workflow

### 1. Select Development Mode

When you run the CLI, always select **Option 1: Testnet Mode** for development:
- Safe testing with test tokens
- No real money at risk
- Built-in faucets for most chains

### 2. Code Quality Checks

```bash
# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt
```

### 3. Testing Strategy

Each blockchain has dedicated test suites:
```bash
# Test specific blockchain
cargo test --package walletd-bitcoin
cargo test --package walletd-ethereum

# Integration tests
cargo test --test '*'
```

## Getting Test Tokens

### Hedera (Testnet Portal)
1. Select blockchain 4 (Hedera) in CLI
2. Choose "Get Testnet HBAR" option
3. Follow guided setup to access portal.hedera.com faucet

### Monero (Stagenet Setup)
1. Select blockchain 6 (Monero) in CLI
2. Choose "Get Stagenet XMR" option
3. Follow guided stagenet wallet setup

### Bitcoin
- Use testnet faucets (links provided in CLI)
- Or use the faucet integration in the CLI

### Ethereum
- Use Sepolia testnet faucets
- CLI provides direct links to faucets

### Solana
- Built-in devnet airdrop in CLI
- Automatically requests SOL when needed

## API Usage Examples

### Basic Wallet Creation

```rust
use walletd_bitcoin::prelude::*;
use bdk::bitcoin::Network;
use bdk::keys::bip39::Mnemonic;

#[tokio::main]
async fn main() -> Result<(), walletd_bitcoin::Error> {
    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let mut btc_wallet = BitcoinWallet::builder()
        .mnemonic(mnemonic)
        .network_type(Network::Testnet)
        .build()?;

    println!("Bitcoin address: {}", btc_wallet.receive_address()?);
    Ok(())
}
```

### Cross-Chain Operations

```rust
// See docs/cross_chain.md for complete examples
use walletd_icp_cli::swaps;

// Cross-chain swap functionality
let swap_result = swaps::execute_swap(from_chain, to_chain, amount).await?;
```

## Development Environment

### Project Structure

```
walletd/
├── coins/           # Blockchain-specific implementations
│   ├── bitcoin/     # Bitcoin + Lightning Network
│   ├── ethereum/    # Ethereum + ERC-20/721
│   ├── solana/      # Solana + SPL tokens
│   ├── monero/      # Monero privacy features
│   ├── hedera/      # Hedera Hashgraph
│   └── icp/         # Internet Computer Protocol
├── key_manager/     # HD wallet and key management
├── walletd_icp_cli/ # Command-line interface
├── tests/           # Integration tests
└── examples/        # Usage examples
```

### Configuration Files

- `Cargo.toml` - Workspace configuration
- `rustfmt.toml` - Code formatting rules
- `dfx.json` - ICP development configuration

### Available Commands

```bash
# Build specific blockchain
cargo build --package walletd-{bitcoin,ethereum,solana,monero,hedera,icp}

# Run CLI with specific blockchain
cargo run --bin walletd-icp-cli

# Run examples
cargo run --example bitcoin_example

# Generate documentation
cargo doc --open
```

## Debugging and Logging

WalletD uses standard Rust logging. Set log level:

```bash
RUST_LOG=debug cargo run --bin walletd-icp-cli
```

## Next Steps

- [API Reference](API_REFERENCE.md) - Complete API documentation
- [Bitcoin Guide](BITCOIN_GUIDE.md) - Bitcoin-specific development
- [Ethereum Guide](ETHEREUM_GUIDE.md) - Ethereum smart contracts
- [Cross-Chain Guide](cross_chain.md) - Cross-chain operations

---

**Need help?** Check the `examples/` directory for working code or open an issue on GitHub.
