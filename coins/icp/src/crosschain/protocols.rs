use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainType {
    ICP,
    ETH,
    BTC,
    SOL,
}

pub struct Protocol;
