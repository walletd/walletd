# Installation Guide

> **What you'll learn**: Install WalletD SDK and set up your development environment for multi-chain wallet development.

## System Requirements

### Minimum Requirements
- **Rust**: 1.70.0+ (MSRV - verified in [RELEASES.md](../../RELEASES.md))
- **Operating System**: macOS, Linux, or Windows
- **Memory**: 4GB RAM minimum
- **Storage**: 2GB free space

### Development Dependencies

**All Platforms:**
- Git
- OpenSSL/LibSSL
- pkg-config (Linux)

## Platform-Specific Setup

### macOS

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies via Homebrew
brew install openssl pkg-config

# Reload shell
source ~/.cargo/env
```

### Ubuntu/Debian

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
sudo apt-get update
sudo apt-get install -y libssl-dev pkg-config build-essential

# Reload shell
source ~/.cargo/env
```

### Windows

```powershell
# Install Rust via rustup-init.exe from https://rustup.rs/
# Then install build tools:
# Download and install "C++ Build Tools" from Microsoft

# Or use chocolatey:
choco install rust
choco install openssl
```

## Install WalletD

### Method 1: From Source (Recommended)

```bash
# Clone repository
git clone https://github.com/walletd/walletd
cd walletd

# Build release version
cargo build --release

# Verify installation
cargo run --bin walletd-icp-cli --help
```

### Method 2: Specific Components

```bash
# Build only Bitcoin support
cargo build --package walletd-bitcoin --release

# Build only Ethereum support  
cargo build --package walletd-ethereum --release

# Build only the CLI
cargo build --bin walletd-icp-cli --release
```

## Verify Installation

### Run Quality Checks

```bash
# Ensure all tests pass
cargo test

# Check code quality
cargo clippy

# Verify formatting
cargo fmt --check
```

### Test CLI Interface

```bash
# Launch interactive CLI
cargo run --bin walletd-icp-cli

# Should display:
#     ██╗    ██╗  █████╗  ██╗      ██╗      ███████╗ ████████╗ ██████╗         
#     ██║    ██║ ██╔══██╗ ██║      ██║      ██╔════╝ ╚══██╔══╝ ██╔══██╗   ██╗  
#     ...
```

## Add to Your Project

### As Library Dependency

Add to your project's `Cargo.toml`:

```toml
[dependencies]
# Core framework
walletd = { path = "path/to/walletd" }

# Specific blockchains (choose what you need)
walletd-bitcoin = { path = "path/to/walletd/coins/bitcoin" }
walletd-ethereum = { path = "path/to/walletd/coins/ethereum" }
walletd-solana = { path = "path/to/walletd/coins/solana" }
walletd-monero = { path = "path/to/walletd/coins/monero" }
walletd-hedera = { path = "path/to/walletd/coins/hedera" }
walletd-icp = { path = "path/to/walletd/coins/icp" }

# Key management
walletd-hd-key = { path = "path/to/walletd/key_manager/hd_key" }
```

### Verify Integration

```rust
// test_integration.rs
use walletd_bitcoin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WalletD integration test");
    
    // This should compile without errors
    let _builder = BitcoinWallet::builder();
    println!("✅ WalletD integration successful!");
    
    Ok(())
}
```

## Troubleshooting

### Common Issues

**Error: `failed to run custom build command for 'openssl-sys'`**
```bash
# macOS:
brew install openssl
export OPENSSL_DIR=/opt/homebrew/opt/openssl

# Ubuntu:
sudo apt-get install libssl-dev
```

**Error: `linker 'cc' not found`**
```bash
# Ubuntu/Debian:
sudo apt-get install build-essential

# CentOS/RHEL:
sudo yum groupinstall "Development Tools"
```

**Rust version too old:**
```bash
rustup update stable
```

### Performance Optimization

For faster builds during development:

```bash
# Use faster linker on Linux
sudo apt-get install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Enable incremental compilation
export CARGO_INCREMENTAL=1
```

## Next Steps

- [Quick Start Guide](quick-start.md) - Create your first wallet
- [First Wallet Tutorial](first-wallet.md) - Step-by-step wallet creation
- [Developer Guide](../developer-guide.md) - Complete development workflow

---

**Having issues?** Check our [troubleshooting guide](../resources/troubleshooting.md) or open an issue on GitHub.
