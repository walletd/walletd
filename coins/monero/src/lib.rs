pub use walletd_coin_core::{CryptoAmount, CryptoWallet};
pub use walletd_hd_key::{HDKey, HDNetworkType};
pub use walletd_monero_mnemonic::{Mnemonic, MnemonicExt, Seed};

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
pub use hash::Hash;
pub use network::Network;
pub mod fee_utils;
pub mod payment_id;
pub use payment_id::{PaymentId, PaymentIdStyle};
pub mod key_image;
pub use key_image::{KeyDerivation, KeyImage};
pub mod monero_lws;
pub use monero_lws::MoneroLWSConnection;
pub mod mix_outs;
pub use mix_outs::MixAmountAndOuts;
pub mod generators_bulletproof_plus;
pub mod monero_serialize;
pub use monero_serialize::{DoSerialize, SerializedArchive};
pub mod rct_types;
pub mod transaction;
pub mod varint;
pub use varint::{VarInt, VarIntEncoding};


pub struct BlockchainClient(pub reqwest::Client);

impl BlockchainClient {
    pub fn new(_url: &str) -> Result<Self, anyhow::Error> {
        Ok(Self(reqwest::Client::new()))
    }
}
