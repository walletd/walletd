//!
//! This is a Rust implementation of the Monero Mnemonic Seed standard for Monero Wallets
//! mnemonic phrases.
//!
//!
//! ## Quickstart
//!
//! ```rust
//! use walletd_monero_mnemonic::Language::English;
//! use walletd_monero_mnemonic::Mnemonic;
//! use walletd_monero_mnemonic::MnemonicType;
//! use walletd_monero_mnemonic::MnemonicHandler;
//!
//! /// create a new randomly generated mnemonic phrase
//! let passphrase: &str = "mypassphrase";
//! let mnemonic = walletd_monero_mnemonic::Mnemonic::new(English, MnemonicType::Words13, Some(passphrase));
//! /// get the wallet seed
//! let seed = mnemonic.get_seed();
//!
//! // get the HD wallet seed as raw bytes
//! let mnemonic_phrase = &"outer ride neither foil glue number place usage ball shed dry point";
//! let passphrase: Option<&str> = Some("mypassphrase");
//! let restored_mnemonic = <walletd_monero_mnemonic::Mnemonic as MnemonicHandler>::from_phrase(English, mnemonic_phrase, passphrase);
//! ```
pub mod language;
pub mod mnemonic;
pub mod mnemonic_type;

#[doc(inline)]
pub use language::Language;
pub use language::WordList;
#[doc(inline)]
pub use self::mnemonic::Mnemonic;

pub use self::mnemonic_type::MnemonicType;

pub use walletd_mnemonics::MnemonicHandler;