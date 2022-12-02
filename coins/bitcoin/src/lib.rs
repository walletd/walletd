extern crate bitcoincore_rpc;
extern crate reqwest;
use base58::{FromBase58, ToBase58};
use bech32::ToBase32;
use ripemd::Ripemd160;
use core::{fmt, fmt::Display, str::FromStr};
use libsecp256k1::{PublicKey, SecretKey};
use bitcoincore_rpc::bitcoin::{Block, BlockHash, Txid, Transaction};
use bitcoincore_rpc::bitcoincore_rpc_json::GetBlockchainInfoResult;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use sha2::{Digest, Sha256, Sha512};
use walletd_bip39::{Language, Mnemonic, MnemonicHandler, MnemonicType};
use walletd_coins::{CryptoCoin, CryptoTypeData, CryptoWallet};
use walletd_hd_keys::{BIP32,NetworkType};

pub const USER: &str = "test";
pub const PASS: &str = "test";

#[derive(Default)]
pub enum BitcoinFormat {
    P2PKH,
    P2SH,
    #[default]
    Bech32,
}
impl BitcoinFormat {
    pub fn to_string(&self) -> String {
        match self {
            BitcoinFormat::P2PKH => "Legacy (P2PKH) Address".to_string(),
            BitcoinFormat::P2SH => "SegWit Address (P2SH Script Function)".to_string(),
            BitcoinFormat::Bech32 => "Native SegWit Address (P2WPKH) Bech32".to_string(), 
        }
    }
}

pub struct BitcoinAddress {
    address: String,
    format: BitcoinFormat,
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
    seed_hex: Option<String>,
}

impl CryptoTypeData for BitcoinWallet {
    fn print_public_address(&self) -> () {
        println!("Public address: {}", self.public_address);
    }
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

    fn public_address(&self) -> &String {
        &self.public_address
    }
}

impl BitcoinWallet {

    pub fn public_address_p2pkh_from_public_key(public_key: Vec<u8>) -> String {
        //p2pkh format
        let mut address = [0u8; 25];

        address[0] = 0x00;
        address[1..21].copy_from_slice(&BIP32::hash160(&public_key));

        let checksum = &(Sha256::digest(Sha256::digest(&address[0..21]).as_slice()).to_vec())[0..4];
        address[21..25].copy_from_slice(checksum);
        return address.to_base58()
    }

    fn public_address_p2sh_from_public_key(public_key: &Vec<u8>, network_type: &NetworkType) -> String {
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

    fn public_address_bech32_from_public_key(public_key: &Vec<u8>, network_type: &NetworkType) -> String {
        let mut data = Ripemd160::digest(&Sha256::digest(public_key).as_slice()).to_vec().to_base32();
        data.insert(0, bech32::u5::try_from_u8(0).unwrap()); // version 0
        match network_type {
            NetworkType::MainNet => {
                let prefix = "bc";
                let address = bech32::encode(prefix, &data, bech32::Variant::Bech32).unwrap();
                return address
            }
            NetworkType::TestNet => {
                let prefix = "tb";
                let address = bech32::encode(prefix, &data, bech32::Variant::Bech32).unwrap();
                return address
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
    // this will need to be moved to a rpc wallet transport layer.
    // pub fn rpc_create_wallet(
    //     client: &Client,
    //     wallet_path: &str,
    // ) -> Result<bitcoincore_rpc::json::LoadWalletResult, String> {
    //     let created_wallet = client
    //         .create_wallet(wallet_path, Some(false), Some(false), Some(""), Some(false))
    //         .unwrap();
    //     Ok(created_wallet)
    // }
    // pub fn rpc_load_wallet(
    //     &self,
    //     wallet_path: &str,
    // ) -> Result<bitcoincore_rpc::json::LoadWalletResult, String> {
    //     let loaded_wallet = self.blockchain_client.as_ref().expect("Error rpc load wallet").load_wallet(wallet_path).unwrap();
    //     Ok(loaded_wallet)
    // }
    // pub fn rpc_list_wallets(&self) -> Result<Vec<String>, String> {
    //     let listed_wallets = self.blockchain_client.as_ref().expect("Error rpc list wallets").list_wallets().unwrap();
    //     Ok(listed_wallets)
    // }
    // pub fn rpc_get_wallet_info(
    //     &self
    // ) -> Result<bitcoincore_rpc::json::GetWalletInfoResult, String> {
    //     let wallet_info = self.blockchain_client.as_ref().expect("Error rpc get wallet info").get_wallet_info().unwrap();
    //     Ok(wallet_info)
    // }
    // pub fn rpc_unload_wallet(&self, wallet_path: &str) -> Result<(), String> {
    //     let unloaded = self.blockchain_client.as_ref().expect("Error rpc unload wallet").unload_wallet(Some(wallet_path)).unwrap();
    //     Ok(unloaded)
    // }
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
    pub blockchain_client: Client,
}

impl BlockchainClient {
    pub fn new(url: &str) -> Result<Self, String> {
        let client = Client::new(url, Auth::UserPass(USER.to_string(), PASS.to_string())).unwrap();

        Ok(Self {
            blockchain_client: client,
        })
    }
    
    // use get_block_hash to get the block hash from the block height
    pub fn get_block(&self, hash: &BlockHash) -> Result<Block, String> {
      Ok(self.blockchain_client.get_block(hash).unwrap())
    }

    pub fn get_block_count(&self) -> Result<u64, String> {
        Ok(self.blockchain_client.get_block_count().unwrap())
    }

    pub fn get_best_block_hash(&self) -> Result<BlockHash, String> {
        Ok(self.blockchain_client.get_best_block_hash().unwrap())
    }

    pub fn get_block_hash(&self, height: u64) -> Result<BlockHash, String> {
        Ok(self.blockchain_client.get_block_hash(height).unwrap())
    }

    pub fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult, String> {
      Ok(self.blockchain_client.get_blockchain_info().unwrap())
    }

    pub fn get_raw_mempool(&self) -> Result<Vec<Txid>, String> {
      Ok(self.blockchain_client.get_raw_mempool().unwrap())
    }

    pub fn get_raw_transaction(&self, txid: &Txid) -> Result<Transaction, String> {
      Ok(self.blockchain_client.get_raw_transaction(txid, None).unwrap())
    }

    // pub fn get_transaction(&self, txid: &Txid) -> Result<GetTransactionResult, String> {
    //   Ok(self.blockchain_client.get_transaction(txid, None).unwrap())
    // }

    // pub fn get_tx_out(&self, txid: &Txid, vout: u32) -> Result<GetTxOutResult, String> {
    //   Ok(self.blockchain_client.get_tx_out(txid, vout, None).unwrap())
    // }
}
