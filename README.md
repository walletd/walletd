# WalletD 🌐⚡

### *The Ultimate Multi-Chain Wallet SDK*

> **One SDK. Six Blockchains. Infinite Possibilities.**

Build the future of Web3 with the most comprehensive multi-chain wallet framework ever created. From Bitcoin's Lightning Network to Ethereum's smart contracts, from Solana's speed to Monero's privacy - **WalletD makes it effortless**.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

**🎯 Perfect for:** DeFi protocols • Cross-chain apps • Wallet integrations • Blockchain research • Enterprise solutions

## ⚡ Why WalletD is Amazing

### 🚀 **Developer Experience**
- **5-minute setup** - Get running instantly
- **Type-safe APIs** - Rust's compile-time guarantees  
- **Unified interface** - Same patterns across all chains
- **Rich examples** - Copy-paste ready code
- **Best practices** - Security and performance built-in

### 🌍 **Cross-Chain Power**
- **Atomic swaps** - Seamless value transfer
- **Chain abstraction** - Write once, run everywhere
- **Real bridges** - Production-ready integrations
- **Live testnets** - Instant funding and testing
- **Enterprise ready** - Battle-tested architecture

## 🌈 What Makes WalletD Special?

### 🎯 **For Beginners**
```bash
# Literally this simple:
cargo run --bin walletd-icp-cli
# Select: 1) Testnet Mode
# Select: 4) Hedera  
# Select: 1) Create Account
# 🎉 1000 HBAR instantly!
```

### 👨‍💻 **For Developers**
```rust
use walletd_bitcoin::prelude::*;

let wallet = BitcoinWallet::builder()
    .mnemonic(mnemonic)
    .network_type(Network::Testnet)
    .build()?;

println!("Address: {}", wallet.receive_address()?);
```

### ⚡ **Live Demo Features**
- 🧪 **Testnet integration** with faucet access for all supported chains
- 🔄 **Real cross-chain swaps** using live testnet bridges
- 🎮 **Interactive CLI** with guided workflows
- 🔒 **Safe testnet environment** - experiment fearlessly!

## 📄 License

**Dual-licensed for maximum compatibility:**
- [MIT License](LICENSE-MIT) - For permissive use
- [Apache 2.0](LICENSE-APACHE) - For enterprise adoption

---

**Built with ❤️ by the WalletD team**

*One SDK. Six blockchains. Infinite possibilities.* 🌐⚡
