// src/core/provider.rs

use crate::core::errors::WalletDError;
use crate::core::types::Transaction;
use async_trait::async_trait;

#[async_trait]
pub trait BlockchainProvider {
    async fn create_account(&self) -> Result<(), WalletDError>;
    async fn send_transaction(&self, txn: Transaction) -> Result<(), WalletDError>;

    // New methods
    async fn send_hbar(
        &self,
        recipient_account_id: &str,
        amount: u64,
    ) -> Result<(), WalletDError>;
    async fn transfer_tokens(
        &self,
        token_id: &str,
        recipient_account_id: &str,
        amount: u64,
    ) -> Result<(), WalletDError>;
}
