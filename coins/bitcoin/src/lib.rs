//! # WalletD Bitcoin
//!
//! Provides a wallet implementation for Bitcoin including the ability to create a new wallet or import an existing wallet, check balances, and handle transactions.
//! It supports a hierarchical deterministic (HD) wallet structure and provides the ability to search for previously used addresses associated with the wallet as well as the creation of new addresses.
//! It also facilitates obtaining blockchain information.
//!
//! ## Quickstart Guide
//!
//! The [BitcoinWallet] struct is the main entry point to access walletD functionality for Bitcoin.
//!
//! ### Import from Seed
//!
//! Here's how you can access a bitcoin wallet based on a master [Seed].
//! The [Seed] can be derived from a [Mnemonic] using the [Mnemonic::to_seed] method.
//! ```
//! use walletd_bitcoin::prelude::*;
//! use walletd_hd_key::prelude::*;
//! # fn import_btc_hd_wallet() -> Result<(), walletd_bitcoin::Error> {
//! let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! let network_type = HDNetworkType::TestNet;
//! let master_hd_key = HDKey::new_master(master_seed, network_type)?;
//! let mut btc_wallet = BitcoinWallet::builder().master_hd_key(master_hd_key).build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Default Derivation Path
//!
//! Deriving a [BitcoinWallet] is simple and uses some default settings under the hood.
//! When using [BitcoinWalletBuilder], the default settings set the [AddressType] as ['P2wpkh`][bitcoin::AddressType::P2wpkh] and the corresponding default [HDPurpose] for the derivation path is set as [HDPurpose::BIP49].
//!```
//! # use walletd_bitcoin::prelude::*;
//! # use walletd_hd_key::prelude::*;
//! use walletd_bitcoin::bitcoin;
//! # fn import_btc_hd_wallet() -> Result<(), walletd_bitcoin::Error> {
//! # let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let network_type = HDNetworkType::TestNet;
//! # let master_hd_key = HDKey::new_master(master_seed, network_type)?;
//! # let mut btc_wallet = BitcoinWallet::builder().master_hd_key(master_hd_key).build()?;
//!   assert_eq!(btc_wallet.address_format(), bitcoin::AddressType::P2wpkh);
//!   assert_eq!(btc_wallet.hd_path_builder()?.purpose, Some(HDPurpose::BIP84.to_shortform_num()));
//!
//! # Ok(())
//! # }
//!```
//!
//! ### Using Blockstream as Blockchain Connector
//!
//! The [BitcoinWallet] struct can be used to access blockchain data through a [BlockchainConnector] such as [Blockstream].
//! The [Blockstream] instance can be used on its own to access blockchain data such as fee estimates.
//! It can be affiliated with a [BitcoinWallet] to enable the [BitcoinWallet] to access blockchain data and send transactions.
//! ```no_run
//! # use walletd_bitcoin::prelude::*;
//! # use walletd_hd_key::prelude::*;
//! # use walletd_bitcoin::bitcoin;
//! # async fn import_btc_hd_wallet() -> Result<(), walletd_bitcoin::Error> {
//! # let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let network_type = HDNetworkType::TestNet;
//! # let master_hd_key = HDKey::new_master(master_seed, network_type)?;
//! # let mut btc_wallet = BitcoinWallet::builder().master_hd_key(master_hd_key).build()?;
//! let btc_client = Box::new(Blockstream::new("https://blockstream.info/testnet/api")?);
//! let fee_estimates = btc_client.fee_estimates().await?;
//! println!("fee estimates: {:?}", fee_estimates);
//! btc_wallet.set_blockchain_client(btc_client);
//! # Ok(())
//! # }
//! ```
//!
//! ### Sync BitcoinWallet, Load BitcoinAddresses
//!
//! The [BitcoinWallet] struct can be used to sync the wallet with the blockchain and load the associated [BitcoinAddress]es.
//! ```no_run
//! # use walletd_bitcoin::prelude::*;
//! # use walletd_hd_key::prelude::*;
//! # use walletd_bitcoin::bitcoin;
//! # async fn import_btc_hd_wallet() -> Result<(), walletd_bitcoin::Error> {
//! # let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let network_type = HDNetworkType::TestNet;
//! # let master_hd_key = HDKey::new_master(master_seed, network_type)?;
//! # let mut btc_wallet = BitcoinWallet::builder().master_hd_key(master_hd_key).build()?;
//! let btc_client = Box::new(Blockstream::new("https://blockstream.info/testnet/api")?);
//! let fee_estimates = btc_client.fee_estimates().await?;
//! println!("fee estimates: {:?}", fee_estimates);
//! btc_wallet.set_blockchain_client(btc_client);
//! for addr in btc_wallet.associated_info() {
//!     println!("address: {}, derivation path {}",
//!     addr.address.public_address(), addr.hd_key().derivation_path().to_string());
//! }
//! println!("next receive address: {}", btc_wallet.receive_address()?);
//! println!("next change address: {}", btc_wallet.next_change_address()?.public_address());
//!
//!
//!
//! let balance = btc_wallet.balance().await?;
//! println!(
//!     "bitcoin wallet balance: {} BTC, ({} satoshi",
//!     balance.btc(),
//!     balance.satoshi()
//! );
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod connectors;
use async_trait::async_trait;
pub use bitcoin;
pub use bitcoin::{
    Address as AddressInfo, AddressType, Network, PublicKey as BitcoinPublicKey, Script,
};

mod bitcoin_address;
pub use bitcoin_address::{BitcoinAddress, BitcoinPrivateKey};
mod bitcoin_wallet;
pub use bitcoin_wallet::{BitcoinWallet, BitcoinWalletBuilder};
mod bitcoin_amount;
pub use bitcoin_amount::BitcoinAmount;

#[doc(hidden)]
pub mod blockstream;
pub mod mempool_space;
pub use blockstream::Blockstream;
pub use mempool_space::MempoolSpace;

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

use crate::connectors::{BTransaction, FeeEstimates, Utxos};

/// Bitcoin Blockchain Connector which allows the creation of multiple connectors to be created and used by BitcoinWallets
#[async_trait]
pub trait BitcoinConnector {
    /// Checks if the given address has had an past transactions, returns true if it has and false if it has not
    /// Errors if the address is invalid or if the API returns an error
    async fn check_if_past_transactions_exist(&self, public_address: &str) -> Result<bool, Error>;
    /// Fetch the block height
    async fn block_height(&self) -> Result<u64, Error>;
    /// Fetch fee estimates
    async fn fee_estimates(&self) -> Result<FeeEstimates, Error>;
    /// Fetch transactions for the given address
    async fn transactions(&self, address: &str) -> Result<Vec<BTransaction>, Error>;
    /// Fetch mempool transactions
    async fn mempool_transactions(&self, address: &str) -> Result<Vec<BTransaction>, Error>;
    /// Fetch UTXOs
    async fn utxo(&self, address: &str) -> Result<Utxos, Error>;
    /// Fetch raw transaction hex for a given txid
    async fn raw_transaction_hex(&self, txid: &str) -> Result<String, Error>;
    /// Fetch transaction info
    async fn transaction(&self, txid: &str) -> Result<BTransaction, Error>;
    /// Broadcast a raw transaction to the network
    async fn broadcast_tx(&self, raw_transaction_hex: &'static str) -> Result<String, Error>;
}
