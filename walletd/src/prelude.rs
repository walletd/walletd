//! This prelude module simplifies importing many useful items from the walletd crate using a glob import.
//!
//! # Examples
//!
//! To use this prelude, add the following to your code:
//! ```
//! use walletd::prelude::*;
//! ```
//!
//! It is also possible to import from the prelude from multiple in-scope crates at once.
//! Doing this allows you to choose to import the items you would likely need in a modular fashion while avoiding unnecessary imports.
//!
//! ```
//! use walletd::prelude::*;
//! use walletd_bip39::prelude::*;
//! use walletd_bitcoin::prelude::*;
//! use walletd_ethereum::prelude::*;
//! ```

pub use crate::CryptoCoin;
pub use crate::{KeyPair, KeyPairBuilder, MnemonicKeyPairType};
pub use walletd_coin_core::prelude::*;
pub use walletd_hd_key::prelude::*;
pub use walletd_mnemonics_core::prelude::*;
