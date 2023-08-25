//! This prelude module simplifies importing many useful items from the walletd_ethereum crate using a glob import.
//!
//! To use this prelude, add the following to your code:
//! ```
//! use walletd_ethereum::prelude::*;
//! ```

pub use crate::{EthClient, EthereumAmount, EthereumFormat, EthereumWallet, EthereumWalletBuilder};

pub use bdk::keys::bip39::Mnemonic;
pub use ethers::types::Transaction;
pub use walletd_mnemonics_core::Seed;
