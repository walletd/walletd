use bitcoin::address::Error as BitcoinAddressError;
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
    MissingMasterHDKey,
    /// Error downcasting to BitcoinWallet
    #[error("Could not downcast to BitcoinWallet")]
    UnableToDowncastWallet,
    /// Error downcasting a blockchain connector
    #[error("Error downcasting: {0}")]
    UnableToDowncastBlockchainConnector(String),
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
    /// Error returned from the bitcoin crate, address module.
    #[error("Bitcoin address error: {0}")]
    Bitcoin(#[from] BitcoinAddressError),
    /// Error returned from secp256k1
    #[error("Secp256k1 error: {0}")]
    Secp256k1(#[from] secp256k1::Error),
    /// Bitcoin key error
    #[error("Bitcoin key error: {0}")]
    BitcoinKey(#[from] bitcoin::key::Error),
    /// Bitcoin PushBytesError
    #[error("Bitcoin PushBytesError: {0}")]
    BitcoinPushBytes(#[from] bitcoin::blockdata::script::PushBytesError),
    /// Error related to converting from or to a hex
    #[error("Hex error: {0}")]
    Hex(#[from] hex::FromHexError),
    /// Error from trying to convert from/to an int or usigned int
    #[error("Error converting from/to int: {0}")]
    TryFromInt(#[from] std::num::TryFromIntError),
    /// Error from serde_json
    #[error("Error converting from/to json: {0}")]
    SerdeJson(#[from] serde_json::Error),
    /// Error from reqwest
    #[error("Error from reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error from the walletd_hd_key crate
    #[error("Error from walletd_hd_key: {0}")]
    WalletdHDKey(#[from] walletd_hd_key::Error),
    /// Error when discerning the timestamp
    #[error("Error discerning the timestamp: {0}")]
    Timestamp(String),
    /// Converts a format error from the time crate
    #[error("Error from time crate: {0}")]
    TimeFormat(#[from] time::error::Format),
}
