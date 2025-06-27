// src/core/errors.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalletDError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Hedera SDK error: {0}")]
    HederaError(#[from] hedera::Error),

    #[error("General error: {0}")]
    GeneralError(String),
}
