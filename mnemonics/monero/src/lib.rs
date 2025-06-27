//! This is a Rust implementation of the Monero Mnemonic Seed standard for
//! Monero Wallets mnemonic phrases.
//!
//!
//! ## Quickstart
//!
//! ```rust
//! use walletd_monero_mnemonic::{Language, Mnemonic, MnemonicExt, MnemonicStyleBuilder, MnemonicType};
//!
//! // create a new randomly generated mnemonic phrase
//! let mnemonic = Mnemonic::builder().generate().unwrap();
//! println!("phrase: {:?}", mnemonic);
//!
//! /// get the wallet seed
//! let seed = mnemonic.to_seed();
//! println!("seed hex: {}", seed);
//!
//! // get the HD wallet seed as raw bytes
//! let mnemonic_phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
//! let restored_mnemonic = Mnemonic::builder().set_phrase(mnemonic_phrase).restore().unwrap();
//! let seed = restored_mnemonic.to_seed();
//! println!("seed as bytes: {:?}", seed.as_bytes());
//! ```
#![forbid(unsafe_code)]

mod errors;
mod language;
mod mnemonic;
mod mnemonic_type;

pub use language::Language;
pub use walletd_mnemonics_core::Seed;
pub use self::errors::Error;
use self::language::WordList;
pub use self::mnemonic::{Mnemonic, MnemonicBuilder};
pub use self::mnemonic_type::MnemonicType;
