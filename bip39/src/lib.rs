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
//! use walletd_bip39::{Language, Mnemonic, MnemonicHandler, MnemonicType};
//!
//! // Create a new randomly generated mnemonic phrase
//! let passphrase: &str = "mypassphrase";
//! let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words12, Some(passphrase));
//! println!("phrase: {}", mnemonic);
//!
//! // Get the wallet seed
//! let seed = mnemonic.to_seed();
//! println!("seed hex: {}", seed);
//!
//! // Get the HD wallet seed as raw bytes
//! let mnemonic_phrase: &str =
//!     "outer ride neither foil glue number place usage ball shed dry point";
//! let passphrase: Option<&str> = Some("mypassphrase");
//! let restored_mnemonic =
//!     Mnemonic::from_phrase(Language::English, mnemonic_phrase, passphrase).unwrap();
//! let seed = restored_mnemonic.to_seed();
//! println!("seed as bytes: {:?}", seed.as_bytes());
//! ```
#![forbid(unsafe_code)]

mod language;
mod mnemonic;
mod mnemonic_type;

pub use walletd_mnemonic_model::{MnemonicHandler, Seed};

#[doc(inline)]
pub use self::language::Language;
use self::language::WordList;
pub use self::mnemonic::Mnemonic;
#[doc(inline)]
pub use self::mnemonic_type::MnemonicType;
