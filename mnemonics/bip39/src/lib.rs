//! This is a Rust implementation of the [bip39][bip39-standard] standard for
//! Bitcoin HD wallet mnemonic phrases.
//!
//! `bip39` is a walletd Rust library for generating BIP39 mnemonic phrases and
//! converting them into 128 / 256 bit BIP-39 compliant seed strings.
//!
//! [bip39-standard]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
//!
//! # Quickstart
//!
//! ```rust
//! use walletd_bip39::{Bip39Language, Bip39Mnemonic, Mnemonic, MnemonicBuilder, Bip39MnemonicBuilder, Bip39MnemonicType};
//!
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! // Create a new randomly generated mnemonic phrase
//! let passphrase: &str = "mypassphrase";
//! let mnemonic = Bip39Mnemonic::builder().passphrase(passphrase).build()?;
//!
//! println!("phrase: {:?}", mnemonic);
//!
//! // Get the wallet seed
//! let seed = mnemonic.to_seed();
//! println!("seed hex: {}", seed);
//!
//! // Get the HD wallet seed as raw bytes
//! let mnemonic_phrase: &str =
//!     "outer ride neither foil glue number place usage ball shed dry point";
//! let passphrase: &str = "mypassphrase";
//! let restored_mnemonic = Bip39Mnemonic::builder()
//!     .mnemonic_phrase(mnemonic_phrase)
//!     .passphrase(passphrase)
//!     .build()?;
//!
//! let seed = restored_mnemonic.to_seed();
//! println!("seed as bytes: {:?}", seed.as_bytes());
//!
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod error;
mod language;
mod mnemonic;
mod mnemonic_type;

pub use self::error::Error;
pub use self::language::Bip39Language;
pub use self::language::WordList;
pub use self::mnemonic::{Bip39Mnemonic, Bip39MnemonicBuilder};
pub use self::mnemonic_type::Bip39MnemonicType;
pub use walletd_mnemonics_core::{Mnemonic, MnemonicBuilder, Seed};
pub mod prelude;
