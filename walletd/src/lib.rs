//! # WalletD
//!
//! This library facilitates the creation and use of non-custodial multi-currency wallets.
//!
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use walletd_bip39::{
    Language as Bip39Language, Mnemonic as Bip39Mnemonic, MnemonicType as Bip39MnemonicType, Seed,
};

pub use walletd_mnemonics_core::{MnemonicExt, MnemonicStyleBuilder};

mod keypair;
pub use keypair::{KeyPair, KeyPairBuilder, MnemonicKeyPairType};
pub use walletd_bitcoin::blockstream;
pub use walletd_bitcoin::{BitcoinAmount, BitcoinWallet};
pub use walletd_coin_core::{
    BlockchainConnector, ConnectorType, CryptoAddress, CryptoAmount,
    CryptoWallet, CryptoWalletBuilder, CryptoWalletGeneral,
};
pub use walletd_ethereum::{EthClient, EthereumAmount, EthereumFormat, EthereumWallet};
pub use walletd_hd_key::{HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose};
pub use {
    walletd_bip39, walletd_bitcoin, walletd_coin_core, walletd_ethereum, walletd_hd_key,
    walletd_mnemonics_core,
};

mod crypto_coin;
pub use crypto_coin::CryptoCoin;

mod error;
pub use error::Error;
