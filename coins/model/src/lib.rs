//! # WalletD Coin Model Library
//!
//! This library provides common traits, functionality, and interfaces that are used by other walletd libraries that are specific to a particular cryptocurrency.
#![warn(missing_docs)]

mod blockchain_connector;
mod crypto_address;
mod crypto_amount;
mod crypto_wallet;
mod error;

pub use blockchain_connector::{
    BlockchainConnector, BlockchainConnectorBuilder, BlockchainConnectorGeneral, ConnectorType,
};
pub use crypto_address::CryptoAddress;
pub use crypto_amount::CryptoAmount;
pub use crypto_wallet::{CryptoWallet, CryptoWalletBuilder, CryptoWalletGeneral};
pub use error::Error;
