use thiserror::Error;

#[derive(Error, Debug)]
pub enum BaseError {
    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Wallet error: {0}")]
    WalletError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}
