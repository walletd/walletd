//! # WalletD
//!
//! A cryptocurrency wallet library that encapsulates Rust-based functionality for various different cryptocurrencies and blockchains.
//! Contains features to handle creating and importing wallets, checking balances, and sending and receiving transactions.
//! Provides a common interface for interacting with different cryptocurrencies and blockchains.
//! Built to facilitate and simplify development and implementation of multi-cryptocurrency non-custodial wallets.
//!
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use walletd_bip39::{
    Bip39Language, Bip39Mnemonic, Bip39MnemonicBuilder, Bip39MnemonicType, Seed,
};
pub use walletd_mnemonics_core::{Mnemonic, MnemonicBuilder};

mod keypair;
pub use keypair::{KeyPair, KeyPairBuilder, MnemonicKeyPairType};
#[doc(hidden)]
pub use walletd_bitcoin::blockstream;
pub use walletd_bitcoin::blockstream::Blockstream;
pub use walletd_bitcoin::{BitcoinAmount, BitcoinWallet};
#[doc(hidden)]
pub use walletd_coin_core::ConnectorType;
pub use walletd_coin_core::{
    BlockchainConnector, CryptoAddress, CryptoAmount, CryptoWallet, CryptoWalletBuilder,
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
