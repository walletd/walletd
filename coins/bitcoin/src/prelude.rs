//! This prelude module simplifies importing many useful items from the walletd_bitcoin crate using a glob import.
//!
//! To use this prelude, add the following to your code:
//! ```
//! use walletd_bitcoin::prelude::*;
//! ```

pub use crate::{BitcoinPublicKey, BitcoinWallet, BitcoinWalletBuilder};
pub use walletd_coin_core::prelude::*;
