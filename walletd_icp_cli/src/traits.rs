use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait WalletOperations {
    async fn new_address(&self) -> Result<String>;
    async fn get_balance(&self, address: &str) -> Result<u64>;
    async fn build_transaction(&self, to: &str, amount: u64) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait KeyManager {
    async fn sign_transaction(&self, tx: &[u8]) -> Result<Vec<u8>>;
    async fn get_public_key(&self) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait NetworkProvider {
    async fn broadcast(&self, tx: &[u8]) -> Result<String>;
    async fn get_transaction(&self, txid: &str) -> Result<TransactionInfo>;
}

#[async_trait]
pub trait WalletStorage {
    async fn store(&self, key: &str, value: &[u8]) -> Result<()>;
    async fn retrieve(&self, key: &str) -> Result<Vec<u8>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub txid: String,
    pub status: String,
    pub confirmations: u32,
}
