use async_trait::async_trait;
use std::any::Any;


#[async_trait]
pub trait BlockchainConnector {
    fn new(url: &str) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn url(&self) -> &str;
    async fn display_fee_estimates(&self) -> Result<String, anyhow::Error>;
}