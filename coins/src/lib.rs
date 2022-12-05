
#[derive(Default, PartialEq, Copy, Clone)]
pub enum CryptoCoin {
// value is the coin type value in accordance with SLIP-0044: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    #[default]
    BTC = 0,
    ETH = 60,
    XMR = 128,
    SOL = 501,
}

impl CryptoCoin {
    pub fn coin_type(&self) -> usize {
        *self as usize
    }
    pub fn from_str(coin_name: &str) -> Result<CryptoCoin, String> {
        match coin_name {
            "btc" | "bitcoin" => Ok(CryptoCoin::BTC),
            "eth" | "ethereum" | "ether" => Ok(CryptoCoin::ETH),
            "sol" | "solana" => Ok(CryptoCoin::SOL),
            "xmr" | "monero" => Ok(CryptoCoin::XMR),
            _ => Err("Current valid options are BTC, ETH, SOL, or XMR".to_string()),
        }
    }
}

pub trait CryptoTypeData {
    fn print_public_address(&self) -> ();
}

pub trait BlockChainConnector {
    type BlockchainClient;

    fn setup_connection() -> Result<Self::BlockchainClient, String>;
}
