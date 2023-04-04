use std::any::Any;
use std::fmt;
use walletd_hd_key::HDKey;
use crate::{BlockchainConnector, CryptoAmount};

use async_trait::async_trait;

#[async_trait]
pub trait CryptoWallet: Sized + TryFrom<Box<dyn CryptoWalletGeneral>> {
    // TODO(#61): create custom error type for each coin
    // and add a new associated type here for Error
    type CryptoAmount: CryptoAmount;
    type BlockchainClient: BlockchainConnector;
    type NetworkType;

    fn set_blockchain_client(&mut self, client: Self::BlockchainClient);
    fn blockchain_client(&self) -> Result<&Self::BlockchainClient, anyhow::Error>;

    fn new(master_key: &HDKey, blockchain_client: Option<Box<dyn BlockchainConnector>>) -> Result<Self, anyhow::Error>;

    async fn balance(
        &self,
    ) -> Result<Self::CryptoAmount, anyhow::Error>;

    async fn transfer(
        &self,
        send_amount: &Self::CryptoAmount,
        public_address: &str,
    ) -> Result<String, anyhow::Error>;

    async fn sync(&mut self) -> Result<(), anyhow::Error>;

    fn receive_address(&self) -> Result<String, anyhow::Error>;

}


pub trait CryptoWalletGeneral: fmt::Display {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn CryptoWalletGeneral>;
}
