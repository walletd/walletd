//! This prelude module simplifies importing many useful items from the walletd_mnemonics_core crate using a glob import.
//!
//! To use this prelude, add the following to your code:
//! ```
//! use walletd_mnemonics_core::prelude::*;
//! ```

pub use crate::{FromStr, Language, Mnemonic, MnemonicBuilder, Seed};
