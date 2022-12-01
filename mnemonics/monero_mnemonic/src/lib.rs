//!
//! This is a Rust implementation of the Monero Mnemonic Seed standard for Monero Wallets
//! mnemonic phrases.
//!
//!
//! ## Quickstart
//!
//! ```rust
//! use monero_mnemonic::{Mnemonic, Language};
//!
//! /// create a new randomly generated mnemonic phrase
//! let mnemonic = Mnemonic::new(Language::English);
//!
//! /// get the phrase
//!  println!("{}", mnemonic.to_string());
//!
//! /// get the wallet seed
//!
//! // get the HD wallet seed as raw bytes
//! let restored_mnemonic = Mnemonic::from_phrase(Language::English, "toffee tedious awakened vampire corrode deepest washing goggles rowboat technical hesitate building toffee").unwrap();
//! println!("{}", restored_mnemonic.to_string());
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