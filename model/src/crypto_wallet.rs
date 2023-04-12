use std::any::Any;
use std::fmt;
use walletd_hd_key::{HDPathBuilder, HDKey, Seed};
use crate::{BlockchainConnector, BlockchainConnectorGeneral, CryptoAmount};

use async_trait::async_trait;

#[async_trait]
pub trait CryptoWallet: Sized + TryFrom<Box<dyn CryptoWalletGeneral>> + CryptoWalletGeneral + Clone {
    type ErrorType: std::error::Error + fmt::Display + Send + Sync + 'static;
    type CryptoAmount: CryptoAmount;
    type BlockchainClient: BlockchainConnector + BlockchainConnectorGeneral;
    type NetworkType;
    type WalletBuilder: CryptoWalletBuilder<Self>;
    type AddressFormat;

    fn set_blockchain_client(&mut self, client: Self::BlockchainClient);
    fn blockchain_client(&self) -> Result<&Self::BlockchainClient, Self::ErrorType>;

    async fn balance(
        &self,
    ) -> Result<Self::CryptoAmount, Self::ErrorType>;

    async fn transfer(
        &self,
        send_amount: &Self::CryptoAmount,
        public_address: &str,
    ) -> Result<String, Self::ErrorType>;

    async fn sync(&mut self) -> Result<(), Self::ErrorType>;

    fn receive_address(&self) -> Result<String, Self::ErrorType>;

    fn builder() -> Self::WalletBuilder;
    
}


pub trait CryptoWalletGeneral: fmt::Display {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn CryptoWalletGeneral>;
}

pub trait CryptoWalletBuilder<T> where T: CryptoWallet + CryptoWalletGeneral + Clone {
    
    fn new() -> Self;
    fn build(&self) -> Result<T, T::ErrorType>;
    fn with_master_hd_key(&mut self, master_hd_key: HDKey) -> &mut Self;
    fn with_mnemonic_seed(&mut self, mnemonic_seed: Seed) -> &mut Self;
    fn with_blockchain_client(&mut self, client: Box<dyn BlockchainConnectorGeneral>) -> &mut Self;
    fn with_address_format(&mut self, address_format: T::AddressFormat) -> &mut Self;
    fn with_hd_path_builder(&mut self, hd_path_builder: HDPathBuilder) -> &mut Self;
    fn with_network_type(&mut self, network_type: T::NetworkType) -> &mut Self;
}

