use thiserror::Error;

#[derive(Error, Debug)]
pub enum IcpWalletError {
    #[error("Invalid principal: {0}")]
    InvalidPrincipal(String),

    #[error("Invalid mnemonic: {0}")]
    InvalidMnemonic(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<anyhow::Error> for IcpWalletError {
    fn from(err: anyhow::Error) -> Self {
        IcpWalletError::Other(err.to_string())
    }
}
