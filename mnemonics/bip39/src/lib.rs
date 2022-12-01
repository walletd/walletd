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
//! use bip_39::{Mnemonic, MnemonicType, Language};
//!
//! /// create a new randomly generated mnemonic phrase
//! let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words12, None);
//! /// get the wallet seed
//! let seed = mnemonic.get_seed();
//!
//! // get the HD wallet seed as raw bytes
//! let restored_mnemonic = Mnemonic::from_phrase(Language::English, "outer ride neither foil glue number place usage ball shed dry point", None).unwrap();
//! println!("{}", restored_mnemonic.to_string());
//! ```
//!
pub mod language;
pub mod mnemonic;
pub mod mnemonic_type;

#[doc(inline)]
pub use self::language::Language;
pub use self::language::WordList;
#[doc(inline)]
pub use self::mnemonic::Mnemonic;
pub use self::mnemonic_type::MnemonicType;

pub use walletd_mnemonics::MnemonicHandler;