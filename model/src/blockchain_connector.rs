use async_trait::async_trait;
use std::any::Any;
use std::fmt;
use crate::Error;


/// BlockchainConnector trait is used to connect to a blockchain and send and receive information to and from the blockchain.
#[async_trait]
pub trait BlockchainConnector: {
    /// ErrorType is the type of error that is returned by the BlockchainConnector
    type ErrorType: std::error::Error + fmt::Display + Send + Sync + 'static;

    /// new creates a new BlockchainConnector with a given url
    fn new(url: &str) -> Result<Self, Self::ErrorType>
    where
        Self: Sized;

    /// url returns the url of the BlockchainConnector
    fn url(&self) -> &str;

    /// Returns in a string format information about the current fee estimates for the blockchain
    async fn display_fee_estimates(&self) -> Result<String, Self::ErrorType>;

    /// Returns the builder that can be used to build a BlockchainConnector with custom options
    fn builder() -> BlockchainConnectorBuilder<Self>
    where
        Self: Sized + Clone + BlockchainConnectorGeneral
    {
        BlockchainConnectorBuilder::new()
    }
}

/// BlockchainConnectorGeneral is a general trait that can work with any struct that implements the BlockchainConnector trait
pub trait BlockchainConnectorGeneral {
    /// Returns a dyn Any reference to the BlockchainConnector
    fn as_any(&self) -> &dyn Any;

    /// Returns a clone in a box type 
    fn box_clone(&self) -> Box<dyn BlockchainConnectorGeneral>;
}

/// ConnectorType is an enum that represents the type of connector that is being used, the different enum variants are meant to bue used with different cryptocurrency types and the generic type T is meant to be a specific struct that implements the BlockchainConnector trait
#[derive(Debug, Clone, Copy)]
pub enum ConnectorType<T> where T: BlockchainConnector + Clone {
    /// BTC is a variant that represents a connector that is used to connect to a Bitcoin blockchain
    BTC(T),
    /// ETH is a variant that represents a connector that is used to connect to an Ethereum blockchain
    ETH(T),
}

/// BlockchainConnectorBuilder is a builder that can be used to build a BlockchainConnector with custom options
#[derive(Debug, Clone, Default)]
pub struct BlockchainConnectorBuilder<T> where T: BlockchainConnector + Clone {
    url: Option<String>,
    connector_type: Option<ConnectorType<T>>,
}



impl<T> BlockchainConnectorBuilder<T> where T: BlockchainConnector + BlockchainConnectorGeneral + Clone {   
    /// new creates a new BlockchainConnectorBuilder with default options (no url and no connector type specified) 
    pub fn new() -> Self {
        Self { url: None, connector_type: None}
    }

    /// set_url sets the url of the BlockchainConnectorBuilder
    pub fn set_url(&mut self, url: String) -> Self {
        self.url = Some(url);
        self.clone()
    }

    /// set_connector sets the connector type of the BlockchainConnectorBuilder, this requires the associated BlockchainConnector struct to be fully defined with data
    pub fn set_connector(&mut self, connector_type: ConnectorType<T>) -> Self {
        self.connector_type = Some(connector_type);
        self.clone()
    }

    /// build builds the BlockchainConnectorBuilder and returns a Box<dyn BlockchainConnectorGeneral> that can be used to connect to a blockchain
    /// The result of build can be downcasted to a specific BlockchainConnector struct
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