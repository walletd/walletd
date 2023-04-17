//! # WalletD Coin Model Library
//! 
//! This library provides common traits, functionality, and interfaces that are used by other walletd libraries that are specific to a particular cryptocurrency.
#![warn(missing_docs)]

mod crypto_address;
mod crypto_wallet;
mod crypto_amount;
mod blockchain_connector; 
mod error;

pub use crypto_wallet::{CryptoWallet, CryptoWalletGeneral, CryptoWalletBuilder};
pub use crypto_address::CryptoAddress;
pub use crypto_amount::CryptoAmount;
pub use error::Error;
pub use blockchain_connector::{BlockchainConnector, BlockchainConnectorGeneral, BlockchainConnectorBuilder, ConnectorType};



