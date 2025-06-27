use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid derivation path or specification: {0}")]
    Invalid(String),
    #[error("Error converting to a type from a string: {0}")]
    StringConversion(String),
    #[error("Missing public key")]
    MissingPublicKey,
    #[error("Missing private key")]
    MissingPrivateKey,
    #[error("Cannot serialize key: {0}")]
    CannotSerializeKey(String),
    #[error("Currently not supported: {0}")]
    CurrentlyNotSupported(String),
    #[error("Secp256k1 error: {0}")]
    Secp256k1(#[from] secp256k1::Error),
    #[error("HmacSha512 error: {0}")]
    HmacSha512(String),
    #[error("Index out of range: {index} > {max}")]
    IndexOutOfRange { index: u32, max: u32 },
    #[error("Hex error: {0}")]
    Hex(#[from] hex::FromHexError),
}