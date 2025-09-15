use ethers::types::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseTransaction {
    pub chain_id: u64,
    pub nonce: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub gas_limit: U256,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Bytes,
}

impl BaseTransaction {
    pub fn new(chain_id: u64) -> Self {
        Self {
            chain_id,
            nonce: U256::zero(),
            max_fee_per_gas: U256::zero(),
            max_priority_fee_per_gas: U256::zero(),
            gas_limit: U256::from(21000),
            to: None,
            value: U256::zero(),
            data: Bytes::default(),
        }
    }
}
