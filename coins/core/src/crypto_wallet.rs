use crate::{BlockchainConnector, CryptoAmount};
use async_trait::async_trait;
use walletd_hd_key::{HDKey, HDPathBuilder, Seed};

/// Provides common functionality for a crypto wallet. Contains functions to get the balance, send and receive transactions, and sync the wallet with the blockchain.
#[async_trait]
pub trait CryptoWallet: Sized + Clone {
    /// ErrorType is the type of error that is returned by the CryptoWallet
    type ErrorType: std::error::Error + Send + Sync + 'static;
    /// CryptoAmount is the type of amount that is used by the CryptoWallet to represent amounts of cryptocurrency
    type CryptoAmount: CryptoAmount;
    /// BlockchainClient is the type of BlockchainConnector that is used by the CryptoWallet to connect to the blockchain
    type BlockchainClient: BlockchainConnector;
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

    /// Syncs the [CryptoWallet] with the blockchain
    async fn sync(&mut self) -> Result<(), Self::ErrorType>;

    /// Returns the receive address of the CryptoWallet, this is the address that is used to receive transactions
    fn receive_address(&self) -> Result<String, Self::ErrorType>;

    /// Returns a builder for the CryptoWallet that can be used to build a CryptoWallet with custom options
    fn builder() -> Self::WalletBuilder;
}

/// Provides a common interface for building a [CryptoWallet].
pub trait CryptoWalletBuilder<T>
where
    T: CryptoWallet + Clone,
{
    /// Constructs a new [CryptoWalletBuilder].
    fn new() -> Self;
    /// Builds a [CryptoWallet] from the [CryptoWalletBuilder].
    fn build(&self) -> Result<T, T::ErrorType>;
    /// Specifies the [master HD key][HDKey] for the [CryptoWalletBuilder].
    fn master_hd_key(&mut self, master_hd_key: HDKey) -> &mut Self;
    /// Specifies the [mnemonic seed][Seed] for the [CryptoWalletBuilder].
    fn mnemonic_seed(&mut self, mnemonic_seed: Seed) -> &mut Self;
    /// Specifies the [address format][CryptoWallet::AddressFormat] for the [CryptoWalletBuilder].
    fn address_format(&mut self, address_format: T::AddressFormat) -> &mut Self;
    /// Specifies the[] HD path builder][HDPathBuilder] for the [CryptoWalletBuilder].
    fn hd_path_builder(&mut self, hd_path_builder: HDPathBuilder) -> &mut Self;
    /// Specifies the [network type][CryptoWallet::NetworkType] for the [CryptoWalletBuilder].
    fn network_type(&mut self, network_type: T::NetworkType) -> &mut Self;
}
