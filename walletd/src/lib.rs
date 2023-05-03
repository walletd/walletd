//! # WalletD
//!
//! A cryptocurrency wallet library that encapsulates Rust-based functionality for various different cryptocurrencies and blockchains.
//! Contains features to handle creating and importing wallets, checking balances, and sending and receiving transactions.
//! Provides a common interface for interacting with different cryptocurrencies and blockchains.
//! Built to facilitate and simplify development and implementation of multi-cryptocurrency non-custodial wallets.
//!
//! # Quickstart Guide
//!
//! A good way to access many of the different features of this walletD library is through the use of the [KeyPair] struct which can enable a user to create a HD (Hierarchical Deterministic) wallet from a mnemonic phrase that could be used with multiple cryptocurrencies.
//! The [KeyPairBuilder] struct which can be accessed with default settings through [`KeyPair::builder()`] is a versatile way to specify options for and build a [KeyPair] struct.
//!
//! You can use the [KeyPairBuilder] to specify the mnemonic phrase, mnemonic seed, passphrase, network type, and the mnemonic key pair type.
//! The default specifications for the [KeyPairBuilder] are: no mnemonic phrase, no mnemonic seed, no passphrase, mainnet network type, and BIP39 mnemonic key pair type.
//! To use the testnet network, specify the network type as [HDNetworkType::TestNet] using the [`network_type method`][KeyPairBuilder::network_type()] on a [KeyPairBuilder] object.
//! You need to either specify a mnemonic seed or a mnemonic phrase to build a [KeyPair] struct. If a mnemonic phrase is specified, the mnemonic seed is derived from the mnemonic phrase and the optional passphrase.
//! If the mnemonic seed is specified, any specifications for the mnemonic phrase and passphrase are ignored when deriving a HD wallet key, but any specifications given for these attributes are stored on the [KeyPair] struct.
//!
//! ***Warning**
//!
//!
//!
//! ```
//! use walletd::prelude::*;
//! use walletd::Error;
//! fn main() -> Result<(), Error> {
//! let seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! let keypair = KeyPair::builder().mnemonic_seed(seed).network_type(HDNetworkType::TestNet).build()?;
//!
//! Ok(())
//! }
//! ```
//!
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use walletd_bip39::{
    Bip39Language, Bip39Mnemonic, Bip39MnemonicBuilder, Bip39MnemonicType, Seed,
};
pub use walletd_mnemonics_core::{Language, Mnemonic, MnemonicBuilder};

mod keypair;
pub use keypair::{KeyPair, KeyPairBuilder, MnemonicKeyPairType};

#[doc(hidden)]
pub use walletd_bitcoin::blockstream;

pub use walletd_bitcoin::blockstream::Blockstream;
pub use walletd_bitcoin::{BitcoinAmount, BitcoinWallet};

pub use walletd_coin_core::ConnectorType;

pub use walletd_coin_core::{
    BlockchainConnector, BlockchainConnectorBuilder, CryptoAddress, CryptoAmount, CryptoWallet,
    CryptoWalletBuilder,
};
pub use walletd_ethereum::{EthClient, EthereumAmount, EthereumWallet};
pub use walletd_hd_key::{HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose};
pub use {
    walletd_bip39, walletd_bitcoin, walletd_coin_core, walletd_ethereum, walletd_hd_key,
    walletd_mnemonics_core,
};

mod crypto_coin;
pub use crypto_coin::CryptoCoin;

mod error;
pub use error::Error;
pub mod prelude;
