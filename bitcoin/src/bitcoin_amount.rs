use core::fmt;
use core::fmt::Display;
use std::ops;

use walletd_coin_model::CryptoAmount;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BitcoinAmount {
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
    pub fn new_from_btc(btc_amount: f64) -> Self {
        let satoshi = (btc_amount * 100_000_000.0) as u64; // 100 million satoshis per bitcoin
        Self { satoshi }
    }

    pub fn btc(&self) -> f64 {
        self.satoshi as f64 / 100_000_000.0 // 100 million satoshis per bitcoin
    }

    pub fn satoshi(&self) -> u64 {
        self.satoshi
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

impl CryptoAmount for BitcoinAmount {
    fn new_from_decimal_value(value: f64) -> Self {
        Self::new_from_btc(value)
    }
}
