//! # WalletD Bip39
//!
//!
//! A Rust library implementation of the [BIP39][bip39-standard] standard for HD wallet mnemonic phrases.
//! Facilitates generating and importing BIP39 mnemonic phrases and seeds.
//!
//! [bip39-standard]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
//!
//! # Quickstart
//! A quick way to access the different features related to the BIP39 mnemonic in this walletD library is to make use of the [Bip39Mnemonic] builder [`Bip39MnemonicBuilder`] which can be also be accessed through [`Bip39Mnemonic::builder()`].
//!
//! Here's how you can create a new randomly generated BIP39 mnemonic as per your specifications:
//! 
//!  ```
//! use walletd_bip39::prelude::*;
//! # fn main() -> Result<(), ParseMnemonicError> {
//! // Generate a new BIP39 mnemonic using the default language of English and the default mnemonic type of 12 words
//! let mnemonic = Bip39Mnemonic::builder().build()?;
//! println!("mnemonic phrase: {}", mnemonic.phrase());
//! // You can get the mnemonic seed from the Bip39Mnemonic struct using the [`to_seed`][Seed::to_seed] method
//! println!("mnemonic seed hex: {:x}", mnemonic.to_seed());  // can use the hex format specifier to print the seed as hex
//! println!("mnemonic seed as bytes: {:?}", mnemonic.to_seed().as_bytes()); // can use the [`as_bytes`][Seed::as_bytes] method to get the seed as a byte array
//! # assert_eq!(mnemonic.language(), Bip39Language::English);
//! # assert_eq!(mnemonic.mnemonic_type(), Bip39MnemonicType::Words12);
//! 
//! // Generate a new BIP39 mnemonic with the default language of English but specify the BIP39 mnemonic type of 24 words in the phrase
//! let mnemonic = Bip39Mnemonic::builder().mnemonic_type(Bip39MnemonicType::Words24).build()?;
//! println!("mnemonic phrase: {}", mnemonic.phrase());
//! println!("mnemonic seed hex: {:x}", mnemonic.to_seed());
//! println!("mnemonic seed as bytes: {:?}", mnemonic.to_seed().as_bytes());
//! # Ok(())
//! # }
//! ```
//! 
//!  
//! Create a new randomly generated mnemonic phrase using the default language of English and the default mnemonic type of 12 words
//!     - Using the builder with no specifications and default options to create a new mnemonic
//!         - The default language is English and the default mnemonic type is 12 words
//!         - The BIP39 mnemonic ([Bip39Mnemonic]) is generated randomly and can be displayed as a string 
//!         - The [Bip39Mnemonic] can be converted to a [Seed] type and displayed as a hex string or have the raw bytes returned
//! 
//! ```
//! # use walletd_bip39::{Bip39Language, Bip39Mnemonic, MnemonicExt, MnemonicBuilder, Bip39MnemonicType, ParseMnemonicError};
//! # fn main() -> Result<(), ParseMnemonicError> {
//! let mnemonic = Bip39Mnemonic::builder().build()?;
//! println!("mnemonic phrase: {}", mnemonic.phrase());
//! println!("mnemonic seed hex: {:x}", mnemonic.to_seed());
//! println!("mnemonic seed as bytes: {:?}", mnemonic.to_seed().as_bytes());
//! 
//! # Ok(())
//! # }
//! ```
//! 
//! 2. Restore a mnemonic phrase from a string and passphrase
//!    - Using the builder with default options while specifying the mnemonic phrase and passphrase to restore the mnemonic
//!         - The default language is English, the mnemonic type will be detected from the number of words in the mnemonic phrase 
//!         - Providing a passphrase is optional, but the if a passphrase was used when first creating the mnemonic, the same passphrase must be provided when recovering the mnemonic
//!         - The mnemonic phrase, seed hex, and the seed bytes of a recovered mnemonic can be displayed in the same manner as for a newly generated mnemonic
//! ```
//! # use walletd_bip39::{Bip39Language, Bip39Mnemonic, MnemonicExt, MnemonicBuilder, Bip39MnemonicType, ParseMnemonicError};
//! # fn main() -> Result<(), ParseMnemonicError> {
//! let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! let passphrase: &str = "mypassphrase";
//! let restored_mnemonic = Bip39Mnemonic::builder().mnemonic_phrase(mnemonic_phrase).passphrase(passphrase).build()?;
//! # assert_eq!(restored_mnemonic.phrase(), mnemonic_phrase);
//! println!("mnemonic phrase: {}", restored_mnemonic.phrase());
//! println!("mnemonic seed hex: {:x}", restored_mnemonic.to_seed());
//! println!("mnemonic seed as bytes: {:?}", restored_mnemonic.to_seed().as_bytes());
//! 
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
