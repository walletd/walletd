use std::fmt::Display;

use anyhow::anyhow;
use async_trait::async_trait;
mod crypto_coin;
pub use crypto_coin::CryptoCoin;
mod crypto_wallet;
pub use crypto_wallet::{CryptoWallet, CryptoWalletGeneral};

#[async_trait]
pub trait BlockchainConnector: Sized {
    fn new(url: &str) -> Result<Self, anyhow::Error>;

    /// TODO: currently only implemented for Bitcoin, otherwise results in error
    async fn check_if_past_transactions_exist(
        &self,
        public_address: &str,
    ) -> Result<bool, anyhow::Error> {
        Err(anyhow!("Function not implemented yet"))
    }
}

pub trait CryptoAmount: std::fmt::Display {
    /// In the units of main "big" unit (a floating point number, not an integer subdivision) using decimal value, for bitcoin BTC, ethereum ETH, etc.
    fn new_from_decimal_value(value: f64) -> Self;
}
