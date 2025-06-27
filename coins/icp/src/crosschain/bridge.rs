use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CrossChainBridge: Send + Sync {
    async fn lock_funds(&self, amount: u64, secret_hash: &str) -> Result<()>;
    async fn unlock_funds(&self, secret: &str) -> Result<()>;
}
