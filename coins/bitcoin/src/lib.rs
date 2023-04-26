//! # WalletD Bitcoin Library
//!
//! This library provides a wallet implementation for Bitcoin including the ability create a new wallet or import an existing wallet, check balances, and handle transactions.
//! It supports a heirarchical deterministic (HD) wallet structure and provides the ability to search for previously used addresses associated with the wallet as well as the creation of new addresses.
//! It also facilitates obtaining blockchain information.

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
    Language as Bip39Language, Mnemonic as Bip39Mnemonic, MnemonicExt, MnemonicStyleBuilder,
    MnemonicType as Bip39MnemonicType, Seed,
};
pub use walletd_coin_core::{CryptoAddress, CryptoAmount, CryptoWallet, CryptoWalletBuilder};
pub use walletd_hd_key::{HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose};
