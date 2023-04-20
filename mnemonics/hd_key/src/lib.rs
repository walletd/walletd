//! Walletd HD Key library for generating BIP32 compliant HD keys to facilitate
//! Hierarchical Deterministic (HD) wallets. Supports multiple HD key derivation
//! paths including BIP44, BIP49, and BIP84. Has support for customization of
//! the derivation path.

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
