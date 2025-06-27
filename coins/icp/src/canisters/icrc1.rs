//! ICRC-1 Token Standard Implementation

use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use super::{CanisterClient, CanisterError};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Account {
   pub owner: Principal,
   pub subaccount: Option<[u8; 32]>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct TransferArg {
   pub from_subaccount: Option<[u8; 32]>,
   pub to: Account,
   pub amount: Nat,
   pub fee: Option<Nat>,
   pub memo: Option<Vec<u8>>,
   pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TransferError {
   BadFee { expected_fee: Nat },
   BadBurn { min_burn_amount: Nat },
   InsufficientFunds { balance: Nat },
   TooOld,
   CreatedInFuture { ledger_time: u64 },
   Duplicate { duplicate_of: Nat },
   TemporarilyUnavailable,
   GenericError { error_code: Nat, message: String },
}

pub type TransferResult = Result<Nat, TransferError>;

/// ICRC-1 Token client
pub struct Icrc1Client {
   client: CanisterClient,
}

impl Icrc1Client {
   pub fn new(client: CanisterClient) -> Self {
       Self { client }
   }
   
   /// Get token name
   pub async fn name(&self) -> Result<String, CanisterError> {
       self.client.query("icrc1_name", ()).await
   }
   
   /// Get token symbol
   pub async fn symbol(&self) -> Result<String, CanisterError> {
       self.client.query("icrc1_symbol", ()).await
   }
   
   /// Get token decimals
   pub async fn decimals(&self) -> Result<u8, CanisterError> {
       self.client.query("icrc1_decimals", ()).await
   }
   
   /// Get total supply
   pub async fn total_supply(&self) -> Result<Nat, CanisterError> {
       self.client.query("icrc1_total_supply", ()).await
   }
   
   /// Get balance of an account
   pub async fn balance_of(&self, account: Account) -> Result<Nat, CanisterError> {
       self.client.query("icrc1_balance_of", (account,)).await
   }
   
   /// Transfer tokens
   pub async fn transfer(&self, args: TransferArg) -> Result<TransferResult, CanisterError> {
       self.client.update("icrc1_transfer", (args,)).await
   }
}
