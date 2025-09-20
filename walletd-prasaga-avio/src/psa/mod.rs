//! Programmable Smart Assets (PSA) implementation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgrammableSmartAsset {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub total_supply: u128,
    pub decimals: u8,
}

impl ProgrammableSmartAsset {
    pub fn new(name: String, symbol: String, total_supply: u128, decimals: u8) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            symbol,
            total_supply,
            decimals,
        }
    }
}
