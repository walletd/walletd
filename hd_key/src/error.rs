use thiserror::Error;

/// Custom error type for this crate
#[derive(Error, Debug)]
pub enum Error {
    /// Error returned when invalid derivation path or specification is used
    #[error("Invalid derivation path or specification: {0}")]
    Invalid(String),
    /// Error converting to a type when parsing from a string
    #[error("Error converting to a type from a string: {0}")]
    FromStr(String),
    /// Error due to missing public key
    #[error("Missing public key")]
    MissingPublicKey,
    /// Error due to missing private key
    #[error("Missing private key")]
    MissingPrivateKey,
    /// Error due to unable to serialize a key
    #[error("Cannot serialize key: {0}")]
    CannotSerializeKey(String),
    /// Error returned when some feature or option is currently not supported.
    #[error("Currently not supported: {0}")]
    CurrentlyNotSupported(String),
    /// Error returned from secp256k1
    #[error("Secp256k1 error: {0}")]
    Secp256k1(#[from] secp256k1::Error),
}
