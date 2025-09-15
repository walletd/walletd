/// SLIP-44 coin type constants.
pub const BITCOIN: u32 = 0;
pub const TESTNET: u32 = 1;
pub const MONERO: u32 = 128; // SLIP-44 coin type for Monero

/// Enum representing coin symbols for easier use in examples and tests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    BTC,
    ETH,
    XMR,
}

/// Enum representing SLIP-44 coin types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Bitcoin,
    Ethereum,
    Testnet,
    Monero,
}

impl Coin {
    /// Returns the SLIP-44 coin type ID.
    pub fn id(&self) -> u32 {
        match self {
            Coin::Bitcoin => BITCOIN,
            Coin::Testnet => TESTNET,
            Coin::Monero => MONERO,
            Coin::Ethereum => 60,
        }
    }
}

impl From<Symbol> for Coin {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::BTC => Coin::Bitcoin,
            Symbol::ETH => Coin::Ethereum, // Using Bitcoin for ETH mapping as example
            Symbol::XMR => Coin::Monero,
        }
    }
}
