//! # WalletD Ethereum Library
//!
//! This library provides a wallet implementation for Ethereum and blockchain-specific functionality.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use core::fmt;

mod ethclient;
pub use ethclient::EthClient;
mod ethereum_amount;
pub use ethereum_amount::EthereumAmount;
mod ethereum_wallet;
pub use ethereum_wallet::EthereumWallet;
mod error;
pub use error::Error;
pub use web3;

/// EthereumFormat is an enum that represents the format of an Ethereum address (checksummed or non-checksummed)
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
