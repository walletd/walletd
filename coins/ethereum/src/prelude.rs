//! This prelude module simplifies importing many useful items from the walletd_ethereum crate using a glob import.
//!
//! To use this prelude, add the following to your code:
//! ```
//! use walletd_ethereum::prelude::*;
//! ```

pub use crate::{EthereumAmount, EthereumFormat, EthereumWallet, EthereumWalletBuilder};
