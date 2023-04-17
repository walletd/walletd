use core::fmt;
use core::fmt::Display;
use std::ops;

use walletd_coin_model::CryptoAmount;

/// BitcoinAmount contains a field representing the amount of satoshis in the amount. It also has functions to convert to and from the main unit (BTC) and the smallest unit (satoshi). 
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BitcoinAmount {
    /// The number of satoshis (u64) in the amount
    pub satoshi: u64,
}

impl ops::Add<Self> for BitcoinAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            satoshi: self.satoshi + rhs.satoshi,
        }
    }
}

impl ops::AddAssign for BitcoinAmount {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            satoshi: self.satoshi + other.satoshi,
        }
    }
}

impl ops::Sub for BitcoinAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            satoshi: self.satoshi - rhs.satoshi,
        }
    }
}

impl ops::Mul for BitcoinAmount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            satoshi: self.satoshi * rhs.satoshi,
        }
    }
}

impl ops::Mul<f64> for BitcoinAmount {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            satoshi: ((self.satoshi as f64) * rhs) as u64,
        }
    }
}

impl ops::Div for BitcoinAmount {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            satoshi: self.satoshi / rhs.satoshi,
        }
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
