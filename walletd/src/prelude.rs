//! This prelude module simplifies importing many useful items from the walletd crate using a glob import.
//!
//! To use this prelude, add the following to your code:
//! ```
//! use walletd::prelude::*;
//! ```

pub use crate::CryptoCoin;
pub use crate::{KeyPair, KeyPairBuilder, MnemonicKeyPairType};
pub use walletd_coin_core::prelude::*;
pub use walletd_hd_key::prelude::*;
pub use walletd_mnemonics_core::prelude::*;
