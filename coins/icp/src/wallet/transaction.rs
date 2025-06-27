use candid::Principal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Principal,
    pub to: Principal,
    pub amount: u64,
    pub fee: Option<u64>,
    pub memo: Option<u64>,
    pub created_at: u64,
}

pub struct TransactionBuilder {
    transaction: Transaction,
}

impl TransactionBuilder {
    pub fn new(from: Principal) -> Self {
        Self {
            transaction: Transaction {
                from,
                to: Principal::anonymous(),
                amount: 0,
                fee: Some(10_000),
                memo: None,
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
        }
    }

    pub fn to(mut self, to: Principal) -> Self {
        self.transaction.to = to;
        self
    }

    pub fn amount(mut self, amount: u64) -> Self {
        self.transaction.amount = amount;
        self
    }

    pub fn memo(mut self, memo: u64) -> Self {
        self.transaction.memo = Some(memo);
        self
    }

    pub fn build(self) -> Transaction {
        self.transaction
    }
}
