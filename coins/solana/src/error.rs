use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Solana client error: {0}")]
    Client(#[from] solana_client::client_error::ClientError),
}