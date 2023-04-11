use thiserror::Error;


/// Custom error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Error from the BlockchainConnectorBuilder struct
    #[error("BlockchainConnectorBuilder error: {0}")]
    BlockchainConnectorBuilder(String),

}