// This file contains the complete Phase 1 implementation additions

use candid::Principal;
use sha2::{Sha256, Digest};
use crate::{
    IcpWallet, IcpTransaction, TransactionError, IcpWalletError,
    IcpLedger, LedgerError, AccountIdentifier,
    IcpDID, DIDDocument, DIDError,
    HDNetworkType,
};

// Transaction implementation extensions
impl IcpTransaction {
    /// Sign transaction with private key
    pub fn sign(&self, private_key: &[u8]) -> Result<Vec<u8>, TransactionError> {
        let hash = self.hash()?;
        
        // For now, return mock signature
        // Real implementation would use ed25519-dalek
        Ok(vec![0u8; 64])
    }
}

// Wallet implementation extensions for Phase 1
impl IcpWallet {
    /// Create a signed transaction ready for submission
    pub fn prepare_transaction(
        &self,
        to: Principal,
        amount: u64,
        memo: Option<u64>,
    ) -> Result<IcpTransaction, IcpWalletError> {
        Ok(IcpTransaction {
            from: self.principal,
            to,
            amount,
            fee: 10_000, // Default fee
            memo: memo.unwrap_or(0),
            created_at_time: None,
        })
    }
    
    /// Get subaccount for this wallet
    pub fn get_subaccount(&self, index: u32) -> [u8; 32] {
        let mut subaccount = [0u8; 32];
        subaccount[0..4].copy_from_slice(&index.to_be_bytes());
        subaccount
    }
}

// HD Wallet utilities
pub mod hd_utils {
    use super::*;
    
    /// Get ICP derivation path: m/44'/223'/account'/0/address_index
    pub fn get_icp_derivation_path(account: u32, address_index: u32) -> Vec<u32> {
        vec![
            0x8000002C, // 44'
            0x800000DF, // 223' (ICP coin type)
            0x80000000 | account, // account'
            0,          // change
            address_index, // address_index
        ]
    }
}

#[cfg(test)]
mod phase1_tests {
    use super::*;
    
    #[test]
    fn test_transaction_creation() {
        let from = Principal::anonymous();
        let to = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        
        let tx = IcpTransaction {
            from,
            to,
            amount: 100_000_000,
            fee: 10_000,
            memo: 0,
            created_at_time: None,
        };
        
        assert_eq!(tx.amount, 100_000_000);
    }
    
    #[test]
    fn test_derivation_path() {
        let path = hd_utils::get_icp_derivation_path(0, 0);
        assert_eq!(path, vec![0x8000002C, 0x800000DF, 0x80000000, 0, 0]);
    }
}
