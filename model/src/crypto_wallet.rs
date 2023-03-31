use std::any::Any;
use std::fmt;

use async_trait::async_trait;

#[async_trait]
pub trait CryptoWallet: Sized {
    // TODO(#61): create custom error type for each coin
    // and add a new associated type here for Error
    type MnemonicSeed;
    type AddressFormat;
    type CryptoAmount;
    type BlockchainClient;
    type NetworkType;

    fn set_blockchain_client(&mut self, client: Self::BlockchainClient);

    // TODO(AS): Refactor this to use a builder pattern, or some other way that will work for all coins
    // fn from_mnemonic(mnemonic_seed: &Self::MnemonicSeed, network_type: HDNetworkType, address_format: Self::AddressFormat) -> Result<Self, anyhow::Error>;
    
    // fn from_hd_key(
    //     hd_key: &HDKey,
    //     address_format: Self::AddressFormat,
    // ) -> Result<Self, anyhow::Error>;

    async fn balance(
        &self,
    ) -> Result<Self::CryptoAmount, anyhow::Error>;

    async fn transfer(
        &self,
        send_amount: &Self::CryptoAmount,
        public_address: &str,
    ) -> Result<String, anyhow::Error>;

}


pub trait CryptoWalletGeneral: fmt::Display {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn CryptoWalletGeneral>;
}
