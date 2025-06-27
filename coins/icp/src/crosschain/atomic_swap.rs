use super::protocols::ChainType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomicSwap {
    pub initiator: String,
    pub target_chain: ChainType,
    pub amount: u64,
    pub state: SwapState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SwapState {
    Initiated,
    Locked,
    Redeemed,
    Refunded,
}

impl AtomicSwap {
    pub fn new(initiator: String, target_chain: ChainType, amount: u64) -> Self {
        Self {
            initiator,
            target_chain,
            amount,
            state: SwapState::Initiated,
        }
    }
}
