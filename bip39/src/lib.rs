//! This is a Rust implementation of the [bip39][bip39-standard] standard for Bitcoin HD wallet
//! mnemonic phrases.
//!
//! `bip39` is a walletd Rust library for generating BIP39 mnemonic phrases and converting them into 128 / 256 bit BIP-39 compliant seed strings.
//!
//! [bip39-standard]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
//!
//! ## Quickstart
//!
//! ```rust
//! use walletd_bip39::Language::English;
//! use walletd_bip39::Mnemonic;
//! use walletd_bip39::MnemonicType;
//! use crate::walletd_bip39::MnemonicHandler;
//!
//! /// create a new randomly generated mnemonic phrase
//! let passphrase: &str = "mypassphrase";
//! let mnemonic = walletd_bip39::Mnemonic::new(English, MnemonicType::Words12, Some(passphrase));
//! /// get the wallet seed
//! let seed = mnemonic.get_seed();
//!
//! // get the HD wallet seed as raw bytes
//! let mnemonic_phrase = &"outer ride neither foil glue number place usage ball shed dry point";
//! let passphrase: Option<&str> = Some("mypassphrase");
//! let restored_mnemonic = <walletd_bip39::Mnemonic as MnemonicHandler>::from_phrase(English, mnemonic_phrase, passphrase);
//! ```
pub mod language;
pub mod mnemonic;
pub mod mnemonic_type;

#[doc(inline)]
pub use self::language::Language;
pub use self::language::WordList;
#[doc(inline)]
pub use self::mnemonic::Mnemonic;
pub use self::mnemonic_type::MnemonicType;

pub use mnemonic_model::MnemonicHandler;