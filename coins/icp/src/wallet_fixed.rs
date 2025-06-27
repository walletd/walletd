use thiserror::Error;
use candid::Principal;
use ic_agent::Agent;
use walletd_hd_key::{HDKey, HDNetworkType};
use serde::{Deserialize, Serialize};
use crate::{
    keys::{IcpKeyManager, KeyError},
    ledger::{IcpLedger, AccountIdentifier, LedgerError},
    did::{DIDError, IcpDID, DIDDocument},
    transaction::{IcpTransaction, TransactionError},
};

#[derive(Debug, Error)]
pub enum IcpWalletError {
    #[error("Invalid principal: {0}")]
    InvalidPrincipal(String),
    #[error("HD key error")]
    HdKey,
    #[error("Key error: {0}")]
    Key(#[from] KeyError),
    #[error("Ledger error: {0}")]
    Ledger(#[from] LedgerError),
    #[error("DID error: {0}")]
    Did(#[from] DIDError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcpWallet {
    principal: Principal,
    account_identifier: AccountIdentifier,
    network: HDNetworkType,
    did: Option<String>,
}

impl IcpWallet {
    pub fn from_principal(principal: Principal, network: HDNetworkType) -> Self {
        let account_identifier = IcpLedger::principal_to_account(&principal);
        Self { 
            principal, 
            account_identifier,
            network,
            did: None,
        }
    }

    pub fn from_hd_key(hd_key: &HDKey, _account: u32) -> Result<Self, IcpWalletError> {
        let principal = IcpKeyManager::principal_from_hd_key(hd_key)?;
        let account_identifier = IcpLedger::principal_to_account(&principal);
        
        Ok(Self {
            principal,
            account_identifier,
            network: HDNetworkType::MainNet,
            did: None,
        })
    }

    pub fn principal(&self) -> &Principal {
        &self.principal
    }

    pub fn account_identifier(&self) -> &AccountIdentifier {
        &self.account_identifier
    }

    pub fn address(&self) -> String {
        self.account_identifier.to_string()
    }

    pub fn create_transaction(
        &self,
        to: Principal,
        amount: u64,
        memo: Option<u64>,
    ) -> Result<IcpTransaction, IcpWalletError> {
        IcpTransaction::new(
            self.principal,
            to,
            amount,
            Some(10_000), // Default fee
            memo,
        ).map_err(|e| e.into())
    }

    pub async fn send_transaction(
        &self,
        transaction: &IcpTransaction,
        _private_key: &[u8],
        _agent: &Agent,
    ) -> Result<u64, IcpWalletError> {
        let ledger = IcpLedger::new(self.network);
        
        // Convert transaction to ledger transfer
        let to_account = IcpLedger::principal_to_account(&transaction.to);
        let block_index = ledger.transfer(
            to_account,
            transaction.amount,
            transaction.fee,
            transaction.memo,
            None, // from_subaccount
        ).await?;
        
        Ok(block_index)
    }

    pub async fn get_balance(&self, _agent: &Agent) -> Result<u64, IcpWalletError> {
        let ledger = IcpLedger::new(self.network);
        let balance = ledger.balance(self.account_identifier.clone()).await?;
        Ok(balance)
    }

    pub async fn create_did(
        &mut self,
        _public_key: Vec<u8>,
        _agent: &Agent,
    ) -> Result<DIDDocument, IcpWalletError> {
        let icp_did = IcpDID::create(self.principal)?;
        
        // In production, this would register on-chain
        let document = icp_did.document().clone();
        self.did = Some(document.id.clone());
        
        Ok(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);
        assert_eq!(wallet.principal(), &principal);
    }
}
