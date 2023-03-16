use anyhow::anyhow;
use async_trait::async_trait;
pub mod crypto_coin;
pub use crypto_coin::CryptoCoin;
pub mod crypto_wallet;
pub use crypto_wallet::{CryptoWallet, CryptoWalletGeneral};
use std::any::Any;

#[async_trait]
pub trait BlockchainConnector{
    fn new(url: &str) -> Result<Self, anyhow::Error> where Self: Sized;
    fn as_any(&self) -> &dyn Any;

    /// TODO(#84): currently only implemented for Bitcoin, otherwise results in error
    async fn check_if_past_transactions_exist(
        &self,
        _public_address: &str,
    ) -> Result<bool, anyhow::Error> {
        Err(anyhow!("Function not implemented yet"))
    }
}

pub trait CryptoAmount: std::fmt::Display {
    /// In the units of main "big" unit (a floating point number, not an integer
    /// subdivision) using decimal value, for bitcoin BTC, ethereum ETH, etc.
    fn new_from_decimal_value(value: f64) -> Self;
}
