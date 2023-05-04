//! # WalletD BIP39
//!
//!
//! A Rust library implementation of the [`BIP39 standard`](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) for HD wallet mnemonic phrases.
//! Facilitates generating and importing BIP39 mnemonic phrases and seeds.
//!
//! ## Quickstart Guide
//! A good way to access the different features related to the BIP39 mnemonic in this walletD library is to make use of the [Bip39Mnemonic] builder ([Bip39MnemonicBuilder]) which can be also be accessed with the default settings through [`Bip39Mnemonic::builder()`].
//!
//! ### Create with Defaults
//! The default specifications for the [Bip39MnemonicBuilder] are: [English language][Bip39Language::English], [12 words in the mnemonic phrase][Bip39MnemonicType::Words12], and no passphrase specified.
//! You can get the mnemonic seed from the [Bip39Mnemonic] struct using the [`to_seed`][Bip39Mnemonic::to_seed] method.
//!
//! Here's how you can create a new randomly generated BIP39 mnemonic using the default specifications.
//!
//!  ```
//! use walletd_bip39::prelude::*;
//!
//! fn bip39_mnemonics() -> Result<(), walletd_bip39::Error> {
//! let mnemonic = Bip39Mnemonic::builder().build()?;
//! // display the generated mnemonic phrase
//! println!("mnemonic phrase: {}", mnemonic.phrase());
//! // can use the hex format specifier to print the seed as hex
//! println!("mnemonic seed hex: {:x}", mnemonic.to_seed());
//! // can use the as_bytes method to get the seed as a byte array
//! println!("mnemonic seed as bytes: {:?}", mnemonic.to_seed().as_bytes());
//! # assert_eq!(mnemonic.language(), Bip39Language::English);
//! # assert_eq!(mnemonic.mnemonic_type(), Bip39MnemonicType::Words12);
//! Ok(())
//! }
//! ```
//! ### Specify Options
//! You can override the default specifications by providing your desired specifications to the builder.
//! You can also reuse the [Bip39MnemonicBuilder] object in a mutable way to create multiple BIP39 mnemonics and even override previous specifications.
//! ```
//! # use walletd_bip39::prelude::*;
//! # fn create_mnemonics() -> Result<(), walletd_bip39::Error> {
//! let mut mnemonic_builder = Bip39Mnemonic::builder();
//!
//! // specify that the mnemonic phrase should consist of 24 words
//! let mnemonic_1 = mnemonic_builder.mnemonic_type(Bip39MnemonicType::Words24).build()?;
//! println!("mnemonic_1 phrase: {}", mnemonic_1.phrase());
//! println!("mnemonic_1 seed hex: {:x}", mnemonic_1.to_seed());
//! # assert_eq!(mnemonic_1.mnemonic_type(), Bip39MnemonicType::Words24);
//! // see the number of entropy bits for the specified mnemonic type
//! # assert_eq!(mnemonic_1.mnemonic_type().entropy_bits(), 256);
//! println!("mnemonic_1 number of entropy bits: {}", mnemonic_1.mnemonic_type().entropy_bits());
//!
//! // reuse builder but now specify 18 words in the mnemonic phrase
//! let mnemonic_2 = mnemonic_builder.mnemonic_type(Bip39MnemonicType::Words18).build()?;
//! # assert_eq!(mnemonic_2.mnemonic_type(), Bip39MnemonicType::Words18);
//! # assert_eq!(mnemonic_2.mnemonic_type().entropy_bits(), 192);
//! println!("mnemonic_2 phrase: {}", mnemonic_2.phrase());
//! println!("mnemonic_2 seed hex: {:x}", mnemonic_2.to_seed());
//! println!("mnemonic_2 number of entropy bits: {}", mnemonic_2.mnemonic_type().entropy_bits());
//!
//! # Ok(())
//! # }
//! ```
//!
//! It may be useful in some cases to provide all of the specifications even when using the some of the default settings.
//!
//! ### Use of Optional Passphrase
//! You can specify a passphrase to use when generating the mnemonic.
//! Note that the same passphrase must be used when recovering the mnemonic.
//!
//! **Warning:**
//! If a [Bip39Mnemonic] mnemonic phrase is generated using a specification of passphrase, both the mnemonic phrase and the passphrase is required to recover the [Bip39Mnemonic].
//! The specified passphrase is not stored in the [Bip39Mnemonic] struct. It is important to store the passphrase you specified securely as well as the mnemonic phrase to enable recovery of the [Bip39Mnemonic].
//!
//! ```
//! # use walletd_bip39::prelude::*;
//! # fn make_mnemonic_with_passphrase() -> Result<(), walletd_bip39::Error> {
//! let mnemonic_3 = Bip39Mnemonic::builder()
//!     .passphrase("mypassphrase")
//!     .mnemonic_type(Bip39MnemonicType::Words12)
//!     .language(Bip39Language::English)
//!     .build()?;
//! # assert_eq!(mnemonic_3.mnemonic_type(), Bip39MnemonicType::Words12);
//! # assert_eq!(mnemonic_3.language(), Bip39Language::English);
//! println!("mnemonic_3 phrase: {}", mnemonic_3.phrase());
//! println!("mnemonic_3 seed hex: {:x}", mnemonic_3.to_seed());
//! # Ok(())
//! }
//! ```
//!  
//! ### Restoring a Mnemonic
//!
//! A [Bip39Mnemonic] can be restored from a specified valid mnemonic phrase or from a specified valid mnemonic phrase and passphrase if a passphrase was specified when the mnemonic was generated.
//!
//! ```
//! # use walletd_bip39::prelude::*;
//! # fn restore() -> Result<(), walletd_bip39::Error> {
//! let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! let restored_mnemonic_1 = Bip39Mnemonic::builder().mnemonic_phrase(mnemonic_phrase).build()?;
//! # assert_eq!(restored_mnemonic_1.mnemonic_type(), Bip39MnemonicType::Words12);
//! # assert_eq!(restored_mnemonic_1.language(), Bip39Language::English);
//! # assert_eq!(restored_mnemonic_1.phrase(), mnemonic_phrase);
//! # let seed_hex_1 = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
//! # assert_eq!(format!("{:x}", restored_mnemonic_1.to_seed()), seed_hex_1);
//! println!("restored_mnemonic_1 phrase: {}", restored_mnemonic_1.phrase());
//! println!("restored_mnemonic_1 seed hex: {:x}", restored_mnemonic_1.to_seed());
//!
//! let specified_passphrase = "mypassphrase";
//! let restored_mnemonic_2 = Bip39Mnemonic::builder().mnemonic_phrase(mnemonic_phrase).passphrase(specified_passphrase).build()?;
//! # assert_eq!(restored_mnemonic_2.mnemonic_type(), Bip39MnemonicType::Words12);
//! # assert_eq!(restored_mnemonic_2.language(), Bip39Language::English);
//! # assert_eq!(restored_mnemonic_2.phrase(), mnemonic_phrase);
//! # let seed_hex_2 = "3c536b023d71d81e6abc58b0b91c64caff8bb08fabf0c9f3cf948a9f3a494e8ecb0790b6e933834796c930a2d437170bd6071c00bc0553d06235d02315f2c229";
//! # assert_eq!(format!("{:x}", restored_mnemonic_2.to_seed()), seed_hex_2);
//! println!("restored_mnemonic_2 phrase: {}", restored_mnemonic_2.phrase());
//! println!("restored_mnemonic_2 seed hex: {:x}", restored_mnemonic_2.to_seed());
//! # Ok(())
//! # }
//! ```
//!
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
