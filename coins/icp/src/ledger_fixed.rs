use thiserror::Error;
use serde::{Serialize, Deserialize};
use candid::{CandidType, Principal};
use sha2::{Sha256, Digest};

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("Account not found")]
    AccountNotFound,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Transfer failed: {0}")]
    TransferFailed(String),
    #[error("Agent error: {0}")]
    Agent(String),
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct AccountIdentifier(pub Vec<u8>);

impl AccountIdentifier {
    pub fn new(principal: &Principal, subaccount: Option<[u8; 32]>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(b"\x0Aaccount-id");
        hasher.update(principal.as_slice());
        hasher.update(&subaccount.unwrap_or([0; 32]));
        let hash = hasher.finalize();
        
        let mut account_id = vec![0; 32];
        account_id.copy_from_slice(&hash);
        AccountIdentifier(account_id)
    }
}

impl Clone for AccountIdentifier {
    fn clone(&self) -> Self {
        AccountIdentifier(self.0.clone())
    }
}

impl std::fmt::Display for AccountIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

#[derive(CandidType, Deserialize)]
pub struct Tokens {
    pub e8s: u64,
}

impl From<u64> for Tokens {
    fn from(e8s: u64) -> Self {
        Tokens { e8s }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub memo: u64,
    pub amount: Tokens,
    pub fee: Tokens,
    pub from_subaccount: Option<[u8; 32]>,
    pub to: Vec<u8>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferResult {
    Ok(u64),
    Err(TransferError),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TransferError {
    BadFee { expected_fee: Tokens },
    InsufficientFunds { balance: Tokens },
    TxTooOld { allowed_window_nanos: u64 },
    TxCreatedInFuture,
    TxDuplicate { duplicate_of: u64 },
}

use walletd_hd_key::HDNetworkType;

pub struct IcpLedger {
    network: HDNetworkType,
    ledger_canister_id: Principal,
}

impl IcpLedger {
    pub fn new(network: HDNetworkType) -> Self {
        let ledger_canister_id = match network {
            HDNetworkType::MainNet => {
                Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()
            }
            HDNetworkType::TestNet => {
                Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap() // Same for now
            }
        };
        
        Self {
            network,
            ledger_canister_id,
        }
    }
    
    pub async fn balance(&self, account: AccountIdentifier) -> Result<u64, LedgerError> {
        // In production, this would make an actual IC call
        // For now, return a mock balance
        Ok(1_000_000_000) // 10 ICP
    }
    
    pub async fn transfer(
        &self,
        to: AccountIdentifier,
        amount: u64,
        fee: u64,
        memo: u64,
        from_subaccount: Option<[u8; 32]>,
    ) -> Result<u64, LedgerError> {
        // In production, this would make an actual transfer
        // For now, return a mock block index
        Ok(12345)
    }
    
    pub fn principal_to_account(principal: &Principal) -> AccountIdentifier {
        AccountIdentifier::new(principal, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_identifier() {
        let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let account = IcpLedger::principal_to_account(&principal);
        assert!(!account.0.is_empty());
    }
}
