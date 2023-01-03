//!
//! This is a Rust implementation of the Monero Mnemonic Seed standard for Monero Wallets
//! mnemonic phrases.
//!
//!
//! ## Quickstart
//!
//! ```rust
//! use walletd_monero_mnemonic::Language;
//! use walletd_monero_mnemonic::Mnemonic;
//! use walletd_monero_mnemonic::MnemonicHandler;
//! use walletd_monero_mnemonic::MnemonicType;
//!
//! /// create a new randomly generated mnemonic phrase
//! let passphrase: &str = "mypassphrase";
//! let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words25, Some(passphrase));
//! println!("phrase: {}", mnemonic);
//!
//! /// get the wallet seed
//! let seed = mnemonic.to_seed();
//! println!("seed hex: {}", seed);
//!
//! // get the HD wallet seed as raw bytes
//! let mnemonic_phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
//! let passphrase: Option<&str> = Some("mypassphrase");
//! let restored_mnemonic = Mnemonic::from_phrase(Language::English, mnemonic_phrase, passphrase).unwrap();
//! let seed = restored_mnemonic.to_seed();
//! println!("seed as bytes: {:?}", seed.as_bytes());
//! ```
mod language;
mod mnemonic;
mod mnemonic_type;

use self::language::WordList;
#[doc(inline)]
pub use self::mnemonic::Mnemonic;
#[doc(inline)]
pub use language::Language;

pub use self::mnemonic_type::MnemonicType;

pub use walletd_mnemonic_model::MnemonicHandler;
pub use walletd_mnemonic_model::Seed;
