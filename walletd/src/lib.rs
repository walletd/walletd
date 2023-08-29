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
//! The method can be used to derive a wallet for a specific cryptocurrency from a [KeyPair].
//! You can specify a concrete struct such as [BitcoinWallet]  or [EthereumWallet] to derive a cryptowallet from the `keypair` of the specified concrete type.
//! ```
//! use walletd::prelude::*;
//! use walletd_bitcoin::prelude::*;
//! use walletd_ethereum::prelude::*;
//! use bdk::bitcoin::Network;
//! fn main() -> Result<(), walletd::Error> {
//! let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//!
//! let mut btc_wallet = BitcoinWalletBuilder::new().mnemonic(mnemonic.clone()).network_type(Network::Testnet).build().unwrap();
//! let mut eth_wallet = EthereumWalletBuilder::new().mnemonic(mnemonic).network_type(HDNetworkType::TestNet).build().unwrap();
//! Ok(())
//! }
//! ```
//! ### Specify Blockchain Connectors
//!
//! You can setup a blockchain client to access the Bitcoin blockchain and an [EthClient] blockchain client to access the Ethereum blockchain.
//! Specifying a valid endpoint url is required for the [EthClient] blockchain clients.
//!
//! ```no_run
//! # use walletd::prelude::*;
//! # use walletd_bitcoin::prelude::*;
//! # use walletd_ethereum::prelude::*;
//! # use bdk::bitcoin::Network;
//! # fn main() -> Result<(), walletd::Error> {
//! # let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! # let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//!
//! let mut btc_wallet = BitcoinWalletBuilder::new().mnemonic(mnemonic.clone()).network_type(Network::Testnet).build().unwrap();
//! let mut eth_wallet = EthereumWalletBuilder::new().mnemonic(mnemonic).network_type(HDNetworkType::TestNet).build().unwrap();
//! let ethclient_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
//! let _eth_client = EthClient::new(ethclient_url)?;
//!
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use bdk::keys::bip39::Mnemonic;
pub use walletd_mnemonics_core::Seed;

mod keypair;
pub use keypair::{KeyPair, KeyPairBuilder, MnemonicKeyPairType};

pub use walletd_bitcoin::BitcoinWallet;

pub use walletd_ethereum::{EthClient, EthereumAmount, EthereumWallet};
pub use walletd_hd_key::{HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose};
pub use {walletd_bitcoin, walletd_ethereum, walletd_hd_key, walletd_mnemonics_core};

mod error;
pub use error::Error;
pub mod prelude;
