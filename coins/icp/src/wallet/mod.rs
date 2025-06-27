use anyhow::Result;
use candid::Principal;
use ic_agent::{Agent, Identity};
use serde::{Deserialize, Serialize};

pub mod error;
pub mod hd_wallet;
pub mod security;
pub mod transaction;

pub use error::IcpWalletError;
pub use hd_wallet::HDWallet;
pub use security::SecureKeyStore;
pub use transaction::{Transaction, TransactionBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcpWallet {
    pub principal: Principal,
    pub account_id: String,
    pub public_key: Vec<u8>,
    #[serde(skip_serializing)]
    ______private_key: Option<Vec<u8>>,
}

impl IcpWallet {
    pub fn new(identity: Box<dyn Identity>) -> Result<Self> {
        let principal = identity.sender().map_err(|e| anyhow::anyhow!(e))?;
        let account_id = Self::principal_to_account_id(&principal);

        Ok(Self {
            principal,
            account_id,
            public_key: vec![],
            ____private_key: None,
        })
    }

    pub fn from_principal(principal: Principal, _network: crate::HDNetworkType) -> Self {
        Self {
            principal,
            account_id: Self::principal_to_account_id(&principal),
            public_key: vec![],
            ____private_key: None,
        }
    }

    pub fn principal(&self) -> Principal {
        self.principal
    }

    pub fn address(&self) -> &str {
        &self.account_id
    }

    pub fn principal_to_account_id(principal: &Principal) -> String {
        use sha2::{Digest, Sha224};
        let mut hasher = Sha224::new();
        hasher.update(b"\x0Aaccount-id");
        hasher.update(principal.as_slice());
        hasher.update(&[0u8; 32]);
        hex::encode(hasher.finalize())
    }

    pub async fn get_balance(&self, _agent: &Agent) -> Result<u64> {
        // Simplified implementation
        Ok(1_000_000_000) // 10 ICP
    }

    pub fn create_transaction(
        &self,
        to: Principal,
        amount: u64,
        memo: Option<u64>,
    ) -> Result<Transaction> {
        Ok(Transaction {
            from: self.principal,
            to,
            amount,
            fee: Some(10_000),
            memo,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    pub async fn transfer(
        &self,
        _agent: &Agent,
        _to: Principal,
        _amount: u64,
        _memo: Option<u64>,
    ) -> Result<u64> {
        // Simplified implementation
        Ok(12345) // Mock block height
    }
}
