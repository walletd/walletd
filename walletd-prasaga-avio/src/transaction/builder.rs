use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionBuilder {
    pub operations: Vec<Operation>,
    pub nonce: Option<u64>,
    pub gas_limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Transfer {
        to: String,
        amount: u128,
    },
    CreateObject {
        class_id: String,
        initial_state: serde_json::Value,
    },
    InvokeMethod {
        object_id: String,
        method: String,
        params: Vec<serde_json::Value>,
    },
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            nonce: None,
            gas_limit: None,
        }
    }

    pub fn add_operation(mut self, op: Operation) -> Self {
        self.operations.push(op);
        self
    }

    pub fn with_nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }

    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
