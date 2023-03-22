use core::fmt;
use core::fmt::Display;
use std::ops;

use walletd_coin_model::CryptoAmount;
use web3::ethabi::ethereum_types::U256;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct EthereumAmount {
    pub wei: U256,
}

impl ops::Add<Self> for EthereumAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            wei: self.wei + rhs.wei,
        }
    }
}

impl ops::AddAssign for EthereumAmount {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            wei: self.wei + other.wei,
        }
    }
}

impl ops::Sub for EthereumAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            wei: self.wei - rhs.wei,
        }
    }
}
impl ops::Mul<u64> for EthereumAmount {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self {
            wei: self.wei * rhs,
        }
    }
}
impl ops::Mul for EthereumAmount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            wei: self.wei * rhs.wei,
        }
    }
}

impl ops::Div for EthereumAmount {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            wei: self.wei / rhs.wei,
        }
    }
}

impl EthereumAmount {
    pub fn new_from_eth(eth_amount: f64) -> Self {
        let wei = (eth_amount * f64::powf(10.0, 18.0)) as usize; // 10^18 wei per ethereum
        Self { wei: wei.into() }
    }

    pub fn eth(&self) -> f64 {
        self.wei.as_u64() as f64 / f64::powf(10.0, 18.0) // 10^18 wei per
                                                         // ethereum
    }

    pub fn wei(&self) -> U256 {
        self.wei
    }

    pub fn new_from_wei(wei_amount: U256) -> Self {
        Self { wei: wei_amount }
    }
}

impl Display for EthereumAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} ETH ({} wei)", self.eth(), self.wei())?;
        Ok(())
    }
}

impl CryptoAmount for EthereumAmount {
    
    fn new_from_main_unit_decimal_value(value: f64) -> Self {
        Self::new_from_eth(value)
    }

    fn to_main_unit_decimal_value(&self) -> f64 {
        self.eth()
    }

    fn to_smallest_unit_integer_value(&self) -> u64 {
        self.wei.as_u64()
    }
}