//! This prelude module simplifies importing many useful items from the walletd_hd_key crate using a glob import.
//!
//! To use this prelude, add the following to your code:
//! ```
//! use walletd_hd_key::prelude::*;
//! ```

pub use crate::{
    FromStr, HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose, Seed,
};
