mod crypto_coin;
pub use crypto_coin::CryptoCoin;
mod crypto_wallet;
pub use crypto_wallet::{CryptoWallet, CryptoWalletGeneral};

pub struct BlockchainClient {}

pub trait BlockchainConnector {
    type BlockchainClient;

    fn setup_connection() -> Result<Self::BlockchainClient, anyhow::Error>;
}
