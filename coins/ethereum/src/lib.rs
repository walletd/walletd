//! # WalletD Ethereum Library
//!
//! This library provides a wallet implementation for Ethereum and blockchain-specific functionality.
//!
//! # Quickstart Guide
//!
//! Using the [EthereumWallet] struct is a good starting point for using this library and access most of the functionality.
//!
//! Here's how you can import an Ethereum wallet based on a master [HDKey].
//! ```
//! use walletd_ethereum::prelude::*;
//!
//! fn import_ethereum_hd_wallet() -> Result<(), walletd_ethereum::Error> {
//! Ok(())
//! }
//! ```
//!

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use core::fmt;

mod ethclient;
pub use ethclient::EthClient;
mod ethereum_amount;
pub use ethereum_amount::EthereumAmount;
mod ethereum_wallet;
pub use ethereum_wallet::{EthereumWallet, EthereumWalletBuilder};
mod error;
pub use error::Error;
pub use web3;
pub mod prelude;
pub use walletd_bip39::{
    Bip39Language, Bip39Mnemonic, Bip39MnemonicType, Mnemonic, MnemonicBuilder, Seed,
};
pub use walletd_coin_core::{CryptoAddress, CryptoAmount, CryptoWallet, CryptoWalletBuilder};
pub use walletd_hd_key::{HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose};

/// Represents the format of an Ethereum address (checksummed or non-checksummed)
#[derive(Default, Debug, Clone, Copy)]
pub enum EthereumFormat {
    #[default]
    /// Checksummed is the checksummed format of an Ethereum address where the case of each letter is mixed using the checksum algorithm
    Checksummed,
    /// NonChecksummed is the non-checksummed format of an Ethereum address where the letters are all lowercase
    NonChecksummed,
}

impl fmt::Display for EthereumFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EthereumFormat::Checksummed => write!(f, "Checksummed"),
            EthereumFormat::NonChecksummed => write!(f, "NonChecksummed"),
        }
    }
}
