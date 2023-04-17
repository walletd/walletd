use crate::{BlockchainConnector, BlockchainConnectorGeneral, CryptoAmount};
use std::any::Any;
use std::fmt;
use walletd_hd_key::{HDKey, HDPathBuilder, Seed};

use async_trait::async_trait;

/// CryptoWallet is a trait that provides common functionality for a crypto wallet. It provides functions to get the balance, send and receive transactions, and sync the wallet with the blockchain.
#[async_trait]
pub trait CryptoWallet:
    Sized + TryFrom<Box<dyn CryptoWalletGeneral>> + CryptoWalletGeneral + Clone
{
    /// ErrorType is the type of error that is returned by the CryptoWallet
    type ErrorType: std::error::Error + fmt::Display + Send + Sync + 'static;
    /// CryptoAmount is the type of amount that is used by the CryptoWallet to represent amounts of cryptocurrency
    type CryptoAmount: CryptoAmount;
    /// BlockchainClient is the type of BlockchainConnector that is used by the CryptoWallet to connect to the blockchain
    type BlockchainClient: BlockchainConnector + BlockchainConnectorGeneral;
    /// NetworkType is the type of network that the CryptoWallet is connected to
    type NetworkType;
    /// WalletBuilder is the type of builder that is used to build a CryptoWallet
    type WalletBuilder: CryptoWalletBuilder<Self>;
    /// AddressFormat is the type of address format that is used by the CryptoWallet
    type AddressFormat;

    /// Associates a particular blockchain client with the CryptoWallet
    fn set_blockchain_client(&mut self, client: Self::BlockchainClient);
    /// Returns the blockchain client that is associated with the CryptoWallet if it exists, otherwise returns an error
    fn blockchain_client(&self) -> Result<&Self::BlockchainClient, Self::ErrorType>;

    /// Returns the balance of the CryptoWallet as a CryptoAmount
    async fn balance(&self) -> Result<Self::CryptoAmount, Self::ErrorType>;

    /// Sends a transaction from the CryptoWallet to a given public address with a given amount
    async fn transfer(
        &self,
        send_amount: &Self::CryptoAmount,
        public_address: &str,
    ) -> Result<String, Self::ErrorType>;

    /// Syncs the CryptoWallet with the blockchain
    async fn sync(&mut self) -> Result<(), Self::ErrorType>;

    /// Returns the receive address of the CryptoWallet, this is the address that is used to receive transactions
    fn receive_address(&self) -> Result<String, Self::ErrorType>;

    /// Returns a builder for the CryptoWallet that can be used to build a CryptoWallet with custom options
    fn builder() -> Self::WalletBuilder;
}

/// CryptoWalletGeneral is a general trait that can work with any struct that implements the CryptoWallet trait
pub trait CryptoWalletGeneral: fmt::Display {
    /// Returns a dyn Any reference to the CrypotowalletGeneral
    fn as_any(&self) -> &dyn Any;

    /// Returns a clone in a box type
    fn box_clone(&self) -> Box<dyn CryptoWalletGeneral>;
}

/// CryptoWalletBuilder is a trait that provides a common interface for building a CryptoWallet
pub trait CryptoWalletBuilder<T>
where
    T: CryptoWallet + CryptoWalletGeneral + Clone,
{
    /// Constructs a new CryptoWalletBuilder
    fn new() -> Self;
    /// Builds a CryptoWallet from the CryptoWalletBuilder
    fn build(&self) -> Result<T, T::ErrorType>;
    /// Specifies the master HD key for the CryptoWalletBuilder
    fn with_master_hd_key(&mut self, master_hd_key: HDKey) -> &mut Self;
    /// Specifies the mnemonic seed for the CryptoWalletBuilder
    fn with_mnemonic_seed(&mut self, mnemonic_seed: Seed) -> &mut Self;
    /// Specifies the blockchain client for the CryptoWalletBuilder
    fn with_blockchain_client(&mut self, client: Box<dyn BlockchainConnectorGeneral>) -> &mut Self;
    /// Specifies the address format for the CryptoWalletBuilder
    fn with_address_format(&mut self, address_format: T::AddressFormat) -> &mut Self;
    /// Specifies the HD path builder for the CryptoWalletBuilder
    fn with_hd_path_builder(&mut self, hd_path_builder: HDPathBuilder) -> &mut Self;
    /// Specifies the network type for the CryptoWalletBuilder
    fn with_network_type(&mut self, network_type: T::NetworkType) -> &mut Self;
}
