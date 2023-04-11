use async_trait::async_trait;
use std::any::Any;
use std::fmt;
use crate::Error;


#[async_trait]
pub trait BlockchainConnector: {
    type ErrorType: std::error::Error + fmt::Display + Send + Sync + 'static;

    fn new(url: &str) -> Result<Self, Self::ErrorType>
    where
        Self: Sized;

    fn url(&self) -> &str;
    async fn display_fee_estimates(&self) -> Result<String, Self::ErrorType>;

    fn builder() -> BlockchainConnectorBuilder<Self>
    where
        Self: Sized + Clone + BlockchainConnectorGeneral
    {
        BlockchainConnectorBuilder::new()
    }
}

pub trait BlockchainConnectorGeneral {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn BlockchainConnectorGeneral>;
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectorType<T> where T: BlockchainConnector + Clone {
    BTC(T),
    ETH(T),
}

#[derive(Debug, Clone, Default)]
pub struct BlockchainConnectorBuilder<T> where T: BlockchainConnector + Clone {
    url: Option<String>,
    connector_type: Option<ConnectorType<T>>,
}


impl<T> BlockchainConnectorBuilder<T> where T: BlockchainConnector + BlockchainConnectorGeneral + Clone {   

    pub fn new() -> Self {
        Self { url: None, connector_type: None}
    }

    pub fn set_url(&mut self, url: String) -> Self {
        self.url = Some(url);
        self.clone()
    }

    pub fn set_connector(&mut self, connector_type: ConnectorType<T>) -> Self {
        self.connector_type = Some(connector_type);
        self.clone()
    }

    pub fn build(&mut self) -> Result<Box<dyn BlockchainConnectorGeneral>, Error> {
        match &self.connector_type {
            Some(ConnectorType::BTC(connector) | ConnectorType::ETH(connector)) => {
                Ok(connector.box_clone())
            }
            None => {
                    match self.url {
                    Some(ref url) => {
                        let client = T::new(url).map_err(|e| Error::BlockchainConnectorBuilder(e.to_string()))?;
                        let client_gen = client.box_clone();
                    Ok(client_gen)
                }
                    None => Err(Error::BlockchainConnectorBuilder("url not set".into())),
                }

            }
        }
    }
}