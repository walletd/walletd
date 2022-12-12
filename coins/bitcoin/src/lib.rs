extern crate bitcoincore_rpc;
extern crate reqwest;
use base58::ToBase58;
use bech32::ToBase32;
use ripemd::Ripemd160;
use core::{fmt, fmt::Display};
use bitcoincore_rpc::bitcoin::{Block, BlockHash, Txid, Transaction};
use bitcoincore_rpc::bitcoincore_rpc_json::GetBlockchainInfoResult;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use walletd_bip39::Mnemonic;
use walletd_coins::{CryptoCoin, CryptoWallet};
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
    network: NetworkType,
}

impl CryptoWallet for BitcoinWallet {
    type HDKeyInfo = BIP32;
    type MnemonicStyle = Mnemonic;
    type AddressFormat = BitcoinFormat;
    fn new_from_hd_keys(hd_keys: &BIP32, address_format: BitcoinFormat) -> Result<Self, String> {
        let public_key_bytes = &hd_keys.extended_public_key.unwrap().to_vec();
        let mut public_address: String;
        match address_format {
            BitcoinFormat::P2PKH => public_address = Self::public_address_p2pkh_from_public_key(public_key_bytes),
            BitcoinFormat::P2SH => public_address = Self::public_address_p2sh_from_public_key(public_key_bytes, &hd_keys.network),
            BitcoinFormat::Bech32 => public_address = Self::public_address_bech32_from_public_key(public_key_bytes, &hd_keys.network), 
        }

        Ok(Self {
            crypto_type: CryptoCoin::BTC,
            address_format,
            public_address,
            private_key: hd_keys.get_private_key_wif().unwrap(),
            public_key: hd_keys.get_public_key_hex().unwrap(),
            network: hd_keys.network,
        })
    }

    fn public_address(&self) -> &String {
        &self.public_address
    }
}

impl BitcoinWallet {

    pub fn public_address_p2pkh_from_public_key(public_key: &Vec<u8>) -> String {
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
    /* Commenting out old code used for initial demo
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
    */
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

pub struct Blockstream {
    pub client: reqwest::Client,
}

pub const BLOCKSTREAM_URL: &str = "https://blockstream.info/api";
impl Blockstream {
    pub fn new(url: &str) -> Result<Self, String> {
        Ok(Self {
            client: reqwest::Client::new(),
        })
    }

    // fetch the block height
    pub fn block_count(&self) -> Result<u64, String> {
      let body = reqwest::blocking::get(format!("{}/blocks/tip/height",BLOCKSTREAM_URL)).expect("Error getting block count").text();
        println!("body = {:?}", body);
        let block_count: u64 = body.unwrap().parse().unwrap();
      Ok(block_count)
    }

    // fetch fee estimates from blockstream
    pub fn fee_estimates(&self) -> Result<Value, String> {
      let body = reqwest::blocking::get(format!("{}/fee-estimates",BLOCKSTREAM_URL)).expect("Error getting fee estimates").text();
        println!("body = {:?}", body);
        //let data = r#"{"21":2.582,"1008":1.0,"12":14.197999999999999,"13":10.038,"10":21.026,"5":21.113999999999997,"9":21.026,"2":28.343999999999998,"14":2.885,"17":2.582,"20":2.582,"3":28.343999999999998,"22":2.582,"23":2.582,"24":2.582,"8":21.026,"1":28.343999999999998,"7":21.026,"11":14.197999999999999,"16":2.582,"25":2.582,"15":2.582,"19":2.582,"144":1.0,"504":1.0,"4":25.835,"18":2.582,"6":21.026}"#;
        let fee_estimates = json!(&body.unwrap());
      Ok(fee_estimates)
    }

    // fetch transactions from blockstream
    pub fn transactions(&self, address: &str) -> Result<Value, String> {
      let body = reqwest::blocking::get(format!("{}/address/{}/txs",BLOCKSTREAM_URL,address)).expect("Error getting transactions").text();
        println!("body = {:?}", body);
        let transactions = json!(&body.unwrap());
      Ok(transactions)
    }

    // fetch mempool transactions from blockstream
    pub fn mempool_transactions(&self, address: &str) -> Result<Value, String> {
      let body = reqwest::blocking::get(format!("{}/address/{}/txs/mempool",BLOCKSTREAM_URL,address)).expect("Error getting transactions").text();
        println!("body = {:?}", body);
        let transactions = json!(&body.unwrap());
      Ok(transactions)
    }

    // fetch utxo from blockstream
    pub fn utxo(&self, address: &str) -> Result<Value, String> {
      let body = reqwest::blocking::get(format!("{}/address/{}/utxo",BLOCKSTREAM_URL,address)).expect("Error getting utxo").text();
        println!("body = {:?}", body);
        let utxo = json!(&body.unwrap());
      Ok(utxo)
    }

    // Fetch transaction info
    pub fn transaction(&self, txid: &str) -> Result<BTransaction, String> {
      let body = reqwest::blocking::get(format!("{}/tx/{}",BLOCKSTREAM_URL,txid)).expect("Error getting transaction").text();
      let data = r#"{
        "txid":"6249b166d78529e435628245034df9e4c81d9b34b4d12c5600527c96b6e0d8ce",
        "version":1,
        "locktime":0,
        "vin":[
          {
            "txid":"4894c96e044bd6c278f927a220c42048602e4d8bfa888f5c35610b1c4643140d",
            "vout":1,
            "prevout":{
              "scriptpubkey":"a914f7861160df5cce001291293dfba24923816fc7e987",
              "scriptpubkey_asm":"OP_HASH160 OP_PUSHBYTES_20 f7861160df5cce001291293dfba24923816fc7e9 OP_EQUAL",
              "scriptpubkey_type":"p2sh",
              "scriptpubkey_address":"3QFoS8FPLCiVzzra4TPqVCq5ntpswP9Ey3",
              "value":48713312
            },
            "scriptsig":"160014630cf4b24dbd691fef2bb3fa50605484632f611e",
            "scriptsig_asm":"OP_PUSHBYTES_22 0014630cf4b24dbd691fef2bb3fa50605484632f611e",
            "witness":[
              "304402201e23c13611331720f5dfe2455b2d3c3b259d84cadc5e3de6e792a750978efeb8022006d3acad3c1c5b7e6227c80b71fee635f9303fd164b378c98a9fb3063105ff9201",
              "025e7a3239de2b1dbde8d8ff5c0c620ac47bfd32e761f509c13424fe8481dbb98e"
            ],
            "is_coinbase":false,
            "sequence":4294967295,
            "inner_redeemscript_asm":"OP_0 OP_PUSHBYTES_20 630cf4b24dbd691fef2bb3fa50605484632f611e"
          }
        ],
        "vout":[
          {
            "scriptpubkey":"a914b3efe280e64077202c171cc3fefb4bb02adc7d0687",
            "scriptpubkey_asm":"OP_HASH160 OP_PUSHBYTES_20 b3efe280e64077202c171cc3fefb4bb02adc7d06 OP_EQUAL",
            "scriptpubkey_type":"p2sh",
            "scriptpubkey_address":"3J6SFNJSHq9k6k2Cwzdy6RMC1z3ubR1ot1",
            "value":15632000
          },
          {
            "scriptpubkey":"a91445a3f3cc49da0b67c969771b0b8ef76c45aaff2787",
            "scriptpubkey_asm":"OP_HASH160 OP_PUSHBYTES_20 45a3f3cc49da0b67c969771b0b8ef76c45aaff27 OP_EQUAL",
            "scriptpubkey_type":"p2sh",
            "scriptpubkey_address":"383ExPThK2M5yZEtHXU1YcqehVBDxHKuWJ",
            "value":33065280
          }
        ],
        "size":247,
        "weight":661,
        "fee":16032,
        "status":{
          "confirmed":true,
          "block_height":663393,
          "block_hash":"0000000000000000000efbc1d707a0b95bc281c908ecf1f149d2d93ca8d6a175",
          "block_time":1609181106
        }
      }"#;
        println!("body = {:?}", body);
        let transaction: BTransaction = serde_json::from_str(&body.unwrap()).unwrap();
      Ok(transaction)
    }
}

#[derive(Serialize, Deserialize)]
pub struct BTransaction {
    txid: String,
    version: u8,
    locktime: u32,
    vin: Vec<Input>,
    vout: Vec<Output>,
    size: u32,
    weight: u32,
    fee: u32,
    status: Status,
}

#[derive(Serialize, Deserialize)]
struct Output {
  scriptpubkey: String,
  scriptpubkey_asm: String,
  scriptpubkey_type: String,
  scriptpubkey_address: String,
  value: u32,
}

#[derive(Serialize, Deserialize)]
struct Input {
  txid: String,
  vout: u8,
  prevout: Output,
  scriptsig: String,
  scriptsig_asm: String,
  witness: Vec<String>,
  is_coinbase: bool,
  sequence: u32,
  inner_redeemscript_asm: String,
}

#[derive(Serialize, Deserialize)]
struct Status {
  confirmed: bool,
  block_height: u32,
  block_hash: String,
  block_time: u32,
}