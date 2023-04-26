use crate::Error;
use async_trait::async_trait;

/// BlockchainConnector trait is used to connect to a blockchain and send and receive information to and from the blockchain.
#[async_trait]
pub trait BlockchainConnector {
    /// ErrorType is the type of error that is returned by the BlockchainConnector
    type ErrorType: std::error::Error + Send + Sync + 'static;

    /// new creates a new BlockchainConnector with a given url
    fn new(url: &str) -> Result<Self, Self::ErrorType>
    where
        Self: Sized;

    /// url returns the url of the BlockchainConnector
    fn url(&self) -> &str;

    // TODO(AS): return the fee estimates.. also consider adding functions to get common blockchain functions

    /// Returns the builder that can be used to build a BlockchainConnector with custom options
    fn builder() -> BlockchainConnectorBuilder<Self>
    where
        Self: Sized + Clone + BlockchainConnector,
    {
        BlockchainConnectorBuilder::new()
    }
}

/// ConnectorType is an enum that represents the type of connector that is being used, the different enum variants are meant to bue used with different cryptocurrency types and the generic type T is meant to be a specific struct that implements the BlockchainConnector trait
#[derive(Debug, Clone, Copy)]
pub enum ConnectorType<T>
where
    T: BlockchainConnector + Clone,
{
    /// BTC is a variant that represents a connector that is used to connect to a Bitcoin blockchain
    BTC(T),
    /// ETH is a variant that represents a connector that is used to connect to an Ethereum blockchain
    ETH(T),
}

/// BlockchainConnectorBuilder is a builder that can be used to build a BlockchainConnector with custom options
#[derive(Debug, Clone, Default)]
pub struct BlockchainConnectorBuilder<T>
where
    T: BlockchainConnector + Clone,
{
    url: Option<String>,
    connector_type: Option<ConnectorType<T>>,
}

impl<T> BlockchainConnectorBuilder<T>
where
    T: BlockchainConnector + Clone,
{
    /// new creates a new BlockchainConnectorBuilder with default options (no url and no connector type specified)
    pub fn new() -> Self {
        Self {
            url: None,
            connector_type: None,
        }
    }

    /// This function sets the url of the BlockchainConnectorBuilder
    pub fn url(&mut self, url: String) -> Self {
        self.url = Some(url);
        self.clone()
    }

    /// This function sets the connector type of the BlockchainConnectorBuilder, this requires the associated BlockchainConnector struct to be fully defined with data
    pub fn connector(&mut self, connector_type: ConnectorType<T>) -> Self {
        self.connector_type = Some(connector_type);
        self.clone()
    }

    /// This function builds the BlockchainConnectorBuilder using the options provided in the builder
    ///
    /// It returns a [BlockchainConnector] that can be used to connect to a blockchain.
    /// The result of that build later can be downcasted to a specific [BlockchainConnector] struct - any compatible struct that implements the [BlockchainConnector] trait.
    pub fn build(&mut self) -> Result<T, Error> {
        let connector_type = self.connector_type.clone();
        match connector_type {
            Some(ConnectorType::BTC(connector) | ConnectorType::ETH(connector)) => Ok(connector),
            None => match self.url {
                Some(ref url) => {
                    let client = T::new(url)
                        .map_err(|e| Error::BlockchainConnectorBuilder(e.to_string()))?;
                    let client_gen = client;
                    Ok(client_gen)
                }
                None => Err(Error::BlockchainConnectorBuilder("url not set".into())),
            },
        }
    }
}
