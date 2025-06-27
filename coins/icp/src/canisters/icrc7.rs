//! ICRC-7 NFT Standard Implementation

use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use super::{CanisterClient, CanisterError};
use super::icrc1::Account;

/// NFT token identifier
pub type TokenId = Nat;

/// NFT metadata
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct TokenMetadata {
   pub token_id: TokenId,
   pub owner: Account,
   pub metadata: Vec<(String, MetadataValue)>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum MetadataValue {
   Text(String),
   Blob(Vec<u8>),
   Nat(Nat),
   Int(i64),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct TransferArg {
   pub from_subaccount: Option<[u8; 32]>,
   pub to: Account,
   pub token_id: TokenId,
   pub memo: Option<Vec<u8>>,
   pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TransferError {
   NonExistingTokenId,
   InvalidRecipient,
   Unauthorized,
   TooOld,
   CreatedInFuture { ledger_time: u64 },
   Duplicate { duplicate_of: Nat },
   TemporarilyUnavailable,
   GenericError { error_code: Nat, message: String },
}

pub type TransferResult = Result<Nat, TransferError>;

/// ICRC-7 NFT client
pub struct Icrc7Client {
   client: CanisterClient,
}

impl Icrc7Client {
   pub fn new(client: CanisterClient) -> Self {
       Self { client }
   }
   
   /// Get collection name
   pub async fn name(&self) -> Result<String, CanisterError> {
       self.client.query("icrc7_name", ()).await
   }
   
   /// Get collection symbol
   pub async fn symbol(&self) -> Result<String, CanisterError> {
       self.client.query("icrc7_symbol", ()).await
   }
   
   /// Get total supply of NFTs
   pub async fn total_supply(&self) -> Result<Nat, CanisterError> {
       self.client.query("icrc7_total_supply", ()).await
   }
   
   /// Get balance (number of NFTs) for an account
   pub async fn balance_of(&self, account: Account) -> Result<Nat, CanisterError> {
       self.client.query("icrc7_balance_of", (account,)).await
   }
   
   /// Get owner of a specific token
   pub async fn owner_of(&self, token_id: TokenId) -> Result<Option<Account>, CanisterError> {
       self.client.query("icrc7_owner_of", (token_id,)).await
   }
   
   /// Get metadata for a token
   pub async fn metadata(&self, token_id: TokenId) -> Result<TokenMetadata, CanisterError> {
       self.client.query("icrc7_metadata", (token_id,)).await
   }
   
   /// Transfer an NFT
   pub async fn transfer(&self, args: TransferArg) -> Result<TransferResult, CanisterError> {
       self.client.update("icrc7_transfer", (args,)).await
   }
}
