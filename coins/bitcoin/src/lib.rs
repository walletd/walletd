extern crate bitcoincore_rpc;
extern crate reqwest;
use walletd_coins::{CryptoCoin, CryptoWallet};
use walletd_bip39::{Language, Mnemonic, MnemonicType, MnemonicHandler};
use walletd_hd_keys::{BIP32, NetworkType};
use bitcoincore_rpc::bitcoin::BlockHash;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use sha2::{Digest, Sha256, Sha512};
use base58::{FromBase58, ToBase58};
use bech32::ToBase32;
use ripemd::Ripemd160;
use core::{fmt, fmt::Display, str::FromStr};
use libsecp256k1::{PublicKey, SecretKey};

pub const USER: &str = "test";
pub const PASS: &str = "test";

#[derive(Default)]
pub enum BitcoinFormat {
    /// Pay-to-Pubkey Hash, e.g. 1NoZQSmjYHUZMbqLerwmT4xfe8A6mAo8TT
    P2PKH,
    /// Pay-to-Witness-Script Hash, e.g. 347N1Thc213QqfYCz3PZkjoJpNv5b14kBd
    P2WSH,
    /// SegWit Pay-to-Witness-Public-Key Hash, e.g. 34AgLJhwXrvmkZS1o5TrcdeevMt22Nar53
    P2SH_P2WPKH,
    /// Bech32, e.g. bc1pw508d6qejxtdg4y5r3zarvary0c5xw7kw508d6qejxtdg4y5r3zarvary0c5xw7k7grplx
    #[default]
    Bech32,
}
impl BitcoinFormat {
    pub fn to_string(&self) -> String {
        match self {
            BitcoinFormat::P2PKH => "P2PKH".to_string(),
            BitcoinFormat::P2WSH => "P2WSH".to_string(),
            BitcoinFormat::P2SH_P2WPKH => "P2SH_P2WPKH".to_string(),
            BitcoinFormat::Bech32 => "Bech32".to_string(), 
        }
    }
}

#[derive(Default)]
pub struct BitcoinWallet {
    crypto_type: CryptoCoin,
    address_format: BitcoinFormat,
    public_address: String,
    private_key: String,
    public_key: String,
    blockchain_client: Option<Client>,
    network: NetworkType,
}

impl CryptoWallet for BitcoinWallet {
    type HDKeyInfo = BIP32;
    type MnemonicStyle = Mnemonic;
    fn new_from_hd_keys(hd_keys: &BIP32) -> Result<Self, String> {
        Ok(Self {
            crypto_type: CryptoCoin::BTC,
            address_format: BitcoinFormat::Bech32,
            public_address: Self::public_address_bech32_from_public_key(&hd_keys.extended_public_key.unwrap().to_vec(), &hd_keys.network),
            private_key: hd_keys.get_private_key_wif().unwrap(),
            public_key: hd_keys.get_public_key_hex().unwrap(),
            blockchain_client: None,
            network: hd_keys.network,
        })
    }
    fn new_from_mnemonic(mnemonic: Self::MnemonicStyle) -> Result<Self, String> {
        let seed = mnemonic.get_seed_bytes()?;
        let public_key = PublicKey::from_secret_key(
            &libsecp256k1::SecretKey::parse_slice(&seed).unwrap()).serialize_compressed();
        let network = NetworkType::MainNet;

        Ok(Self {
            crypto_type: CryptoCoin::BTC,
            address_format: BitcoinFormat::Bech32,
            private_key: Self::to_private_key_wif(seed, 0x80)?,
            public_address: Self::public_address_bech32_from_public_key(&public_key.to_vec(), &network),
            public_key: Self::to_public_key_hex(&public_key)?,
            blockchain_client: None,
            network,
        })
    }
    fn get_public_address(&self) -> String {
        self.public_address.clone()
    }
}

impl BitcoinWallet {
    // Reference for address formats: https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#p2wpkh-nested-in-bip16-p2sh

    /// The P2PKH is the Legacy address format for Bitcoin
    pub fn public_address_p2pkh_from_public_key(public_key: &Vec<u8>) -> String {
        //p2pkh format
        let mut address = [0u8; 25];
        
        address[0] = 0x00;
        address[1..21].copy_from_slice(&BIP32::hash160(&public_key));

        let checksum =
                &(Sha256::digest(Sha256::digest(&address[0..21]).as_slice()).to_vec())[0..4];
        address[21..25].copy_from_slice(checksum);
        address.to_base58()
    }

    pub fn public_address_bech32_from_public_key(public_key: &Vec<u8>, network_type: &NetworkType) -> String {
        let mut data = Ripemd160::digest(&Sha256::digest(public_key).as_slice()).to_vec().to_base32();
        data.insert(0, bech32::u5::try_from_u8(0).unwrap());
        match network_type {
            NetworkType::MainNet => {
                let prefix = "bc";
                let address = bech32::encode(prefix, &data, bech32::Variant::Bech32).unwrap();
                address
            }
            NetworkType::TestNet => {
                let prefix = "tb";
                let address = bech32::encode(prefix, &data, bech32::Variant::Bech32).unwrap();
                address
            }
        }
    }
    pub fn rpc_create_wallet(
        client: &Client,
        wallet_path: &str,
    ) -> Result<bitcoincore_rpc::json::LoadWalletResult, String> {
        let created_wallet = client
            .create_wallet(wallet_path, Some(false), Some(false), Some(""), Some(false))
            .unwrap();
        Ok(created_wallet)
    }
    pub fn rpc_load_wallet(
        &self,
        wallet_path: &str,
    ) -> Result<bitcoincore_rpc::json::LoadWalletResult, String> {
        let loaded_wallet = self.blockchain_client.as_ref().expect("Error rpc load wallet").load_wallet(wallet_path).unwrap();
        Ok(loaded_wallet)
    }
    pub fn rpc_list_wallets(&self) -> Result<Vec<String>, String> {
        let listed_wallets = self.blockchain_client.as_ref().expect("Error rpc list wallets").list_wallets().unwrap();
        Ok(listed_wallets)
    }
    pub fn rpc_get_wallet_info(
        &self
    ) -> Result<bitcoincore_rpc::json::GetWalletInfoResult, String> {
        let wallet_info = self.blockchain_client.as_ref().expect("Error rpc get wallet info").get_wallet_info().unwrap();
        Ok(wallet_info)
    }
    pub fn rpc_unload_wallet(&self, wallet_path: &str) -> Result<(), String> {
        let unloaded = self.blockchain_client.as_ref().expect("Error rpc unload wallet").unload_wallet(Some(wallet_path)).unwrap();
        Ok(unloaded)
    }
}

impl Display for BitcoinWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Bitcoin Wallet")?;
        writeln!(f, " Network: {}", self.network)?;
        writeln!(f, " Private Key: {}", self.private_key)?;
        writeln!(f, " Public Key: {}", self.public_key)?;
        writeln!(f, " Address Format: {}", self.address_format.to_string())?;
        writeln!(f, " Public Address: {}", self. public_address)?;
        Ok(())
    }
}

pub struct BlockchainClient {
  pub blockchain_client: Option<Client>,
}

impl BlockchainClient {
pub fn new(url: &str) -> Result<Self, String> {
  let client = Client::new(url, Auth::UserPass(USER.to_string(), PASS.to_string())).unwrap();

  Ok(Self {
    blockchain_client: Some(client),
  })
}

pub fn get_block_count(&self) -> Result<u64, String> {
  let block_count = self.blockchain_client.as_ref().expect("Error getting block count").get_block_count().unwrap();
  Ok(block_count)
}

pub fn get_best_block_hash(&self) -> Result<BlockHash, String> {
  let best_block_hash = self.blockchain_client.as_ref().expect("Error getting best block hash").get_best_block_hash().unwrap();
  Ok(best_block_hash)
}

pub fn get_block_hash(&self, height: u64) -> Result<BlockHash, String> {
  let block_hash = self.blockchain_client.as_ref().expect("Error getting block hash").get_block_hash(height).unwrap();
  Ok(block_hash)
}

}