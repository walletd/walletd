//! # WalletD
//!
//! A cryptocurrency wallet library that encapsulates Rust-based functionality for various different cryptocurrencies and blockchains.
//! Contains features to handle creating and importing wallets, checking balances, and sending and receiving transactions.
//! Provides a common interface for interacting with different cryptocurrencies and blockchains.
//! Built to facilitate and simplify development and implementation of multi-cryptocurrency non-custodial wallets.
//!
//! ## Quickstart Guide
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
//! **Warning**:
//! The information in the [KeyPair] struct should be treated as sensitive information and should be stored and handled securely, especially if it is being used to store real funds.
//!
//! The [KeyPair] struct contains the mnemonic phrase, mnemonic seed, and passphrase if specified, as well as the network type and the mnemonic key pair type.
//!
//! ### Create HD KeyPair from Bip39 Mnemonic
//!
//! Here's how you can create a [KeyPair].
//! ```
//! use walletd::prelude::*;  

//! fn main() -> Result<(), walletd::Error> {
//! let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//! let seed = mnemonic.to_seed("");
//! let seed = Seed::new(seed.to_vec());
//! println!("seed_hex: {:x}", seed);
//! let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//! let keypair = KeyPair::builder().mnemonic_phrase(mnemonic_phrase.into()).network_type(HDNetworkType::TestNet).build()?;
//! assert_eq!(keypair.to_master_key(), master_hd_key);
//! Ok(())
//! }
//! ```
//!
//! ### Derive Wallets
//!
//! The method can be used to derive a [cryptowallet][CryptoWallet] for a specific cryptocurrency from a [KeyPair].
//! You can specify a concrete struct that implements the [CryptoWallet] trait such as [BitcoinWallet]  or [EthereumWallet] to derive a cryptowallet from the `keypair` of the specified concrete type.
//! ```
//! use walletd::prelude::*;
//! use walletd_bitcoin::prelude::*;
//! use walletd_ethereum::prelude::*;
//! fn main() -> Result<(), walletd::Error> {
//! let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//! let seed = mnemonic.to_seed("");
//! let seed = Seed::new(seed.to_vec());
//! println!("seed_hex: {:x}", seed);
//! let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//! let keypair = KeyPair::builder().mnemonic_phrase(mnemonic_phrase.into()).network_type(HDNetworkType::TestNet).build()?;
//! let mut btc_wallet = BitcoinWalletBuilder::new().master_hd_key(keypair.to_master_key()).build().unwrap();
//! let mut eth_wallet = EthereumWalletBuilder::new().master_hd_key(keypair.to_master_key()).build().unwrap();
//! Ok(())
//! }
//! ```
//! ### Specify Blockchain Connectors
//!
//! A valid [blockchain client][BlockchainConnector] is a concrete instance of a struct that implements the [BlockchainConnector] trait.
//! You can setup a [Blockstream] blockchain client to access the Bitcoin blockchain and an [EthClient] blockchain client to access the Ethereum blockchain.
//! Specifying a valid endpoint url is required for the [Blockstream] and [EthClient] blockchain clients.
//! To associate an existing instance of a [cryptowallet][CryptoWallet] with a [blockchain client][BlockchainConnector], use the [`set_blockchain_client`][CryptoWallet::set_blockchain_client] method on the [cryptowallet][CryptoWallet] object.
//!
//! ```no_run
//! # use walletd::prelude::*;
//! # use walletd_bitcoin::prelude::*;
//! # use walletd_ethereum::prelude::*;
//! # fn main() -> Result<(), walletd::Error> {
//! # let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! # let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//! # let seed = mnemonic.to_seed("");
//! # let seed = Seed::new(seed.to_vec());
//! # println!("seed_hex: {:x}", seed);
//! # let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//! # let keypair = KeyPair::builder().mnemonic_phrase(mnemonic_phrase.into()).network_type(HDNetworkType::TestNet).build()?;
//! let mut btc_wallet = BitcoinWalletBuilder::new().master_hd_key(keypair.to_master_key()).build().unwrap();
//! let mut eth_wallet = EthereumWalletBuilder::new().master_hd_key(keypair.to_master_key()).build().unwrap();
//! btc_wallet.set_blockchain_client(Box::new(Blockstream::new("https://blockstream.info/testnet/api")?));
//! eth_wallet.set_blockchain_client(EthClient::new("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")?);
//!
//! # Ok(())
//! # }
//! ```
//!
//! ### Use the CryptoWallets
//! Once you have a [cryptowallet][CryptoWallet] object associated with a [blockchain client] you can use the [cryptowallet][CryptoWallet] to access blockchain data.
//! Any object that implements the [CryptoWallet] trait must implement functions within the trait which include [`balance`][CryptoWallet::balance], and [`transfer`][CryptoWallet::transfer].
//!
//! ```no_run
//! # use walletd::prelude::*;
//! # use walletd_bitcoin::prelude::*;
//! # use walletd_ethereum::prelude::*;
//! # async fn cryptowallets() -> Result<(), walletd::Error> {
//! # let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! # let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//! # let seed = mnemonic.to_seed("");
//! # let seed = Seed::new(seed.to_vec());
//! # println!("seed_hex: {:x}", seed);
//! # let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//! # let keypair = KeyPair::builder().mnemonic_phrase(mnemonic_phrase.into()).network_type(HDNetworkType::TestNet).build()?;
//! let mut btc_wallet = BitcoinWalletBuilder::new()
//! .master_hd_key(keypair.to_master_key())
//! .build()
//! .unwrap();
//! let mut eth_wallet = EthereumWalletBuilder::new().master_hd_key(keypair.to_master_key()).build().unwrap();
//! # btc_wallet.set_blockchain_client(Box::new(Blockstream::new("https://blockstream.info/testnet/api")?));
//! # eth_wallet.set_blockchain_client(EthClient::new("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")?);
//! btc_wallet.sync().await?;
//! println!("btc_wallet balance: {} BTC", btc_wallet.balance().await?.btc());
//! print!("eth_wallet public address: {}", eth_wallet.public_address());
//! let eth_client = EthClient::new("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")?;
//! eth_wallet.set_blockchain_client(eth_client);
//! println!("eth_wallet balance: {} ETH", eth_wallet.balance().await?.eth());
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use bdk::keys::bip39::Mnemonic;
pub use walletd_mnemonics_core::Seed;

mod keypair;
pub use keypair::{KeyPair, KeyPairBuilder, MnemonicKeyPairType};
mod keystore;
pub use keystore::KeyStore;

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
    walletd_bitcoin, walletd_coin_core, walletd_ethereum, walletd_hd_key, walletd_mnemonics_core,
};

mod crypto_coin;
pub use crypto_coin::CryptoCoin;

mod error;
pub use error::Error;
pub mod prelude;
