

use std::any::Any;
use walletd_coin_model::BlockchainConnector;
use web3::api::Eth;
use web3::transports::Http;
use web3::Error;
use crate::{EthereumAmount, EthClient};
use async_trait::async_trait;


#[derive(Debug, Clone)]
pub struct BlockchainClient {
    client: web3::Web3<Http>,
    eth: Eth<Http>,
    url: String,
}

#[async_trait]
impl BlockchainConnector for BlockchainClient {
    fn new(url: &str) -> Result<Self, anyhow::Error> {
        let transport = web3::transports::Http::new(url)?;
        let web3 = web3::Web3::new(transport);
        let web3_eth = web3.eth();

        Ok(Self {
            client: web3,
            eth: web3_eth,
            url: url.to_string(),
        })
    }
    fn url(&self) -> &str {
        &self.url
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn display_fee_estimates(&self) -> Result<String, anyhow::Error> {
        let gas_price = self.gas_price().await?;
        let gas_price_gwei = gas_price.eth() * 1_000_000_000f64;
        let gas_price_string = format!("Gas Price: {} Gwei ({} ETH)", gas_price_gwei, gas_price.eth());
        Ok(gas_price_string)
    }

}


impl BlockchainClient {
    
    pub fn to_eth_client(&self) -> EthClient {
        EthClient::new(&self.url)
    }

    pub async fn balance(&self, address: web3::types::H160) -> Result<EthereumAmount, Error> {
        let balance = self.eth.balance(address, None).await?;
        Ok(EthereumAmount { wei: balance })
    }

    pub async fn gas_price(&self) -> Result<EthereumAmount, Error> {
        let gas_price = self.eth.gas_price().await?;
        Ok(EthereumAmount {wei: gas_price})
    }

    pub fn client(&self) -> &web3::Web3<Http> {
        &self.client
    }
    
    pub fn eth(&self) -> &Eth<Http> {
        &self.eth
    }
}
