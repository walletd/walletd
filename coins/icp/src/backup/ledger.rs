// coins/icp/src/ledger.rs
use ic_agent::Agent;
use walletd_hd_key::HDNetworkType;
use crate::transaction::IcpTransaction;use serde::Serialize;use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("Transfer failed: {0}")]
    TransferFailed(String),
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Invalid account")]
    InvalidAccount,
    #[error("Call error: {0}")]
    CallError(String),
}

/// ICP Ledger account identifier
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
#[derive(Serialize)]
pub struct AccountIdentifier(pub Vec<u8>);

/// Transfer arguments for ICP ledger
#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    pub to: AccountIdentifier,
    pub fee: Tokens,
    pub memo: u64,
    pub from_subaccount: Option<[u8; 32]>,
    pub created_at_time: Option<TimeStamp>,
    pub amount: Tokens,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Tokens {
    pub e8s: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TimeStamp {
    pub timestamp_nanos: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TransferResult {
    Ok(u64), // Block height
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

/// ICP Ledger client
pub struct IcpLedger {
    ledger_canister_id: Principal,
}

impl IcpLedger {
    pub fn new(network: walletd_hd_key::HDNetworkType) -> Self {
        let ledger_canister_id = match network {
            walletd_hd_key::HDNetworkType::MainNet => {
                Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()
            }
            walletd_hd_key::HDNetworkType::TestNet => {
                Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap() // Same for now
            }
        };
        
        Self { ledger_canister_id }
    }
    
    /// Get account balance
    pub async fn balance(&self, account: AccountIdentifier) -> Result<u64, LedgerError> {
        let args = (account,);
        let result: CallResult<(Tokens,)> = 
            ic_cdk::call(self.ledger_canister_id, "account_balance", args).await;
        
        match result {
            Ok((balance,)) => Ok(balance.e8s),
            Err((code, msg)) => Err(LedgerError::CallError(
                format!("Error {}: {}", code as u8, msg)
            )),
        }
    }
    
    /// Transfer ICP tokens
    pub async fn transfer(
        &self,
        to: AccountIdentifier,
        amount: u64,
        fee: u64,
        memo: u64,
        from_subaccount: Option<[u8; 32]>,
    ) -> Result<u64, LedgerError> {
        let args = TransferArgs {
            to,
            fee: Tokens { e8s: fee },
            memo,
            from_subaccount,
            created_at_time: None,
            amount: Tokens { e8s: amount },
        };
        
        let result: CallResult<(TransferResult,)> = 
            ic_cdk::call(self.ledger_canister_id, "transfer", (args,)).await;
        
        match result {
            Ok((TransferResult::Ok(block_height),)) => Ok(block_height),
            Ok((TransferResult::Err(err),)) => {
                Err(LedgerError::TransferFailed(format!("{:?}", err)))
            }
            Err((code, msg)) => Err(LedgerError::CallError(
                format!("Error {}: {}", code as u8, msg)
            )),
        }
    }
    
    /// Convert Principal to AccountIdentifier
    pub fn principal_to_account(principal: &Principal) -> AccountIdentifier {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(b"\x0Aaccount-id");
        hasher.update(principal.as_slice());
        hasher.update(&[0u8; 32]); // Default subaccount
        
        let hash = hasher.finalize();
        AccountIdentifier(hash[..].to_vec())
    }
}

/// Helper to convert ICP amount to e8s (1 ICP = 10^8 e8s)
pub fn icp_to_e8s(icp: f64) -> u64 {
    (icp * 100_000_000.0) as u64
}

/// Helper to convert e8s to ICP
pub fn e8s_to_icp(e8s: u64) -> f64 {
    e8s as f64 / 100_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_icp_conversion() {
        assert_eq!(icp_to_e8s(1.0), 100_000_000);
        assert_eq!(icp_to_e8s(0.5), 50_000_000);
impl IcpLedger {
    pub async fn transfer_icp(
        &self,
        transaction: &IcpTransaction,
        agent: &Agent,
    ) -> Result<u64, LedgerError> {
        let ledger_canister_id = match self.network {
            HDNetworkType::MainNet => Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
            HDNetworkType::TestNet => Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        };
        
        let transfer_args = transaction.to_transfer_args();
        
        // For now, return a mock block index
        // In real implementation, this would call the ledger canister
        Ok(12345)
    }
    
    pub async fn get_balance(
        &self,
        account: &AccountIdentifier,
        agent: &Agent,
    ) -> Result<u64, LedgerError> {
        // For now, return the existing balance method
        self.balance(account.clone()).await
    }
}
    }
}
