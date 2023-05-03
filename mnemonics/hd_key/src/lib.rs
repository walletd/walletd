//! Walletd HD Key library for generating BIP32 compliant HD keys to facilitate
//! Hierarchical Deterministic (HD) wallets. Supports multiple HD key derivation
//! paths including BIP44, BIP49, and BIP84. Has support for customization of
//! the derivation path.
//!
//! # Quickstart Guide
//!
//! The [HDKey] struct is the main entry point for the library.
//! You can create a new master [HDKey] from a [Seed] and also derive child [HDKey].
//! The network type [HDNetworkType] is associated with each [HDKey] which affects the format of the associated [ExtendedPrivateKey] and [ExtendedPublicKey] when serialized.
//! The derivation path [HDPath] can be customized to support different HD key derivation schemes including various [HDPurpose] types such as [BIP44][HDPurpose::BIP44], [BIP49][HDPurpose::BIP49], and [BIP84][HDPurpose::BIP84].
//! The [HDPathBuilder] struct which can be easily accessed through [`HDPath::builder()`] implements common default settings for the [HDPath] and can be used to customize the [HDPath] to your needs.
//!
//!
//! Here's how you can import a master hd key based on a seed hex:
//! ```
//! # use walletd_hd_key::prelude::*;
//! # fn main() -> Result<(), walletd_hd_key::Error> {
//! let seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//! Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod hd_key;
pub use hd_key::{ExtendedPrivateKey, ExtendedPublicKey, HDKey, HDNetworkType};
pub use slip44;
mod derive_path;
pub use derive_path::{HDPath, HDPathBuilder, HDPathIndex, HDPurpose};
mod error;
pub use error::Error;
pub use walletd_mnemonics_core::Seed;
pub mod prelude;

#[doc(hidden)]
pub use std::str::FromStr;
