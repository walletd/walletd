use thiserror::Error;

/// Custom error type for the walletD crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Error from the walletd_bitcoin crate
    #[error("walletd_bitcoin error: {0}")]
    WalletdBitcoin(#[from] walletd_bitcoin::Error),
    /// Error from the walletd_ethereum crate
    #[error("walletd_ethereum error: {0}")]
    WalletdEthereum(#[from] walletd_ethereum::Error),
    /// Error from the walletd_hd_key crate
    #[error("walletd_hd_key error: {0}")]
    WalletdHDKey(#[from] walletd_hd_key::Error),
    /// FromHexError
    #[error("hex error: {0}")]
    Hex(#[from] hex::FromHexError),
    /// Error deriving a wallet of a specific type from a KeyPair struct
    #[error("Error deriving a wallet of a specific type from a KeyPair struct: {0}")]
    DeriveWallet(String),
    /// Missing info to generate a KeyPair struct
    #[error("Missing info to generate a KeyPair struct: {0}")]
    MissingKeyPairInfo(String),
}
