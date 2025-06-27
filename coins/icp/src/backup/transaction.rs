use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransactionError {
   #[error("Invalid amount")]
   InvalidAmount,
   #[error("Invalid recipient")]
   InvalidRecipient,
   #[error("Serialization error: {0}")]
   Serialization(String),
   #[error("Signature error: {0}")]
   Signature(String),
}

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct IcpTransaction {
   pub from: Principal,
   pub to: Principal,
   pub amount: u64,
   pub fee: u64,
   pub memo: u64,
   pub created_at_time: Option<u64>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct TransferArgs {
   pub memo: u64,
   pub amount: u64,
   pub fee: u64,
   pub from_subaccount: Option<[u8; 32]>,
   pub to: Vec<u8>,
   pub created_at_time: Option<u64>,
}

impl IcpTransaction {
   pub fn new(
       from: Principal,
       to: Principal,
       amount: u64,
       fee: Option<u64>,
       memo: Option<u64>,
   ) -> Result<Self, TransactionError> {
       if amount == 0 {
           return Err(TransactionError::InvalidAmount);
       }
       
       Ok(Self {
           from,
           to,
           amount,
           fee: fee.unwrap_or(10_000),
           memo: memo.unwrap_or(0),
           created_at_time: None,
       })
   }
   
   pub fn to_transfer_args(&self) -> TransferArgs {
       TransferArgs {
           memo: self.memo,
           amount: self.amount,
           fee: self.fee,
           from_subaccount: None,
           to: self.to.as_slice().to_vec(),
           created_at_time: self.created_at_time,
       }
   }
   
   pub fn hash(&self) -> Result<Vec<u8>, TransactionError> {
       use sha2::{Sha256, Digest};
       let mut hasher = Sha256::new();
       
       hasher.update(self.from.as_slice());
       hasher.update(self.to.as_slice());
       hasher.update(self.amount.to_be_bytes());
       hasher.update(self.fee.to_be_bytes());
       hasher.update(self.memo.to_be_bytes());
       
       if let Some(timestamp) = self.created_at_time {
           hasher.update(timestamp.to_be_bytes());
       }
       
       Ok(hasher.finalize().to_vec())
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   
   #[test]
   fn test_transaction_creation() {
       let from = Principal::anonymous();
       let to = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
       
       let tx = IcpTransaction::new(from, to, 100_000_000, None, None);
       assert!(tx.is_ok());
       
       let tx = tx.unwrap();
       assert_eq!(tx.amount, 100_000_000);
       assert_eq!(tx.fee, 10_000);
       assert_eq!(tx.memo, 0);
   }
}
