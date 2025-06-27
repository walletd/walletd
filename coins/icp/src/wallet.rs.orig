use thiserror::Error;
use candid::Principal;
use crate::{IcpTransaction, IcpDID, DIDDocument, transaction::TransactionError};use ic_agent::Agent;
use walletd_hd_key::{HDKey, HDNetworkType};
use serde::{Deserialize, Serialize};
use crate::{
    keys::{IcpKeyManager, KeyError},
    ledger::{IcpLedger, AccountIdentifier, LedgerError},
    did::DIDError,
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
        })
    }
    
    pub fn principal(&self) -> Principal {
        self.principal
    }
    
    pub fn network(&self) -> HDNetworkType {
        self.network
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wallet_creation() {
        let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);
        
        assert_eq!(wallet.principal(), principal);
        assert_eq!(wallet.network(), HDNetworkType::MainNet);
    }
}

// Add these methods to the IcpWallet impl block:
impl IcpWallet {
    /// Create and sign a transaction
    pub async fn create_transaction(
        &self,
        to: Principal,
        amount: u64,
        memo: Option<u64>,
        private_key: &[u8],
    ) -> Result<IcpTransaction, IcpWalletError> {
        let tx = IcpTransaction::new(
            self.principal,
            to,
            amount,
            None, // Use default fee
            memo,
        )?;
        
        Ok(tx)
    }
    
    /// Send a transaction
    pub async fn send_transaction(
        &self,
        transaction: &IcpTransaction,
        agent: &Agent,
    ) -> Result<u64, IcpWalletError> {
        let ledger = IcpLedger::new(self.network);
        let block_index = ledger.transfer_icp(transaction, agent).await?;
        Ok(block_index)
    }
    
    /// Get wallet balance
    pub async fn get_balance(&self, agent: &Agent) -> Result<u64, IcpWalletError> {
        let ledger = IcpLedger::new(self.network);
        let balance = ledger.get_balance(&self.account_identifier, agent).await?;
        Ok(balance)
    }
    
    /// Create and register DID
    pub async fn create_and_register_did(
        &mut self,
        public_key: Vec<u8>,
        agent: &Agent,
    ) -> Result<DIDDocument, IcpWalletError> {
        let did_doc = IcpDID::create_did_document(self.principal, public_key)?;
        
        let icp_did = IcpDID::new();
        icp_did.register_did_on_chain(&did_doc, agent).await?;
        
        self.did = Some(did_doc.id.clone());
        Ok(did_doc)
    }
}
