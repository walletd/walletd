use core::fmt;
use core::fmt::Display;
use std::str::FromStr;

use anyhow::anyhow;
use walletd_hd_key::slip44::Symbol;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum CryptoCoin {
    BTC,
    ETH,
    // XMR = 128,
    // SOL = 501,
}

impl CryptoCoin {
    pub fn main_unit(&self) -> String {
        match self {
            Self::BTC => "BTC".to_string(),
            Self::ETH => "ETH".to_string(),
            // Self::SOL => "SOL".to_string(),
            // Self::XMR => "XMR".to_string(),
        }
    }

    pub fn fundamental_unit(&self) -> String {
        match self {
            Self::BTC => "satoshi".to_string(),
            Self::ETH => "wei".to_string(),
            // Self::SOL => "lamport".to_string(),
            // Self::XMR => "piconero".to_string(),
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
            //"sol" | "solana" => Ok(Self::SOL),
            //"xmr" | "monero" => Ok(Self::XMR),
            _ => Err(anyhow!("Current valid options are BTC, ETH, SOL, or XMR")),
        }
    }
}

impl Display for CryptoCoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BTC => write!(f, "Bitcoin (BTC)")?,
            Self::ETH => write!(f, "Ethereum (ETH)")?,
            // Self::SOL => write!(f, "Solana (SOL)")?,
            // Self::XMR => write!(f, "Monero (XMR)")?,
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

impl TryInto<Symbol> for CryptoCoin {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Symbol, Self::Error> {
        match self {
            CryptoCoin::BTC => Ok(Symbol::BTC),
            CryptoCoin::ETH => Ok(Symbol::ETH),
            // CryptoCoin::XMR => Ok(Symbol::XMR),
            // CryptoCoin::SOL => Ok(Symbol::SOL),
        }
    }
}
