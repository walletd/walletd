# WalletD ICP Integration

This crate provides support for ICP wallets in the WalletD SDK.

## Features
- Create and manage ICP wallets
- Perform basic token transfers
- Support for decentralized identities (DID)

## Usage
```rust
use walletd_icp::{IcpWallet, IcpTransaction};
use candid::Principal;

let wallet = IcpWallet::from_principal(Principal::anonymous(), 
walletd_hd_key::HDNetworkType::MainNet);
let tx = IcpTransaction::new(wallet.principal(), Principal::anonymous(), 
1000).unwrap();
