use thiserror::Error;

/// Custom error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Error due to missing public key
    #[error("Public Key not included")]
    MissingPublicKey,
    /// Error due to missing private key
    #[error("Private Key not included")]
    MissingPrivateKey,
    /// Missing blockchain client
    #[error("No blockchain client set")]
    MissingBlockchainClient,
    /// Missing master HD key
    #[error("No master HD key set")]
    MissingHDKey,
    /// Unable to import wallet, missing info to import a specific wallet
    #[error("Unable to import wallet: {0}")]
    UnableToImportWallet(String),
    /// Error converting to a type when parsing from a string
    #[error("Error converting to a type from a string: {0}")]
    FromStr(String),
    /// Converted ParseInt error
    #[error("ParseInt error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    /// Error related to converting from or to a hex
    #[error("Hex error: {0}")]
    Hex(#[from] hex::FromHexError),
    /// Error when trying to initialize EthClient
    #[error("Failed to initialize EthClient")]
    EthClientInit,
    /// Error related to a transaaction
    #[error("Failed to retrieve data for transaction: {0}")]
    TxResponse(String),
    /// Error related to block data
    #[error("Failed to retrieve block data")]
    BlockResponse,
    /// Error related to a smart contract filter
    #[error("Failed when processing a block to find smart contract transactions")]
    SmartContractFilter,
    /// Error when trying to retrieve a transaction from a transaction hash
    #[error("An error was encountered while trying to retrieve a tx from a tx hash")]
    GetTx,
    /// Error due to overflow
    #[error("Overflow error: {0}")]
    Overflow(String),
}
