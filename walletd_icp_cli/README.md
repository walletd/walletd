# WalletD SDK

An open-source, modular, and unopinionated Rust-based SDK for building cryptocurrency wallets.

## Quick Start

```rust
use walletd_bitcoin::BitcoinWallet;
use walletd_ethereum::EthereumWallet;
use walletd_icp::IcpWallet;

// Create a multi-chain wallet
let btc_wallet = BitcoinWallet::new()?;
let eth_wallet = EthereumWallet::new()?;
let icp_wallet = IcpWallet::new()?;

// Developers choose their own implementation
Core Modules

walletd_bitcoin - Bitcoin and Lightning Network support
walletd_ethereum - Ethereum and ERC-20/721 support
walletd_icp - Internet Computer Protocol integration
walletd_solana - Solana and SPL token support
walletd_monero - Privacy-focused transactions
walletd_hedera - Enterprise DLT integration

Features

Modular Design: Use only what you need
Unopinionated: Bring your own infrastructure
Type-Safe: Leverage Rust's safety guarantees
Cross-Chain: Built-in swap and bridge interfaces
Hardware Ready: Support for common hardware wallets

Examples
See the examples/ directory for:

Basic wallet implementation
Cross-chain swaps
Hardware wallet integration
Multi-signature setups
Lightning Network channels
