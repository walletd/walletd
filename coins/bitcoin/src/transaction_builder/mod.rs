// Simplified transaction builder
pub mod psbt_handler;
pub mod script_builder;

use bitcoin::Transaction;

pub struct TransactionBuilder {
    tx: Transaction,
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            tx: Transaction {
                version: bitcoin::transaction::Version::TWO,
                lock_time: bitcoin::blockdata::locktime::absolute::LockTime::ZERO,
                input: vec![],
                output: vec![],
            },
        }
    }

    pub fn build(self) -> Transaction {
        self.tx
    }
}
