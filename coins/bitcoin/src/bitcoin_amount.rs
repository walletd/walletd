use core::fmt;
use core::fmt::Display;
use std::ops;
use crate::Error;

use walletd_coin_model::CryptoAmount;

/// BitcoinAmount contains a field representing the amount of satoshis in the amount. It also has functions to convert to and from the main unit (BTC) and the smallest unit (satoshi).
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BitcoinAmount {
    /// The number of satoshis (u64) in the amount
    pub satoshi: u64,
}

impl ops::Add<Self> for BitcoinAmount {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Result<Self, Error>{
        Ok(Self {
            satoshi: self.satoshi.checked_add(rhs.satoshi).ok_or(Error::Overflow(format!("Overflow in u64 when adding {} to {}", self.satoshi, rhs.satoshi)))?,
        })
    }
}


impl ops::Sub for BitcoinAmount {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Result<Self, Error> {
        Ok(Self {
            satoshi: self.satoshi.checked_sub(rhs.satoshi).ok_or(Error::Overflow(format!("Overflow in u64 when subtracting {} from {}", self.satoshi, rhs.satoshi)))?,
        })
    }
}

impl ops::Mul for BitcoinAmount {
    type Output = Result<Self, Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        Ok(Self {
            satoshi: self.satoshi.checked_mul(rhs.satoshi).ok_or(Error::Overflow(format!("Overflow in u64 when multiplying {} by {}", self.satoshi, rhs.satoshi)))?,
        })
    }
}

impl ops::Mul<f64> for BitcoinAmount {
    type Output = Result<Self, Error>;

    fn mul(self, rhs: f64) -> Self::Output {
        let result = self.satoshi as f64 * rhs;
        if result > f64::MAX  || result < f64::MIN {
            return Err(Error::Overflow(format!("Overflow in f64 when multiplying {} by {}", self.satoshi, rhs)));
        }

        let as_u64 = result as u64;
        
        if as_u64 > u64::MAX || as_u64 < u64::MIN {
            return Err(Error::Overflow(format!("Overflow in u64 when multiplying {} by {}", self.satoshi, rhs)));
        }

        Ok(Self {
            satoshi: as_u64
        })
    }
}

impl ops::Div for BitcoinAmount {
    type Output = Result<Self, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        Ok(Self {
            satoshi: self.satoshi.checked_div(rhs.satoshi).ok_or(Error::Overflow(format!("Overflow in u64 when dividing {} by {}", self.satoshi, rhs.satoshi)))?,
        })
    }
}

impl BitcoinAmount {
    /// Returns a BitcoinAmount struct from a decimal value representing the amount in BTC
    pub fn from_btc(btc_amount: f64) -> Self {
        let satoshi = (btc_amount * 100_000_000.0) as u64; // 100 million satoshis per bitcoin
        Self { satoshi }
    }

    /// Returns a BitcoinAmount struct from an integer value representing the amount in satoshis
    pub fn from_satoshi(satoshi_amount: u64) -> Self {
        Self {
            satoshi: satoshi_amount,
        }
    }

    /// Returns the amount in BTC as a f64
    pub fn btc(&self) -> f64 {
        self.satoshi as f64 / 100_000_000.0 // 100 million satoshis per bitcoin
    }

    /// Returns the amount in satoshis as a u64
    pub fn satoshi(&self) -> u64 {
        self.satoshi
    }
}

impl CryptoAmount for BitcoinAmount {
    fn from_main_unit_decimal_value(value: f64) -> Self {
        Self::from_btc(value)
    }

    fn from_smallest_unit_integer_value(value: u64) -> Self {
        Self::from_satoshi(value)
    }

    fn to_main_unit_decimal_value(&self) -> f64 {
        self.btc()
    }

    fn to_smallest_unit_integer_value(&self) -> u64 {
        self.satoshi()
    }
}

impl Display for BitcoinAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Bitcoin Amount: {} BTC, {} satoshi",
            self.btc(),
            self.satoshi()
        )?;
        Ok(())
    }
}
