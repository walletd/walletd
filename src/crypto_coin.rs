use core::fmt;
use core::fmt::Display;
use std::str::FromStr;

use anyhow::anyhow;
use walletd_hd_key::slip44::Symbol;

/// An enum representing the different crypto coins that are supported by WalletD
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum CryptoCoin {
    /// Bitcoin
    BTC,
    /// Ethereum
    ETH,
}

impl CryptoCoin {
    /// Returns the coin's symbol as string which also represents the main unit of the coin's value
    pub fn main_unit(&self) -> String {
        match self {
            Self::BTC => "BTC".to_string(),
            Self::ETH => "ETH".to_string(),
        }
    }

    /// Returns the fundamental unit of the coin's value as string, this is the name of the smallest indivisible unit of the coin's value
    pub fn fundamental_unit(&self) -> String {
        match self {
            Self::BTC => "satoshi".to_string(),
            Self::ETH => "wei".to_string(),
        }
    }
}

impl FromStr for CryptoCoin {
    type Err = anyhow::Error;

    /// Matches coin name ignoring case and allowing either the long form or
    /// short abbrevation form
    fn from_str(coin_name: &str) -> Result<Self, Self::Err> {
        match coin_name.to_string().to_lowercase().as_str() {
            "btc" | "bitcoin" => Ok(Self::BTC),
            "eth" | "ethereum" | "ether" => Ok(Self::ETH),
            _ => Err(anyhow!("Current valid options are BTC or ETH")),
        }
    }
}

impl Display for CryptoCoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BTC => write!(f, "Bitcoin (BTC)")?,
            Self::ETH => write!(f, "Ethereum (ETH)")?,
        }
        Ok(())
    }
}

impl TryFrom<Symbol> for CryptoCoin {
    type Error = anyhow::Error;

    fn try_from(value: Symbol) -> Result<Self, Self::Error> {
        match value {
            Symbol::BTC => Ok(CryptoCoin::BTC),
            Symbol::ETH => Ok(CryptoCoin::ETH),
            // Symbol::XMR => Ok(CryptoCoin::XMR),
            // Symbol::SOL => Ok(CryptoCoin::SOL),
            _ => Err(anyhow!("Unsupported coin type")),
        }
    }
}
