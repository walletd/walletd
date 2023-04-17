//! # WalletD Bitcoin Library
//!
//! This library provides a wallet implementation for Bitcoin including the ability create a new wallet or import an existing wallet, check balances, and handle transactions.
//! It supports a heirarchical deterministic (HD) wallet structure and provides the ability to search for previously used addresses associated with the wallet as well as the creation of new addresses.
//! It also facilitates obtaining blockchain information.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use bitcoin;

mod bitcoin_address;
pub use bitcoin_address::{BitcoinAddress, Network};
mod bitcoin_wallet;
pub use bitcoin_wallet::{BitcoinWallet, BitcoinPrivateKey, BitcoinPublicKey};
mod bitcoin_amount;
pub use bitcoin_amount::BitcoinAmount;
pub mod blockstream;
mod error;
pub use error::Error;

