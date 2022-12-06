
use hex::encode;
use keccak_hash::{H256, keccak256};
use web3::transports::Http;
use web3::Web3;

use walletd_coins::CryptoCoin;
use walletd_bip39::{Language, Mnemonic, MnemonicType, MnemonicHandler};
use walletd_hd_keys::{BIP32, NetworkType};
use walletd_cryptowallet::CryptoWallet;

// run ganache-cli
pub const URL: &str = "http://localhost:8545";

#[derive(Default)]
pub enum EthereumFormat {
    #[default]
    Standard,
}

#[derive(Default)]
pub struct EthereumWallet {
    crypto_type: CryptoCoin,
    address_format: EthereumFormat,
    public_address: String,
    blockchain_client: Option<web3::transports::Http>,
    private_key: String,
    public_key: String,
    network: NetworkType,
}

impl CryptoWallet for EthereumWallet {
    fn new_from_hd_keys(hd_keys: BIP32) -> Result<Self, String> {
        Ok(Self {
            crypto_type: CryptoCoin::ETH,
            address_format: EthereumFormat::Standard,
            public_address: Self::public_address_from_public_key(&hd_keys.extended_public_key.unwrap().to_vec()),
            private_key: hd_keys.get_private_key().unwrap(),
            public_key: hd_keys.get_public_key().unwrap(),
            blockchain_client: None,
            network: hd_keys.network,
        })
    }
    fn get_public_address(&self) -> String {
        self.public_address.clone()
    }
}

impl EthereumWallet {
    pub fn public_address_from_public_key(public_key: &Vec<u8>) -> String {
        let hash1: keccak_hash::H256 = keccak_hash::keccak(&public_key[1..]);
        let address = hex::encode(&hash1.as_bytes()[12..]).to_lowercase();
        let hash2 = hex::encode(&keccak_hash::keccak(hex::decode(&address).unwrap()));
        let mut checksum_address = "0x".to_string();
        for c in 0..40 {
            let ch = match &hash2[c..=c] {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" => address[c..=c].to_lowercase(),
                _ => address[c..=c].to_uppercase(),
            };
            checksum_address.push_str(&ch);
        }
        checksum_address

    }
    #[tokio::main]
    pub async fn main() -> web3::Result<()> {
        let transport = web3::transports::Http::new(URL)?;
        let web3 = web3::Web3::new(transport);

        println!("Calling accounts.");
        let mut accounts = web3.eth().accounts().await?;
        println!("Accounts: {:?}", accounts);
        accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

        println!("Calling balance.");
        for account in accounts {
            let balance = web3.eth().balance(account, None).await?;
            println!("Balance of {:?}: {}", account, balance);
        }

        Ok(())
    }
}

pub struct BlockchainClient {
  blockchain_client: Option<Web3<Http>>,
}

impl BlockchainClient {
pub fn new(url: &str) -> Result<Self, String> {
  let transport = web3::transports::Http::new(url).unwrap();
  let web3 = web3::Web3::new(transport);

  Ok(Self {
    blockchain_client: Some(web3),
  })
}
}
