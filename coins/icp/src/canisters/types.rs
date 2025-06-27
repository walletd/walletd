//! Common types for canister interactions

use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct CanisterMethod {
   pub name: String,
   pub args: Vec<u8>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CanisterResponse {
   pub status: ResponseStatus,
   pub data: Vec<u8>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum ResponseStatus {
   Ok,
   Err(String),
}
