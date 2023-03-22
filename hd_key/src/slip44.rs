use std::fmt;
use std::str::FromStr;

use anyhow::anyhow;

/// SLIP-0044: Registered coin types for BIP-0044
#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub enum SlipCoin {
    // value is the coin type value in accordance with SLIP-0044: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    #[default]
    BTC = 0,
    ETH = 60,
    XMR = 128,
    SOL = 501,
    AnyTestnet = 1,
}

impl fmt::Display for SlipCoin {
    /// Converts a SlipCoin to a number string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SlipCoin::BTC => write!(f, "{}'", 0)?,
            SlipCoin::ETH => write!(f, "{}'", 60)?,
            SlipCoin::SOL => write!(f, "{}'", 501)?,
            SlipCoin::XMR => write!(f, "{}'", 128)?,
            SlipCoin::AnyTestnet => write!(f, "{}'", 1)?,
        }
        Ok(())
    }
}

impl SlipCoin {
    // Creates a new SlipCoin based on the coin type value in accordance with
    // SLIP-0044, assumes mainnet, throws error to be handled if testnet or
    // unsupported type
    pub fn new(value: usize) -> Result<Self, anyhow::Error> {
        match value {
            0 => Ok(Self::BTC),
            60 => Ok(Self::ETH),
            128 => Ok(Self::XMR),
            501 => Ok(Self::SOL),
            1 => Ok(Self::AnyTestnet),
            _ => Err(anyhow!(
                "Currently not supporting a SlipCoin with a coin type value of {}",
                value
            )),
        }
    }
}

impl FromStr for SlipCoin {
    type Err = anyhow::Error;

    /// Matches coin name ignoring case and allowing either the long form or
    /// short abbrevation form
    fn from_str(coin_name: &str) -> Result<Self, anyhow::Error> {
        match coin_name.to_string().to_lowercase().as_str() {
            "btc" | "bitcoin" => Ok(Self::BTC),
            "eth" | "ethereum" | "ether" => Ok(Self::ETH),
            "sol" | "solana" => Ok(Self::SOL),
            "xmr" | "monero" => Ok(Self::XMR),
            _ => Err(anyhow!("Current valid options are BTC, ETH, SOL, or XMR")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(matches!(SlipCoin::new(0), Ok(SlipCoin::BTC)));
        assert!(matches!(SlipCoin::new(60), Ok(SlipCoin::ETH)));
        assert!(matches!(SlipCoin::new(128), Ok(SlipCoin::XMR)));
        assert!(matches!(SlipCoin::new(501), Ok(SlipCoin::SOL)));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(SlipCoin::from_str("btc").unwrap(), SlipCoin::BTC);
        assert_eq!(SlipCoin::from_str("BTC").unwrap(), SlipCoin::BTC);
        assert_eq!(SlipCoin::from_str("ETH").unwrap(), SlipCoin::ETH);
        assert_eq!(SlipCoin::from_str("XMR").unwrap(), SlipCoin::XMR);
        assert_eq!(SlipCoin::from_str("SOL").unwrap(), SlipCoin::SOL);
    }

    #[test]
    fn test_slip_coin() {
        assert_eq!(SlipCoin::BTC.to_string(), "0'");
        assert_eq!(SlipCoin::ETH.to_string(), "60'");
        assert_eq!(SlipCoin::XMR.to_string(), "128'");
        assert_eq!(SlipCoin::SOL.to_string(), "501'");
    }
}
