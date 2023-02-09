pub use walletd_coin_model::{CryptoAmount, CryptoCoin, CryptoWallet, CryptoWalletGeneral};
pub use walletd_hd_keys::{HDKeyPair, NetworkType};
pub use walletd_monero_mnemonic::{Mnemonic, MnemonicHandler, Seed};

pub mod hash;
pub use hash::keccak256;
pub mod monero_amount;
pub use monero_amount::MoneroAmount;
pub mod address;
pub use address::{Address, AddressType, SubaddressIndex, SubaddressKeys};
mod monero_wallet;
pub use monero_wallet::MoneroWallet;
mod private_key;
pub use private_key::PrivateKey;
mod public_key;
pub use public_key::PublicKey;
mod monero_private_keys;
pub use monero_private_keys::MoneroPrivateKeys;
mod monero_public_keys;
pub use monero_public_keys::MoneroPublicKeys;
pub mod network;
pub use network::Network;
pub mod payment_id;
pub use payment_id::{PaymentId, PaymentIdStyle};

pub struct BlockchainClient {
    blockchain_client: reqwest::Client,
}

impl BlockchainClient {
    pub fn new(url: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            blockchain_client: reqwest::Client::new(),
        })
    }
}
