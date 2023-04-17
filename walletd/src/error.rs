use thiserror::Error;

/// Custom error type for this crate.
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
    /// Error from the walletd_bip39 crate
    #[error("walletd_bip39 mnemonic error: {0}")]
    WalletdBip39(#[from] walletd_bip39::ParseMnemonicError),
    /// Error from the walletd_coin_model crate
    #[error("walletd_coin_model error: {0}")]
    WalletdCoinModel(#[from] walletd_coin_model::Error),
    /// Error deriving a wallet of a specific type from a KeyPair struct
    #[error("Error deriving a wallet of a specific type from a KeyPair struct: {0}")]
    DeriveWallet(String),
}
