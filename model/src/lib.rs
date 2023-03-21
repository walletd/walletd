use anyhow::anyhow;
use async_trait::async_trait;
pub mod crypto_wallet;
use std::any::Any;

pub use crypto_wallet::{CryptoWallet, CryptoAddressGeneral};

#[async_trait]
pub trait BlockchainConnector {
    fn new(url: &str) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;

    /// TODO(#84): currently only implemented for Bitcoin, otherwise results in
    /// error
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
    /// The CryptoAmount is a wrapper around a floating point number and the default integer value stored is in the smallest unit of the coin.
    /// For example this would be satoshi for bitcoin, wei for ethereum, etc.
    /// The decimal value is the floating point number representation in the main unit of the coin, for example BTC, ETH, etc.
    fn new_from_main_unit_decimal_value(value: f64) -> Self;

    /// Returns the decimal value of the CryptoAmount, this is the floating point number representation in the main unit of the coin, for example BTC, ETH, etc.
    fn to_main_unit_decimal_value(&self) -> f64;

    /// Returns the integer value of the CryptoAmount, this is the integer representation in the smallest unit of the coin, for example satoshi for bitcoin, wei for ethereum, etc.
    fn to_smallest_unit_integer_value(&self) -> u64;

    /// Constructs a new CryptoAmount with a value of 0.0
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new_from_main_unit_decimal_value(0.0)
    }
}
