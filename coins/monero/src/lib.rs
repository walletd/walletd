extern crate reqwest;

use walletd_coins::{CryptoCoin, CryptoTypeData};
use walletd_cryptowallet::{CryptoWallet};
use walletd_monero_mnemonic::{Language, Mnemonic, MnemonicType, MnemonicHandler};
use walletd_hd_keys::{BIP32, NetworkType};

use reqwest::header::{ACCEPT, CONTENT_TYPE};
use std::collections::HashMap;

// example running monero private testnet, https://github.com/moneroexamples/private-testnet
const URL: &str = "http://localhost:28081/json_rpc";

#[derive(Default)]
pub enum MoneroFormat {
    /// Standard address
    #[default]
    Standard,
    /// Address with payment id (8 bytes)
    Integrated([u8; 8]),
    /// Subaddress
    Subaddress(u32, u32),
}

#[derive(Default)]
pub struct MoneroWallet {
    crypto_type: CryptoCoin,
    address_format: MoneroFormat,
    network: NetworkType,
    public_address: String,
    private_key: String,
    public_key: String,
    blockchain_client: Option<reqwest::Client>,
}

impl CryptoWallet for MoneroWallet {
    fn new_from_hd_keys(hd_keys: BIP32) -> Result<Self, String> {
        Ok(Self {
            crypto_type: CryptoCoin::XMR,
            address_format: MoneroFormat::Standard,
            public_address: Self::public_address_from_public_key(&hd_keys.extended_public_key.unwrap().to_vec(), &hd_keys.network),
            private_key: hd_keys.get_private_key().unwrap(),
            public_key: hd_keys.get_public_key().unwrap(),
            blockchain_client: None,
            network: hd_keys.network,
        })
    }
}

impl MoneroWallet {
    pub fn public_address_from_public_key(public_key: &Vec<u8>, network_type: &NetworkType) -> String {
        "Temp".to_string()
    }
    #[tokio::main]
    pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        // let mut headers = HeaderMap::new();
        let mut map = HashMap::new();
        map.insert("jsonrpc", "2.0");
        map.insert("id", "0");
        map.insert("method", "getblockcount");
        let client = reqwest::Client::new();
        let response = client
            .post(URL)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .json(&map)
            .send()
            .await?
            .text()
            .await?;

        println!("{:#?}", client);
        println!("{:#?}", response);
        Ok(())
    }
}

pub struct BlockchainClient {
  blockchain_client: Option<reqwest::Client>,
}

impl BlockchainClient {
pub fn new(url: &str) -> Result<Self, String> {
  Ok(Self {
    blockchain_client: Some(reqwest::Client::new()),
  })
}
}