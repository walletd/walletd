pub struct CrossChainMessage {
    pub content: String,
    pub from: ChainType,
    pub to: ChainType,
}

impl CrossChainMessage {
    pub fn new(from: ChainType, to: ChainType, content: String) -> Self {
        Self { content, from, to }
    }
}

pub enum MessageStatus {
    Pending,
    Sent,
    Confirmed,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ChainType {
    ICP,
    ETH,
    BTC,
    SOL,
    Bitcoin,
    Ethereum,
    Solana,
}

impl std::fmt::Display for ChainType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainType::ICP => write!(f, "ICP"),
            ChainType::Bitcoin | ChainType::BTC => write!(f, "Bitcoin"),
            ChainType::Ethereum | ChainType::ETH => write!(f, "Ethereum"),
            ChainType::Solana | ChainType::SOL => write!(f, "Solana"),
        }
    }
}
