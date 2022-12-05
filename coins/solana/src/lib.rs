extern crate solana_client;
use solana_client::rpc_client::RpcClient;

const URL: &str = "https://api.devnet.solana.com";

use walletd_coins::{CryptoCoin, CryptoTypeData};
use walletd_cryptowallet::CryptoWallet;
use walletd_bip39::{Language, Mnemonic, MnemonicType, MnemonicHandler};
use walletd_hd_keys::{BIP32, NetworkType};

#[derive(Default)]
pub enum SolanaFormat {
    #[default]
    Standard,
}

#[derive(Default)]
pub struct SolanaWallet {
    crypto_type: CryptoCoin,
    address_format: SolanaFormat,
    public_address: String,
    private_key: String,
    public_key: String, 
    network: NetworkType,
    blockchain_client: Option<RpcClient>,
}

impl CryptoWallet for SolanaWallet {
    fn new_from_hd_keys(hd_keys: BIP32) -> Result<Self, String> {
        Ok(Self {
            crypto_type: CryptoCoin::SOL,
            address_format: SolanaFormat::Standard,
            public_address: Self::public_address_from_public_key(&hd_keys.extended_public_key.unwrap().to_vec(), &hd_keys.network),
            private_key: hd_keys.get_private_key().unwrap(),
            public_key: hd_keys.get_public_key().unwrap(),
            blockchain_client: None,
            network: hd_keys.network,
        })
    }
}

impl SolanaWallet {
    pub fn public_address_from_public_key(public_key: &Vec<u8>, network_type: &NetworkType) -> String {
        "TEMP".to_string()
    }
}

pub struct BlockchainClient {
    blockchain_client: Option<RpcClient>,
}

impl BlockchainClient {
  pub fn new(url: &str) -> Result<Self, String> {
    Ok(Self {
      blockchain_client: Some(RpcClient::new(url)),
    })
  }
}
