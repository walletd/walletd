//! # Walletd HD Key
//!
//! Library for generating [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) compliant HD keys to facilitate
//! Hierarchical Deterministic (HD) wallets. Supports multiple HD key derivation
//! paths including BIP44, BIP49, and BIP84. Has support for customization of
//! the derivation path.
//!
//! # Quickstart Guide
//!
//! The [HDKey] struct is the main entry point for the library.
//! You can create a new master [HDKey] from a [Seed] and also derive a child [HDKey].
//!
//! The network type [HDNetworkType] is associated with each [HDKey] which affects the format of the associated [ExtendedPrivateKey] and [ExtendedPublicKey] when serialized.
//! The derivation path [HDPath] can be customized to support different HD key derivation schemes including various [HDPurpose] types such as [BIP44][HDPurpose::BIP44], [BIP49][HDPurpose::BIP49], and [BIP84][HDPurpose::BIP84].
//! The [HDPathBuilder] struct which can be easily accessed through [`HDPath::builder()`] implements common default settings for the [HDPath] and can be used to customize the [HDPath] to your needs.
//!
//!
//! ## Master HD Key from a Seed
//!
//! Here's how you can import a master hd key based on a seed hex:
//! ```
//! use walletd_hd_key::prelude::*;
//! # fn main() -> Result<(), walletd_hd_key::Error> {
//! let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";;
//! let seed = Seed::from_str(seed_hex)?;
//! let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//! assert_eq!(master_hd_key.depth(), 0);
//! println!("master hd key depth {}", master_hd_key.depth());
//! # Ok(())
//! # }
//! ```
//!
//! Setting a network type on the HDKey is required, you should select [`HDNetworkType::TestNet`] during development and testing purposes and to avoid using real funds and [`HDNetworkType::MainNet`] for production level code with caution.
//! Be sure to be consistent with [HDNetworkType] when connecting to the blockchain, make sure to use a compatible blockchain for the specified network type category.
//!
//! ## HD Key Serialization, WIF
//!   
//! The Wallet Import Format (WIF) is a standard way to encode private keys.
//! The extended public key and extended private key can be serialized using the serialized string format
//!
//! How to display these key formats for an [HDKey]:
//! ```
//! # use walletd_hd_key::prelude::*;
//! # fn main() -> Result<(), walletd_hd_key::Error> {
//! # let seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//!
//! println!("wif of master hd key {}", master_hd_key.to_wif().unwrap());
//! println!(
//!     "master hd key extended public key: {}",
//!     master_hd_key.extended_public_key_serialized()?
//! );
//! println!(
//!     "master hd key extended private key: {}",
//!     master_hd_key.extended_private_key_serialized()?
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Specify Derivation Path
//!
//! Can flexibly specify the derivation path using the [HDPathBuilder]. The default [HDPathBuilder] which can be accessed through [`HDPath::builder()`] does not specify the purpose, or coin type, and uses the hardened indices for the purpose, coin type, and account indices.
//! ```
//! use walletd_hd_key::prelude::*;
//! use slip44::{Coin, Symbol};
//! # fn main() -> Result<(), walletd_hd_key::Error> {
//! # let seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//!
//! let default_deriv_path = HDPath::builder().build().to_string();
//! // without specifying the purpose, the default derivation path is "m
//! assert_eq!(default_deriv_path, "m");
//! println!("default derivation path: {}", default_deriv_path);
//!
//! // can flexibly specify the derivation path using the HDPathBuilder
//! let account_deriv_path = HDPath::builder()
//!.purpose_index(HDPurpose::BIP44.to_shortform_num())
//!.coin_type_index(Coin::from(Symbol::ETH).id())
//!.account_index(0)
//!.no_change_index()
//!.no_address_index()
//!.build().to_string();
//!
//! assert_eq!(account_deriv_path, "m/44'/60'/0'");
//! # Ok(())
//! # }
//! ```
//!
//! ## Derive Child HD Key
//!
//! Can derive a child key from the master key or a parent key using the [HDKey::derive] method.
//! ```
//! # use walletd_hd_key::prelude::*;
//! # use slip44::{Coin, Symbol};
//! # fn main() -> Result<(), walletd_hd_key::Error> {
//! # let seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//! # let account_deriv_path = HDPath::builder()
//! # .purpose_index(HDPurpose::BIP44.to_shortform_num())
//! # .coin_type_index(Coin::from(Symbol::ETH).id())
//! # .account_index(0)
//! # .no_change_index()
//! # .no_address_index()
//! # .build().to_string();
//! let eth_first_account_key = master_hd_key.derive(account_deriv_path.to_string())?;
//! assert_eq!(
//! eth_first_account_key.master_seed(),
//! master_hd_key.master_seed()
//! );
//! println!(
//! "eth_first_account_key depth {}",
//! eth_first_account_key.depth()
//! );
//! assert_eq!(eth_first_account_key.depth(), 3);
//! # Ok(())
//! }
//! ```
//!
//! Can also use a string directly to represent the derivation path, `'` or `h` can be used to denote a hardened index.
//! ```
//! # use walletd_hd_key::prelude::*;
//! # use slip44::{Coin, Symbol};
//! # fn main() -> Result<(), walletd_hd_key::Error> {
//! # let seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//!let account_deriv_path = HDPath::builder()
//!.purpose_index(HDPurpose::BIP44.to_shortform_num())
//!.coin_type_index(Coin::from(Symbol::ETH).id())
//!.account_index(0)
//!.no_change_index()
//!.no_address_index()
//!.build().to_string();
//! # let eth_first_account_key = master_hd_key.derive(account_deriv_path.to_string())?;
//! let compare_account_key = master_hd_key.derive("m/44h/60h/0h".to_string())?;
//! assert_eq!(eth_first_account_key, compare_account_key);
//!
//! let address_key1 = master_hd_key.derive("m/44h/60h/0h/0/0".to_string())?;
//! let address_key2 = eth_first_account_key.derive("m/44'/60'/0'/0/0".to_string())?;
//! assert_eq!(address_key1, address_key2);
//! # Ok(())
//! # }
//! ```
//!
//! A shortcut way to get a derived [HDKey] directly from a master seed, with a specified [HDNetworkType] and a derivation path is to use the [`HDKey::new`] method.
//!
//! ```
//! # use walletd_hd_key::prelude::*;
//! # use slip44::{Coin, Symbol};
//! # fn main() -> Result<(), walletd_hd_key::Error> {
//! # let seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! let custom_key_path = HDPath::builder()
//! .purpose_index(HDPurpose::BIP84.to_shortform_num())
//! .coin_type_index(Coin::Testnet.id())
//! .account_index(0)
//! .change_index(1)
//! .address_index(0)
//! .hardened_address()
//! .build()
//! .to_string();
//!
//! assert_eq!(custom_key_path, "m/84'/1'/0'/1/0'");
//! let derived_key = HDKey::new(
//!    seed,
//!    HDNetworkType::TestNet,
//!    custom_key_path,
//! )?;
//!println!("derived_key: {:?}", derived_key);
//! println!("derived_key depth: {}", derived_key.depth());
//!println!("derived_key wif: {}", derived_key.to_wif()?);
//!println!(
//!   "derived_key public key: {}",
//!   derived_key.extended_public_key_serialized()?
//!);
//!println!(
//!   "derived_key private key: {}",
//!  derived_key.extended_private_key_serialized()?
//!);
//!  // Can also just display the bytes in the extended private and public keys as a hex
//! println!(
//!   "derived_key public key hex: {:x}",
//!  derived_key.extended_public_key()?);
//! println!(
//!     "derived_key private key hex: {:x}",
//!    derived_key.extended_private_key()?);
//! # Ok(())
//! # }
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
