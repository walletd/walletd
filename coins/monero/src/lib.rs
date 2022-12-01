extern crate reqwest;

use walletd_coins::{CryptoCoin, CryptoTypeData, CryptoWallet};
use walletd_monero_mnemonic::{Language, Mnemonic, MnemonicType, MnemonicHandler};
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
    language: Language,
    network: String,
    public_address: String,
    seed_hex: Option<String>,
    private_spend_key: String,
    private_view_key: String,
    public_spend_key: String,
    public_view_key: String,
    blockchain_client: Option<reqwest::Client>,
}

impl CryptoWallet for MoneroWallet {
    fn new() -> Result<Self, String> {
        let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words25, None);
        let seed = mnemonic.get_seed()?;
        println!("Mnemonic Info: \n{}", mnemonic);
        Ok(Self {
            seed_hex: Some(seed),
            ..Default::default()
        })
    }
    fn create_wallet() -> Result<Self, String> {
        let created_wallet = MoneroWallet::new().unwrap();
        Ok(created_wallet)
    }
}

impl MoneroWallet {
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