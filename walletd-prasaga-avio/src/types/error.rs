use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Object operation failed: {0}")]
    ObjectOperationFailed(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("RPC error: {code}: {message}")]
    Rpc { code: i32, message: String },

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;
