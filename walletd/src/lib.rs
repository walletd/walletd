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
//! ```no_run
//! use walletd::prelude::*;  

//! fn main() -> Result<(), walletd::Error> {
//! let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
//! let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
//! let seed = mnemonic.to_seed("");
//! let seed = Seed::new(seed.to_vec());
//! println!("seed_hex: {:x}", seed);
//! let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
//! let keypair = KeyPair::builder().mnemonic_phrase(mnemonic_phrase.into()).build()?;
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
//! let mut eth_wallet = EthereumWalletBuilder::new().mnemonic(mnemonic).build().unwrap();
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
//! let mut eth_wallet = EthereumWalletBuilder::new().mnemonic(mnemonic).build().unwrap();
//!
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use bdk::KeychainKind;
use bdk::bitcoin::AddressType;
use bdk::bitcoin::Network;
use bdk::bitcoin::PrivateKey;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::util::bip32::ExtendedPubKey;
use bdk::database::MemoryDatabase;
pub use bdk::keys::bip39::Mnemonic;
use bdk::template::Bip84;
use derivative::Derivative;
pub use walletd_mnemonics_core::Seed;

mod keypair;
pub use keypair::{KeyPair, KeyPairBuilder, MnemonicKeyPairType};

pub use walletd_bitcoin::BitcoinWallet;

pub use walletd_ethereum::{EthClient, EthereumAmount, EthereumWallet};
// Due to package conflicts, solana is commented out at present

// pub use walletd_solana::solana_client::SolanaClient;
// pub use walletd_solana::solana_account::SolanaAccount;

pub use walletd_hd_key::{HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose};
pub use {walletd_bitcoin, walletd_ethereum, walletd_hd_key, walletd_mnemonics_core};
use async_trait::async_trait;
use auto_impl::auto_impl;
mod error;
pub use error::Error as WalletdError;
use std::sync::Arc;
use std::{fmt::Debug};
use thiserror::Error;
use serde::{de::DeserializeOwned, Serialize, Deserialize};

// Due to package conflicts, solana is commented out at present

// pub use {walletd_bitcoin, walletd_solana, walletd_hd_key, walletd_mnemonics_core};

pub mod prelude;
extern crate savefile;
use savefile::prelude::*;
use std::path::Path;
use ethers::prelude::*;

use bdk::electrum_client::Client;
use bdk::blockchain::ElectrumBlockchain;
use bdk::blockchain::GetHeight;

#[macro_use]
extern crate savefile_derive;

#[derive(Debug)]
pub struct BasicWallet {
    pub address: String,
    pub btc_provider: Option<BtcProvider>,
    pub eth_provider: Option<EthProvider>,
}

impl BasicWallet { 
    pub fn unlock(self, xpriv: ExtendedPrivKey) -> BasicWalletUnlocked {
        BasicWalletUnlocked {
            wallet: self,
            private_key: xpriv,
        }
    }
}

#[derive(Debug)]
pub struct BasicWalletUnlocked {
    pub wallet: BasicWallet,
    pub private_key: ExtendedPrivKey,
}

impl BasicWalletUnlocked { 
    pub fn lock(mut self) -> BasicWallet {
        // zeroize the private key
        BasicWallet { address: self.wallet.address, btc_provider: self.wallet.btc_provider, eth_provider: self.wallet.eth_provider }
    }

    pub fn btc_wallet(self) -> BitcoinWallet {
        let btc_wallet: bdk::Wallet<MemoryDatabase> = bdk::Wallet::new(
            Bip84(self.private_key.clone(), KeychainKind::External),
            Some(Bip84(self.private_key.clone(), KeychainKind::Internal)),
            Network::Testnet,
            MemoryDatabase::new()).unwrap();
        BitcoinWallet {
            wallet: Some(btc_wallet), 
            address_format: AddressType::P2wpkh,
        }
    }
}

#[derive(Debug, Savefile, Serialize, Deserialize)]
pub struct Wallet {
    pub mnemonic: String,
    pub btc_url: String,
    pub eth_url: String,
}

pub enum BlockchainType {
    BTC,
    ETH,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[auto_impl(&, Box, Arc)]
pub trait WalletClient: Debug + Send + Sync {
    /// A JSON-RPC Error
    //type Error: Into<ProviderError> + RpcError;

    /// Sends a request with the provided JSON-RPC and parameters serialized as JSON
    // async fn block_height<T: Debug + Serialize + Send + Sync>(&self, method: &str, params: T) -> U64;
    async fn public_address(&self) -> String;
}

impl<P: WalletClient> WalletProvider<P> {
    /// Instantiate a new provider with a backend.
    pub fn new(provider: P, xpriv: ExtendedPrivKey, xpub: ExtendedPubKey) -> Self {
        Self {
            inner: provider,
            xpriv: xpriv,
            xpub: xpub,
        }
    }
}

#[derive(Debug)]
pub struct EthWallet {
    pub inner: String,
    pub xpriv: ExtendedPrivKey,
    pub xpub: ExtendedPubKey,
}

impl EthWallet {
    /// Instantiate a new provider with a backend.
    pub fn new(provider: String, xpriv: ExtendedPrivKey, xpub: ExtendedPubKey) -> Result<Self, ProviderError> {
        Ok(Self {
            inner: "EthWallet".to_string(),
            xpriv: xpriv,
            xpub: xpub,
        })
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WalletClient for EthWallet {
    async fn public_address(&self) -> String
       {
        self.xpub.public_key.to_string()
    }
}

#[derive(Debug, Error)]
/// An error thrown when making a call to the provider
pub enum ProviderError {
    #[error("ens name not found: {0}")]
    FromHexError(String),
    #[error("ens name not found: {0}")]
    ParseError(String),
    /// An internal error in the JSON RPC Client
    #[error("{0}")]
    JsonRpcClientError(Box<dyn crate::RpcError + Send + Sync>),

    /// An error during ENS name resolution
    #[error("ens name not found: {0}")]
    EnsError(String),

    /// Invalid reverse ENS name
    #[error("reverse ens name not pointing to itself: {0}")]
    EnsNotOwned(String),

    /// Error in underlying lib `serde_json`
    // #[error(transparent)]
    // SerdeJson(#[from] serde_json::Error),

    /// Error in underlying lib `hex`
    #[error(transparent)]
    HexError(#[from] hex::FromHexError),

    /// Error in underlying lib `reqwest`
    // #[error(transparent)]
    // HTTPError(#[from] reqwest::Error),

    /// Custom error from unknown source
    #[error("custom error: {0}")]
    CustomError(String),

    /// RPC method is not supported by this provider
    #[error("unsupported RPC")]
    UnsupportedRPC,

    /// Node is not supported by this provider
    #[error("unsupported node client")]
    UnsupportedNodeClient,

    /// Signer is not available to this provider.
    #[error("Attempted to sign a transaction with no available signer. Hint: did you mean to use a SignerMiddleware?")]
    SignerUnavailable,
}

impl TryFrom<&str> for WalletProvider<EthWallet> {
    type Error = ProviderError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Ok(WalletProvider::<EthWallet>::try_from(src)?)
    }
}

#[derive(Clone, Debug)]
pub struct WalletProvider<P> {
    inner: P,
    xpriv: ExtendedPrivKey,
    xpub: ExtendedPubKey,
}

impl<P> AsRef<P> for WalletProvider<P> {
    fn as_ref(&self) -> &P {
        &self.inner
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[auto_impl(&, Box, Arc)]
pub trait BlockchainClient: Debug + Send + Sync {
    /// A JSON-RPC Error
    //type Error: Into<ProviderError> + RpcError;

    /// Sends a request with the provided JSON-RPC and parameters serialized as JSON
    // async fn block_height<T: Debug + Serialize + Send + Sync>(&self, method: &str, params: T) -> U64;
    async fn block_height(&self) -> U64;
}

pub struct BlockchainProvider<T> {
    pub inner: T,
}

impl<P: BlockchainClient> BlockchainProvider<P> {
    /// Instantiate a new provider with a backend.
    pub fn new(provider: P) -> Self {
        Self {
            inner: provider,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EthProvider {
    pub inner: Provider<Http>,
}

impl EthProvider {
    /// Instantiate a new provider with a backend.
    pub fn new(provider: &str) -> Result<Self, ProviderError> {
        let provider = Provider::try_from(provider).expect("no idea");
        Ok(Self {
            inner: provider,
        })
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BlockchainClient for EthProvider {
    // async fn block_height<T: Debug + Serialize + Send + Sync>(&self, method: &str, params: T) -> U64
    //    {
    //     self.inner.get_block_number().await.unwrap()
    // }
    async fn block_height(&self) -> U64
       {
        self.inner.get_block_number().await.unwrap()
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct BtcProvider {
    #[derivative(Debug="ignore")]
    inner: ElectrumBlockchain,
}

impl BtcProvider {
    /// Instantiate a new provider with a backend.
    pub fn new(provider: &str) -> Result<Self, ProviderError> {
        let client = Client::new(provider).expect("could not create provider");
        let blockchain = ElectrumBlockchain::from(client);
        Ok(Self {
            inner: blockchain,
        })
    }

    pub fn blockchain(&self) -> &ElectrumBlockchain {
        &self.inner
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BlockchainClient for BtcProvider {
    async fn block_height(&self) -> U64
       {
        self.inner.get_height().unwrap().into()
    }
}

pub fn check_exists() -> bool {
    Path::new("wallet.bin").exists()
}

pub fn save_wallet(wallet: &Wallet) {
    save_file("wallet.bin", 0, wallet).unwrap();
}

pub fn load_wallet() -> Wallet {
    load_file("wallet.bin", 0).unwrap()
}