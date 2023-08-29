use thiserror::Error;

/// Custom error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Error returned when some feature or option is currently not supported.
    #[error("Currently not supported: {0}")]
    CurrentlyNotSupported(String),
    /// Error getting fee map
    #[error("Did not get fee map")]
    MissingFeeMap,
    /// Error due to insufficent funds
    #[error("Insufficent funds")]
    InsufficientFunds(String),
    /// Missing master HD key
    #[error("No master HD key set")]
    MissingMasterHDKey,
    /// Missing mnemonic seed
    #[error("Missing mnemonic")]
    MissingMnemonicSeed,
    /// Missing some info, error message provides more info on what is missing
    #[error("Missing info: {0}")]
    MissingInfo(String),
    /// Error getting transaction info
    #[error("Transaction info not available")]
    TransactionInfoUnavailable,
    /// Error broadcasting transaction
    #[error("Error in broadcasting the transaction: {0}")]
    BroadcastTransaction(String),
    /// Error with the transaction id hash
    #[error("Txid error: {0}")]
    TxId(String),
    /// Error with the Script Pub Key
    #[error("Script error: {0}")]
    ScriptInvalid(String),
    /// Missing data
    #[error("Missing data: {0}")]
    MissingData(String),
    /// Missing network type
    #[error("Missing network type")]
    MissingNetwork,
    /// Unable to import wallet, missing info to import a specific wallet
    #[error("Unable to import wallet: {0}")]
    UnableToImportWallet(String),
    /// Error converting to a type when parsing from a string
    #[error("Error converting to a type from a string: {0}")]
    FromStr(String),
    /// Error from trying to convert from/to an int or usigned int
    #[error("Error converting from/to int: {0}")]
    TryFromInt(#[from] std::num::TryFromIntError),
    /// Error from the walletd_hd_key crate
    #[error("Error from walletd_hd_key: {0}")]
    WalletdHDKey(#[from] walletd_hd_key::Error),
    /// Error due to overflow
    #[error("Overflow error: {0}")]
    Overflow(String),
}
