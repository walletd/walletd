use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod simple_swap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRequest {
    pub from_chain: Chain,
    pub to_chain: Chain,
    pub from_asset: String,
    pub to_asset: String,
    pub amount: String,
    pub recipient_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Chain {
    Bitcoin,
    Ethereum,
    Solana,
    Monero,
    ICP,
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chain::Bitcoin => write!(f, "BTC"),
            Chain::Ethereum => write!(f, "ETH"),
            Chain::Solana => write!(f, "SOL"),
            Chain::Monero => write!(f, "XMR"),
            Chain::ICP => write!(f, "ICP"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapQuote {
    pub input_amount: String,
    pub output_amount: String,
    pub exchange_rate: f64,
    pub fee: String,
    pub estimated_time: u64,
    pub route: Vec<SwapStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapStep {
    pub protocol: String,
    pub action: String,
    pub chain: Chain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapResult {
    pub swap_id: String,
    pub from_tx_hash: String,
    pub status: SwapStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwapStatus {
    Pending,
    WaitingForDeposit,
    Confirming,
    Swapping,
    Completed { to_tx_hash: String },
    Failed { reason: String },
}
