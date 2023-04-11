use core::fmt;
use core::fmt::Display;

use anyhow::anyhow;

#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub enum CryptoCoin {
    // value is the coin type value in accordance with SLIP-0044: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    #[default]
    BTC = 0,
    ETH = 60,
    XMR = 128,
    SOL = 501,
}

impl CryptoCoin {
    // Creates a new CryptoCoin based on the coin type value in accordance with
    // SLIP-0044, assumes mainnet, throws error to be handled if testnet or
    // unsupported type
    pub fn new(value: usize) -> Result<Self, anyhow::Error> {
        match value {
            0 => Ok(Self::BTC),
            60 => Ok(Self::ETH),
            128 => Ok(Self::XMR),
            501 => Ok(Self::SOL),
            1 => Err(anyhow!("This value is for any testnet")),
            _ => Err(anyhow!(
                "Currently not supporting a CryptoCoin with a coin type value of {}",
                value
            )),
        }
    }

    pub fn coin_type_mainnet(&self) -> usize {
        *self as usize
    }

    /// Matches coin name ignoring case and allowing either the long form or
    /// short abbrevation form
    pub fn from_str(coin_name: &str) -> Result<Self, anyhow::Error> {
        match coin_name.to_string().to_lowercase().as_str() {
            "btc" | "bitcoin" => Ok(Self::BTC),
            "eth" | "ethereum" | "ether" => Ok(Self::ETH),
            "sol" | "solana" => Ok(Self::SOL),
            "xmr" | "monero" => Ok(Self::XMR),
            _ => Err(anyhow!("Current valid options are BTC, ETH, SOL, or XMR")),
        }
    }
}

impl Display for CryptoCoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BTC => writeln!(f, "Bitcoin (BTC)")?,
            Self::ETH => writeln!(f, "Ethereum (ETH)")?,
            Self::SOL => writeln!(f, "Solana (SOL)")?,
            Self::XMR => writeln!(f, "Monero (XMR)")?,
        }
        Ok(())
    }
}
