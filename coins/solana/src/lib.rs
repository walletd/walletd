extern crate solana_client;
use solana_client::rpc_client::RpcClient;
use hex; 
use core::{fmt, fmt::Display, str::FromStr};

const URL: &str = "https://api.devnet.solana.com";

use libsecp256k1::{PublicKey, SecretKey};
use walletd_coins::{CryptoCoin, CryptoWallet};
use walletd_bip39::{Language, Mnemonic, MnemonicType, MnemonicHandler};
use walletd_hd_keys::{BIP32, NetworkType};


#[derive(Default)]
pub enum SolanaFormat {
    #[default]
    Standard,
}

impl SolanaFormat {
    pub fn to_string(&self) -> String {
        match self {
            SolanaFormat::Standard => "Standard".to_string(),
        }
    }
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
    type MnemonicStyle = Mnemonic;
    type HDKeyInfo = BIP32;

    fn new_from_hd_keys(hd_keys: &BIP32) -> Result<Self, String> {
        Ok(Self {
            crypto_type: CryptoCoin::SOL,
            address_format: SolanaFormat::Standard,
            public_address: Self::public_address_from_public_key(&hd_keys.extended_public_key.unwrap().to_vec()),
            private_key: hd_keys.get_private_key_0x().unwrap(),
            public_key: hd_keys.get_public_key_0x().unwrap(),
            blockchain_client: None,
            network: hd_keys.network,
        })
    }

    fn new_from_mnemonic(mnemonic: Mnemonic) -> Result<Self, String>{
        let seed = mnemonic.get_seed_bytes()?;
        let public_key = PublicKey::from_secret_key(
            &libsecp256k1::SecretKey::parse_slice(&seed).unwrap()).serialize_compressed();
        let network = NetworkType::MainNet;
        Ok(Self {
            crypto_type: CryptoCoin::SOL,
            address_format: SolanaFormat::Standard,
            public_address: Self::public_address_from_public_key(&public_key.to_vec()),
            private_key: Self::to_0x_hex_format(seed)?,
            public_key: Self::to_0x_hex_format(&public_key)?,
            blockchain_client: None,
            network,
        })
    }
    fn get_public_address(&self) -> String {
        self.public_address.clone()
    }
}    


impl SolanaWallet {
    pub fn public_address_from_public_key(public_key: &Vec<u8>) -> String {
        hex::encode(public_key)
    }

   
}

impl Display for SolanaWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Solana Wallet")?;
        writeln!(f, " Network: {}", self.network)?;
        writeln!(f, " Private Key: {}", self.private_key)?;
        writeln!(f, " Public Key: {}", self.public_key)?;
        writeln!(f, " Address Format: {}", self.address_format.to_string())?;
        writeln!(f, " Public Address: {}", self. public_address)?;
        Ok(())
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
