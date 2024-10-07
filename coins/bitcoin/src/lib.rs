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
//! ### Import from mnemonic phrase
//!
//! Here's how you can access a bitcoin wallet based on a mnemonic phrase.
//! ```
//! use bdk::bitcoin::Network;
//! use bdk::keys::bip39::Mnemonic;
//! use walletd_bitcoin::prelude::*;
//! fn import_btc_hd_wallet() -> Result<(), walletd_bitcoin::Error> {
//! let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//! let mut btc_wallet = BitcoinWallet::builder().mnemonic(mnemonic).network_type(Network::Testnet).build()?;
//! Ok(())
//! }
//! ```
//!
//! ### Using Blockstream as Blockchain Connector
//!
//! The [BitcoinWallet] struct can be used to access blockchain data.
//! It can be affiliated with a [BitcoinWallet] to enable the [BitcoinWallet] to access blockchain data and send transactions.
//! ```no_run
//! use walletd_bitcoin::prelude::*;
//! use bdk::bitcoin::Network;
//! use bdk::keys::bip39::Mnemonic;
//! use bdk::blockchain::ElectrumBlockchain;
//! use bdk::electrum_client::Client;
//! async fn import_btc_hd_wallet() -> Result<(), walletd_bitcoin::Error> {
//! let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//! let mut btc_wallet = BitcoinWallet::builder().mnemonic(mnemonic).network_type(Network::Testnet).build()?;
//! let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
//! let blockchain = ElectrumBlockchain::from(client);
//! btc_wallet.sync(&blockchain).await?;
//! let balance = btc_wallet.balance().await?;
//! println!("bitcoin wallet balance: {} satoshi", balance.confirmed);
//! Ok(())
//! }
//! ```
//!
//! ### Load BitcoinAddresses
//!
//! The [BitcoinWallet] struct can be used to sync the wallet with the blockchain and load address.
//! ```no_run
//! use bdk::bitcoin::Network;
//! use bdk::keys::bip39::Mnemonic;
//! use walletd_bitcoin::prelude::*;
//! fn import_btc_hd_wallet() -> Result<(), walletd_bitcoin::Error> {
//! let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//! let mut btc_wallet = BitcoinWallet::builder().mnemonic(mnemonic).network_type(Network::Testnet).build()?;
//! println!("next receive address: {}", btc_wallet.receive_address()?);
//! Ok(())
//! }
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod bitcoin_wallet;
pub use bitcoin_wallet::{BitcoinWallet, BitcoinWalletBuilder};

#[doc(hidden)]
mod error;
pub use error::Error;
pub mod prelude;
