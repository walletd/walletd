use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainType {
   ICP,
   ETH,
   BTC,
   SOL,
}

pub struct Protocol;
