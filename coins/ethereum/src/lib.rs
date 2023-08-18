//! # WalletD Ethereum Library
//!
//! Provides a wallet implementation for Ethereum and blockchain-specific functionality.
//!
//! ## Quickstart Guide
//!
//! Use the [EthereumWallet] struct as a good starting point to access the functionalities for Ethereum that walletD provides.
//!
//! Each [EthereumWallet] is associated with one public address.
//! An [EthereumWallet] could be instantiated without a [EthereumPrivateKey] affiliated with it, but it would not be able to sign transactions.
//! If an [EthereumWallet] is created with an [EthereumPublicKey] but no [EthereumPrivateKey], it will be able to verify signatures but not sign or send transactions.
//!
//! ### Import from Seed
//!
//! Here's how you can import an Ethereum wallet based on a master [HDKey]. We will use the `mut` keyword to make the [ethereum wallet][EthereumWallet] mutable so that we can modify `ethereum_wallet` later.
//! By importing the `ethereum_wallet` fom a `master_hd_key` will both the [EthereumPrivateKey] and the [EthereumPublicKey] will be provided to the `ethereum_wallet`.
//! ```
//! use walletd_ethereum::prelude::*;
//! use walletd_hd_key::prelude::*;
//!
//! # fn ethereum() -> Result<(), walletd_ethereum::Error> {
//! let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;
//! let mut ethereum_wallet = EthereumWallet::builder().master_hd_key(master_hd_key).build()?;
//! let public_address = ethereum_wallet.public_address();
//! println!("ethereum wallet public address: {}", public_address);
//! # assert!(ethereum_wallet.private_key().is_ok());
//! # assert!(ethereum_wallet.public_key().is_ok());
//! # Ok(())
//! # }
//! ```
//!
//! ### Default Derivation Path
//!
//! Deriving an [EthereumWallet] is simple and uses some default settings under the hood.
//! We can inspect our `ethereum_wallet` to see that a child `derived_hd_key` was derived from the master `master_hd_key` and see the derivation path [HDPath] that was used by default.
//! ```
//! # use walletd_ethereum::prelude::*;
//! # use walletd_hd_key::prelude::*;
//! # fn ethereum() -> Result<(), walletd_ethereum::Error> {
//! # let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;
//! # let mut ethereum_wallet = EthereumWallet::builder().master_hd_key(master_hd_key).build()?;
//! let derived_hd_key = ethereum_wallet.derived_hd_key()?;
//! let address_derivation_path = &derived_hd_key.clone().derivation_path;
//! println!("address derivation path: {}", address_derivation_path);
//! # assert_eq!(address_derivation_path.to_string(), "m/44'/60'/0'/0/0".to_string());
//! # Ok(())
//! # }
//! ```
//! We see that by default the Ethereum wallet uses the derivation path "m/44'/60'/0'/0/" corresponding to  [BIP44 for the purpose value][HDPurpose::BIP44] and 60' corresponding to the coin type for Ethereum.
//!
//! We need to add a [blockchain connector][BlockchainConnector] to our [ethereum wallet][EthereumWallet] to be able to interact with the Ethereum blockchain.
//!
//!
//! ### Adding a Blockchain Connector
//! Here's an example of how to add an instance of [EthClient] to our `ethereum_wallet`.
//! [EthClient] currently supports any Ethereum endpoint that conforms to the Ethereum JSON-RPC standard for accessing Ethereum blockchain data.
//! We recommend using [Infura](https://www.infura.io/), or [Alchemy](https://www.alchemy.com/pricing). Both services provide generous free plans that you can use.
//! Note that the url used to connect needs to match the network type being used (pay attention to the difference between testnet networks (used for testing and development purposes) and the mainnet network (where coins have actual value).
//!
//! ```no_run
//! # use walletd_ethereum::prelude::*;
//! # use walletd_hd_key::prelude::*;
//! # fn ethereum() -> Result<(), walletd_ethereum::Error> {
//! # let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;
//! # let mut ethereum_wallet = EthereumWallet::builder().master_hd_key(master_hd_key).build()?;
//!  let ethclient_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
//!  let eth_client = EthClient::new(ethclient_url)?;
//!  ethereum_wallet.set_blockchain_client(eth_client);
//! # Ok(())
//! # }
//! ```
//!
//! ### Using EthClient to Access Blockchain Data
//! The blockchain client `ethclient` can be used separately from the `ethereum_wallet` to access blockchain data such as details of a transaction given a tx hash, the current block number, or the current gas price.
//```no_run
//# use walletd_ethereum::prelude::*;
//# async fn ethereum() -> Result<(), walletd_ethereum::Error> {
//# let ethclient_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
//# let eth_client = EthClient::new(ethclient_url)?;
//let tx_hash: &str = "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b";
//let tx = eth_client.get_transaction_data_from_tx_hash(tx_hash).await?;
//let block_number = eth_client.current_block_number().await;
//let gas_price = eth_client.gas_price().await;
//println!("transaction data: {:?}", tx);
//# Ok(())
//# }
//```
//!
//! ### Balance of Ethereum Wallet on Blockchain
//! When the `ethereum_wallet` is connected to the blockchain, we can find the balance of the wallet.
//! ```no_run
//! # use walletd_ethereum::prelude::*;
//! # use walletd_hd_key::prelude::*;
//! # async fn ethereum() -> Result<(), walletd_ethereum::Error> {
//! # let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
//! # let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;
//! # let mut ethereum_wallet = EthereumWallet::builder().master_hd_key(master_hd_key).build()?;
//! # let ethclient_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
//! # let eth_client = EthClient::new(ethclient_url)?;
//! # ethereum_wallet.set_blockchain_client(eth_client);
//! let balance = ethereum_wallet.balance().await?;
//! println!("ethereum wallet balance: {} ETH, ({} wei)", balance.eth(), balance.wei());
//! # Ok(())
//! # }
//! ```
//!
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use core::fmt;

mod ethclient;
pub use ethclient::EthClient;
mod ethereum_amount;
pub use ethereum_amount::EthereumAmount;
mod ethereum_wallet;
pub use ethereum_wallet::{
    EthereumPrivateKey, EthereumPublicKey, EthereumWallet, EthereumWalletBuilder,
};
mod error;
pub use error::Error;
pub use ethers;
pub mod prelude;
pub use walletd_coin_core::BlockchainConnector;
pub use walletd_hd_key::{HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose};

/// Represents the format of an Ethereum address (checksummed or non-checksummed)
#[derive(Default, Debug, Clone, Copy)]
pub enum EthereumFormat {
    #[default]
    /// Checksummed is the checksummed format of an Ethereum address where the case of each letter is mixed using the checksum algorithm
    /// This is the default format for this enum
    Checksummed,
    /// NonChecksummed is the non-checksummed format of an Ethereum address where the letters are all lowercase
    NonChecksummed,
}

impl fmt::Display for EthereumFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EthereumFormat::Checksummed => write!(f, "Checksummed"),
            EthereumFormat::NonChecksummed => write!(f, "NonChecksummed"),
        }
    }
}
