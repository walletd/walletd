use serde::{Deserialize, Serialize};
use crate::types::{Result, Error, AssetId, Balance};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsaToken {
    pub id: AssetId,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub owner: String,
}

impl PsaToken {
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: u128, owner: String) -> Self {
        Self {
            id: AssetId(uuid::Uuid::new_v4().to_string()),
            name,
            symbol,
            decimals,
            total_supply,
            owner,
        }
    }
    
    pub fn format_amount(&self, amount: u128) -> String {
        let divisor = 10_u128.pow(self.decimals as u32);
        let whole = amount / divisor;
        let fraction = amount % divisor;
        format!("{}.{:0width$}", whole, fraction, width = self.decimals as usize)
    }
}
