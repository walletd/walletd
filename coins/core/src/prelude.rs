//! This prelude module simplifies importing many useful items from the walletd_coin_core crate using a glob import.
//!
//! To use this prelude, add the following to your code:
//! ```
//! use walletd_coin_core::prelude::*;
//! ```

pub use crate::{
    BlockchainConnector, BlockchainConnectorBuilder, ConnectorType, CryptoAddress, CryptoAmount,
    CryptoWallet, CryptoWalletBuilder,
};
