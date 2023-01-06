use ::secp256k1::{PublicKey, Secp256k1, SecretKey};
use anyhow::anyhow;
use async_trait::async_trait;
use core::{fmt, fmt::Display};
use std::any::Any;
use tiny_keccak::{Hasher, Keccak};
use web3::transports::Http;
use web3::Web3;

use walletd_bip39::Seed;
use walletd_coins::{CryptoCoin, CryptoWallet, CryptoWalletGeneral};
use walletd_hd_keypairs::{HDKeyPair, NetworkType};

// run ganache-cli
pub const URL: &str = "http://localhost:8545";

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub struct EthereumWallet {
    crypto_type: CryptoCoin,
    address_format: EthereumFormat,
    public_address: String,
    private_key: String,
    public_key: String,
    network: NetworkType,
}

pub struct EthereumAmount {
    pub wei: u32,
}

impl EthereumAmount {
    pub fn ETH(&self) -> f64 {
        (self.wei as f64) / (i32::pow(10, 18) as f64) // 1 ETH = 10^-18 wei
    }
}

#[async_trait]
impl CryptoWallet for EthereumWallet {
    type MnemonicSeed = Seed;
    type HDKeyInfo = HDKeyPair;
    type AddressFormat = EthereumFormat;
    type CryptoAmount = EthereumAmount;
    type BlockchainClient = web3::transports::Http;
    type NetworkType = NetworkType;

    fn new_from_hd_keys(
        hd_keys: &HDKeyPair,
        address_format: EthereumFormat,
    ) -> Result<Self, anyhow::Error> {
        let public_key_bytes = &hd_keys
            .extended_public_key
            .expect("extended public key data not available")
            .to_vec();
        let mut public_address: String;
        match address_format {
            EthereumFormat::Checksummed => {
                public_address = Self::public_address_checksummed_from_public_key(public_key_bytes)?
            }
            EthereumFormat::NonChecksummed => {
                public_address =
                    Self::public_address_nonchecksummed_from_public_key(public_key_bytes)?
            }
        }
        Ok(Self {
            crypto_type: CryptoCoin::ETH,
            address_format,
            public_address,
            private_key: hd_keys.get_private_key_0x()?,
            public_key: hd_keys.get_public_key_0x()?,
            network: hd_keys.network,
        })
    }

    fn new_from_non_hd_mnemonic_seed(
        mnemonic_seed: &Seed,
        network_type: NetworkType,
        address_format: EthereumFormat,
    ) -> Result<Self, anyhow::Error> {
        let seed_bytes = mnemonic_seed.as_bytes();
        let mut private_key_bytes = [0u8; 32];
        private_key_bytes.copy_from_slice(&seed_bytes[0..32]);
        let public_key_bytes = PublicKey::from_secret_key(
            &Secp256k1::new(),
            &SecretKey::from_slice(&private_key_bytes)?,
        )
        .serialize()
        .to_vec();
        let public_address: String;
        match address_format {
            EthereumFormat::Checksummed => {
                public_address =
                    Self::public_address_checksummed_from_public_key(&public_key_bytes)?
            }
            EthereumFormat::NonChecksummed => {
                public_address =
                    Self::public_address_nonchecksummed_from_public_key(&public_key_bytes)?
            }
        }
        Ok(Self {
            crypto_type: CryptoCoin::ETH,
            address_format,
            public_address,
            private_key: Self::to_0x_hex_format(&private_key_bytes)?,
            public_key: Self::to_0x_hex_format(&public_key_bytes)?,
            network: network_type,
        })
    }

    fn public_address(&self) -> String {
        self.public_address.clone()
    }

    async fn confirmed_balance(
        &self,
        blockchain_client: &Self::BlockchainClient,
    ) -> Result<Self::CryptoAmount, anyhow::Error> {
        Err(anyhow!(
            "Current balance not currently implemented for Ethereum"
        ))
    }
    async fn transfer(
        &self,
        client: &Self::BlockchainClient,
        send_amount: &Self::CryptoAmount,
        public_address: &str,
    ) -> Result<(), anyhow::Error> {
        Err(anyhow!(
            "Transfer functionality not currently implemented for Ethereum"
        ))
    }
}

impl EthereumWallet {
    pub fn public_address_checksummed_from_public_key(
        public_key: &Vec<u8>,
    ) -> Result<String, anyhow::Error> {
        let public_key_full = PublicKey::from_slice(&public_key)?;
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
            let keccak_char = &keccak_digest_hex[i..i + 1];
            if u8::from_str_radix(&keccak_char[..], 16)? >= 8 {
                checksum_address.push(address_char.to_ascii_uppercase());
            } else {
                checksum_address.push(address_char);
            }
        }
        checksum_address = format!("{}{}", "0x", checksum_address);
        Ok(checksum_address)
    }

    pub fn public_address_nonchecksummed_from_public_key(
        public_key: &Vec<u8>,
    ) -> Result<String, anyhow::Error> {
        let public_key_full = PublicKey::from_slice(&public_key)?;
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(&public_key_full.serialize()[1..]);
        hasher.finalize(&mut output);
        let mut address = hex::encode(&output[12..]).to_lowercase();
        address = format!("{}{}", "0x", address);
        Ok(address)
    }

    #[tokio::main]
    pub async fn main() -> Result<(), anyhow::Error> {
        let transport = web3::transports::Http::new(URL)?;
        let web3 = web3::Web3::new(transport);

        println!("Calling accounts.");
        let mut accounts = web3.eth().accounts().await?;
        println!("Accounts: {:?}", accounts);
        accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse()?);

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
        writeln!(f, " Public Address: {}", self.public_address)?;
        Ok(())
    }
}

impl CryptoWalletGeneral for EthereumWallet {
    fn crypto_type(&self) -> CryptoCoin {
        self.crypto_type
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BlockchainClient {
    blockchain_client: Web3<Http>,
}

impl BlockchainClient {
    pub fn new(url: &str) -> Result<Self, anyhow::Error> {
        let transport = web3::transports::Http::new(url)?;
        let web3 = web3::Web3::new(transport);

        Ok(Self {
            blockchain_client: web3,
        })
    }
}
