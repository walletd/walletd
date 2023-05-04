//! # WalletD Bitcoin Library
//!
//! Provides a wallet implementation for Bitcoin including the ability to create a new wallet or import an existing wallet, check balances, and handle transactions.
//! It supports a hierarchical deterministic (HD) wallet structure and provides the ability to search for previously used addresses associated with the wallet as well as the creation of new addresses.
//! It also facilitates obtaining blockchain information.
//!
//! # Quickstart Guide
//!
//! The [BitcoinWallet] struct is the main entry point for the library.
//!
//! Here's how you can access a bitcoin wallet based on a master [HDKey].
//! ```
//! use walletd_bitcoin::prelude::*;
//! use walletd_hd_key::prelude::*;
//! fn import_btc_hd_wallet() -> Result<(), walletd_bitcoin::Error> {
//! let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! let network_type = HDNetworkType::TestNet;
//! let master_hd_key = HDKey::new_master(master_seed, network_type)?;
//! let btc_wallet = BitcoinWallet::builder().master_hd_key(master_hd_key).build()?;
//!
//! Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use bitcoin;
pub use bitcoin::{
    Address as AddressInfo, AddressType, Network, PrivateKey as BitcoinPrivateKey,
    PublicKey as BitcoinPublicKey, Script,
};

mod bitcoin_address;
pub use bitcoin_address::BitcoinAddress;
mod bitcoin_wallet;
pub use bitcoin_wallet::{BitcoinWallet, BitcoinWalletBuilder};
mod bitcoin_amount;
pub use bitcoin_amount::BitcoinAmount;
pub mod blockstream;
mod error;
pub use error::Error;
pub use walletd_bip39::{
    Bip39Language, Bip39Mnemonic, Bip39MnemonicType, Mnemonic, MnemonicBuilder, Seed,
};
pub use walletd_coin_core::{
    BlockchainConnector, CryptoAddress, CryptoAmount, CryptoWallet, CryptoWalletBuilder,
};
pub use walletd_hd_key::{HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose};
pub mod prelude;
