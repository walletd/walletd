// Simplified UTXO manager
use bitcoin::{OutPoint, TxOut};
use std::collections::HashMap;

pub struct UtxoManager {
    #[allow(dead_code)]
    user_utxos: HashMap<String, HashMap<OutPoint, TrackedUtxo>>,
}

#[derive(Clone, Debug)]
pub struct TrackedUtxo {
    pub outpoint: OutPoint,
    pub txout: TxOut,
    pub confirmations: u32,
}

impl Default for UtxoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UtxoManager {
    pub fn new() -> Self {
        Self {
            user_utxos: HashMap::new(),
        }
    }
}
