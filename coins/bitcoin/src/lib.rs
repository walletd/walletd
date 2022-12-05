extern crate bitcoincore_rpc;
extern crate reqwest;
use walletd_coins::{CryptoCoin, CryptoTypeData};
use walletd_cryptowallet::CryptoWallet;
use walletd_bip39::{Language, Mnemonic, MnemonicType, MnemonicHandler};
use walletd_hd_keys::{BIP32, NetworkType};
use bitcoincore_rpc::bitcoin::BlockHash;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use sha2::{Digest, Sha256, Sha512};
use base58::{FromBase58, ToBase58};
use base32;

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

impl CryptoTypeData for BitcoinWallet {
    fn print_public_address(&self) -> () {
        println!("Public address: {}", self.public_address);
    }
}

impl CryptoWallet for BitcoinWallet {
    fn new_from_hd_keys(hd_keys: BIP32) -> Result<Self, String> {
        Ok(Self {
            crypto_type: CryptoCoin::BTC,
            address_format: BitcoinFormat::Bech32,
            public_address: Self::public_address_bech32_from_public_key(&hd_keys.extended_public_key.unwrap().to_vec(), &hd_keys.network),
            private_key: hd_keys.get_private_key().unwrap(),
            public_key: hd_keys.get_public_key().unwrap(),
            blockchain_client: None,
            network: hd_keys.network,
        })
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

    pub fn public_address_p2sh_p2wpkh_from_public_key(public_key: &Vec<u8>, network_type: NetworkType) -> String {
        let redeem_script = Self::create_redeem_script(public_key);
        let mut address = [0u8; 25];
        // prefix
        match network_type {
            NetworkType::MainNet => {
                address[0] = 0x05;
            }
            NetworkType::TestNet => {
                address[0] = 0xC4;
            }
        }
        address[1..21].copy_from_slice(&BIP32::hash160(&Self::create_redeem_script(public_key)));

        let mut checksum = Sha256::digest(&Sha256::digest(&address[0..21]).to_vec())[0..4].to_vec();
        address[21..25].copy_from_slice(checksum.as_slice());
        address.to_base58()

    }

    pub fn public_address_bech32_from_public_key(public_key: &Vec<u8>, network_type: &NetworkType) -> String {
        let redeem_script = Self::create_redeem_script(public_key);
        let mut data = vec![redeem_script[0]];
        data.extend_from_slice(&redeem_script[2..].to_vec());
        
        match network_type {
            NetworkType::MainNet => {
                let prefix = vec![0x62, 0x63];
                return base32::encode(base32::Alphabet::Crockford, &data.splice(0..2, prefix).collect::<Vec<u8>>().as_slice())
            }
            NetworkType::TestNet => {
                let prefix = vec![0x74, 0x62];
                return base32::encode(base32::Alphabet::Crockford, &data.splice(0..2, prefix).collect::<Vec<u8>>().as_slice())
            }
        }
    }

    /// Returns a redeem script for a given Bitcoin public key.
    fn create_redeem_script(public_key: &Vec<u8>) -> [u8; 22] {
        let mut redeem = [0u8; 22];
        redeem[1] = 0x14;
        redeem[2..].copy_from_slice(&BIP32::hash160(&public_key));
        redeem
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