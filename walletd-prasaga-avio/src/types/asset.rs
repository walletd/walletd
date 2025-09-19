use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub amount: u128,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsaConfig {
    pub name: String,
    pub symbol: String,
    pub total_supply: u128,
    pub decimals: u8,
    pub methods: Vec<String>,
}
