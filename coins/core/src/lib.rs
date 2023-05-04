//! # WalletD Coin Core
//!
//! Provides common traits, functionality, and interfaces that are used by other walletD libraries that are specific to a particular cryptocurrency.
//!]

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod blockchain_connector;
mod crypto_address;
mod crypto_amount;
mod crypto_wallet;
mod error;
pub mod prelude;
pub use blockchain_connector::{BlockchainConnector, BlockchainConnectorBuilder, ConnectorType};
pub use crypto_address::CryptoAddress;
pub use crypto_amount::CryptoAmount;
pub use crypto_wallet::{CryptoWallet, CryptoWalletBuilder};
pub use error::Error;
