//! # WalletD Prasaga Avio SDK
//!
//! A Rust SDK for integrating with the Prasaga Avio blockchain's object-oriented architecture.
//!
//! ## Features
//!
//! - **Multi-network support**: Mainnet, Testnet, and Mocknet configurations
//! - **Ed25519 cryptography**: Secure key generation and transaction signing
//! - **XBOM serialization**: Native support for Prasaga's object model
//! - **PSA tokens**: Programmable Smart Asset management
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use walletd_prasaga_avio::{PrasagaAvioClient, Network};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Connect to testnet
//! let client = PrasagaAvioClient::testnet().await?;
//!
//! // Check network health
//! let healthy = client.health_check().await?;
//! # Ok(())
//! # }
//! ```

#![doc(html_logo_url = "https://prasaga.com/logo.png")]
#![doc(html_favicon_url = "https://prasaga.com/favicon.ico")]

pub mod assets;
pub mod indexer;
pub mod keys;
pub mod network;
pub mod psa;
pub mod transaction;
pub mod types;
pub mod utils;
pub mod xbom;

pub use keys::keypair::PrasagaAvioKeypair;
pub use network::client::PrasagaAvioClient;
pub use network::config::{Network, NetworkConfig};
pub use transaction::builder::{Operation, TransactionBuilder};
pub use transaction::signer::{SignedTransaction, TransactionSigner};
#[cfg(feature = "testing")]
// Re-exports
pub use types::*;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}
pub mod walletd_integration;
