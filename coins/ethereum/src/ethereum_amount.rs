use crate::Error;
use std::ops;
use walletd_coin_core::CryptoAmount;
use web3::ethabi::ethereum_types::U256;

/// Contains a field representing the amount of wei in the amount. Also has functions to convert to and from the main unit (ETH) and the smallest unit (wei).
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct EthereumAmount {
    /// The number of wei (U256) in the amount
    pub wei: U256,
}

impl ops::Add<Self> for EthereumAmount {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Result<Self, Error> {
        Ok(Self {
            wei: self
                .wei
                .checked_add(rhs.wei)
                .ok_or(Error::Overflow(format!(
                    "Overflow in U256 when adding {} to {}",
                    self.wei, rhs.wei
                )))?,
        })
    }
}

impl ops::Sub for EthereumAmount {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Result<Self, Error> {
        Ok(Self {
            wei: self
                .wei
                .checked_sub(rhs.wei)
                .ok_or(Error::Overflow(format!(
                    "Overflow in U256 when subtracting {} from {}",
                    self.wei, rhs.wei
                )))?,
        })
    }
}

impl ops::Mul<u64> for EthereumAmount {
    type Output = Result<Self, Error>;

    fn mul(self, rhs: u64) -> Self::Output {
        let result = self.wei * rhs;

        if result > U256::MAX {
            return Err(Error::Overflow(format!(
                "Overflow in U256 when multiplying {} by {}",
                self.wei, rhs
            )));
        }
        Ok(Self { wei: result })
    }
}
impl ops::Mul for EthereumAmount {
    type Output = Result<Self, Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        Ok(Self {
            wei: self
                .wei
                .checked_mul(rhs.wei)
                .ok_or(Error::Overflow(format!(
                    "Overflow in U256 when multiplying {} by {}",
                    self.wei, rhs.wei
                )))?,
        })
    }
}

impl ops::Div for EthereumAmount {
    type Output = Result<Self, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        Ok(Self {
            wei: self
                .wei
                .checked_div(rhs.wei)
                .ok_or(Error::Overflow(format!(
                    "Overflow in U256 when dividing {} by {}",
                    self.wei, rhs.wei
                )))?,
        })
    }
}

impl EthereumAmount {
    /// Creates a new EthereumAmount from a decimal value in ETH
    pub fn from_eth(eth_amount: f64) -> Self {
        let wei = (eth_amount * f64::powf(10.0, 18.0)) as usize; // 10^18 wei per ethereum
        Self { wei: wei.into() }
    }

    /// Creates a new EthereumAmount from a decimal value in ETH
    pub fn eth(&self) -> f64 {
        self.wei.as_u64() as f64 / f64::powf(10.0, 18.0) // 10^18 wei per
                                                         // ethereum
    }

    /// Returns the number of wei in the amount
    pub fn wei(&self) -> U256 {
        self.wei
    }

    /// Creates a new EthereumAmount from the wei amount
    pub fn from_wei(wei_amount: U256) -> Self {
        Self { wei: wei_amount }
    }
}

impl CryptoAmount for EthereumAmount {
    fn from_main_unit_decimal_value(value: f64) -> Self {
        Self::from_eth(value)
    }

    fn from_smallest_unit_integer_value(value: u64) -> Self {
        Self::from_wei(value.into())
    }

    fn to_main_unit_decimal_value(&self) -> f64 {
        self.eth()
    }

    fn to_smallest_unit_integer_value(&self) -> u64 {
        self.wei.as_u64()
    }
}
