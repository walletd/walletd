/// SLIP-44 coin type constants.
pub const BITCOIN: u32 = 0;
pub const TESTNET: u32 = 1;
pub const MONERO: u32 = 128; // SLIP-44 coin type for Monero

/// Enum representing SLIP-44 coin types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Bitcoin,
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
        }
    }
}