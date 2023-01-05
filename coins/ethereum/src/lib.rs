// Mainnet Infura: https://celo-mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// Ropsten Infura: https://ropsten.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// Localhost Ganache: http://localhost:8545 by default
use hex::encode;
use web3::transports::Http;
use web3::Web3;
use tiny_keccak::{Hasher, Keccak};
use libsecp256k1::{PublicKey, SecretKey};

use walletd_coins::{CryptoCoin,CryptoWallet};
use walletd_bip39::{Language, Mnemonic, MnemonicType, MnemonicHandler};
use walletd_hd_keys::{BIP32, NetworkType};

use core::{fmt, fmt::Display, str::FromStr};

// run ganache-cli
pub const URL: &str = "http://localhost:8545";

#[derive(Default)]
pub enum EthereumFormat {
    #[default]
    Checksummed,
    NonChecksummed,
}

impl EthereumFormat {
    pub fn to_string(&self) -> String {
        match self {
            EthereumFormat::Checksummed => "Checksummed".to_string(),
            EthereumFormat::NonChecksummed => "NonChecksummed".to_string(),
        }
    }
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
    type MnemonicStyle = Mnemonic;
    type HDKeyInfo  = BIP32;
    type AddressFormat = EthereumFormat;

    fn new_from_hd_keys(hd_keys: &BIP32, address_format: EthereumFormat) -> Result<Self, String> {
        let public_key_bytes = &hd_keys.extended_public_key.unwrap().to_vec();
        let mut public_address: String;
        match address_format {
            EthereumFormat::Checksummed => public_address = Self::public_address_checksummed_from_public_key(public_key_bytes),
            EthereumFormat::NonChecksummed => public_address = Self::public_address_nonchecksummed_from_public_key(public_key_bytes), 
        }
        Ok(Self {
            crypto_type: CryptoCoin::ETH,
            address_format,
            public_address,
            private_key: hd_keys.get_private_key_0x().unwrap(),
            public_key: hd_keys.get_public_key_0x().unwrap(),
            blockchain_client: None,
            network: hd_keys.network,
        })
    }

    fn public_address(&self) -> &String {
        &self.public_address
    }
}

impl EthereumWallet {
    pub fn public_address_checksummed_from_public_key(public_key: &Vec<u8>) -> String {
        let public_key_full = PublicKey::parse_slice(&public_key, Some(libsecp256k1::PublicKeyFormat::Compressed)).unwrap();
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(&public_key_full.serialize()[1..]);
        hasher.finalize(&mut output);
        let address = hex::encode(&output[12..]).to_lowercase();

        let mut checksum_address = String::new();
        let mut digest_out2 = [0u8; 32];
        let mut hasher2 = Keccak::v256();
        let address_bytes = address.as_bytes();
        hasher2.update(&address_bytes);
        hasher2.finalize(&mut digest_out2);
        let keccak_digest_hex = hex::encode(digest_out2);

        for (i, address_char) in address.chars().enumerate() {
            let keccak_char= &keccak_digest_hex[i..i+1];
            if u8::from_str_radix(&keccak_char[..], 16).unwrap() >= 8 {
                checksum_address.push(address_char.to_ascii_uppercase());
            }
            else {
                checksum_address.push(address_char);
            }
        }
        checksum_address = format!("{}{}", "0x", checksum_address);
        checksum_address
    }

    pub fn public_address_nonchecksummed_from_public_key(public_key: &Vec<u8>) -> String {
        let public_key_full = PublicKey::parse_slice(&public_key, Some(libsecp256k1::PublicKeyFormat::Compressed)).unwrap();
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(&public_key_full.serialize()[1..]);
        hasher.finalize(&mut output);
        let mut address = hex::encode(&output[12..]).to_lowercase();
        address = format!("{}{}", "0x", address);
        address
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

impl Display for EthereumWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Ethereum Wallet")?;
        writeln!(f, " Network: {}", self.network)?;
        writeln!(f, " Private Key: {}", self.private_key)?;
        writeln!(f, " Public Key: {}", self.public_key)?;
        writeln!(f, " Address Format: {}", self.address_format.to_string())?;
        writeln!(f, " Public Address: {}", self. public_address)?;
        Ok(())
    }
}

pub struct BlockchainClient {
  blockchain_client: Web3<Http>,
}

impl BlockchainClient {
    pub fn new(url: &str) -> Result<Self, String> {
    let transport = web3::transports::Http::new(url).unwrap();
    let web3 = web3::Web3::new(transport);

    Ok(Self {
        blockchain_client: web3,
    })
    }
}
