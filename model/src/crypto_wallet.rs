use std::any::Any;
use std::fmt;
use walletd_hd_key::HDKey;
use crate::{BlockchainConnector, BlockchainConnectorGeneral, CryptoAmount};

use async_trait::async_trait;

#[async_trait]
pub trait CryptoWallet: Sized + TryFrom<Box<dyn CryptoWalletGeneral>> {
    type ErrorType: std::error::Error + fmt::Display + Send + Sync + 'static;
    type CryptoAmount: CryptoAmount;
    type BlockchainClient: BlockchainConnector + BlockchainConnectorGeneral;
    type NetworkType;

    fn set_blockchain_client(&mut self, client: Self::BlockchainClient);
    fn blockchain_client(&self) -> Result<&Self::BlockchainClient, Self::ErrorType>;

    fn new(master_key: &HDKey, blockchain_client: Option<Box<dyn BlockchainConnectorGeneral>>) -> Result<Self, Self::Error>;

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

}


pub trait CryptoWalletGeneral: fmt::Display {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn CryptoWalletGeneral>;
}
