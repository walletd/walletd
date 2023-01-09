use std::fmt;

#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub enum SlipCoin {
    // value is the coin type value in accordance with SLIP-0044: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    #[default]
    BTC = 0,
    ETH = 60,
    XMR = 128,
    SOL = 501,
}

impl fmt::Display for SlipCoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SlipCoin::BTC => write!(f, "{}", 0)?,
            SlipCoin::ETH => write!(f, "{}", 60)?,
            SlipCoin::SOL => write!(f, "{}", 501)?,
            SlipCoin::XMR => write!(f, "{}", 128)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slip_coin() {
        assert_eq!(SlipCoin::BTC.to_string(), "0");
        assert_eq!(SlipCoin::ETH.to_string(), "60");
        assert_eq!(SlipCoin::XMR.to_string(), "128");
        assert_eq!(SlipCoin::SOL.to_string(), "501");
    }
}
