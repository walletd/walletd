extern crate reqwest;

use walletd_coins::{CryptoCoin, CryptoWallet, CryptoWalletGeneral};
use walletd_hd_keypairs::{HDKeyPair, NetworkType};
use walletd_monero_mnemonic::{Mnemonic, MnemonicHandler, Seed};

use anyhow::anyhow;
use async_trait::async_trait;
use core::{fmt, fmt::Display};
use curve25519_dalek::scalar::Scalar;
use hmac::{Hmac, Mac};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use sha2::Sha512;
use std::any::Any;
use std::collections::HashMap;
type HmacSha512 = Hmac<Sha512>;
use base58_monero as base58;
use curve25519_dalek::{constants::ED25519_BASEPOINT_TABLE, edwards::EdwardsBasepointTable};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};

// example running monero private testnet, https://github.com/moneroexamples/private-testnet
const URL: &str = "http://localhost:28081/json_rpc";

#[derive(Default, Debug)]
pub enum MoneroFormat {
    /// Standard address
    #[default]
    Standard,
    /// Address with payment id (8 bytes)
    Integrated([u8; 8]),
    /// Subaddress
    Subaddress(u32, u32),
}

impl MoneroFormat {
    pub fn to_string(&self) -> String {
        match self {
            MoneroFormat::Standard => "Standard".to_string(),
            MoneroFormat::Integrated(_) => "Integrated".to_string(),
            MoneroFormat::Subaddress(_, _) => "Subaddress".to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub struct MoneroWallet {
    crypto_type: CryptoCoin,
    address_format: MoneroFormat,
    network: NetworkType,
    public_address: String,
    private_spend_key: String,
    private_view_key: String,
    public_spend_key: String,
    public_view_key: String,
}
pub struct MoneroAmount {
    piconero: u32,
}

impl MoneroAmount {
    #[allow(non_snake_case)]
    pub fn XMR(&self) -> f64 {
        (self.piconero as f64) / (u32::pow(10, 12) as f64)
    }
}

#[async_trait]
impl CryptoWallet for MoneroWallet {
    type MnemonicSeed = Seed;
    type HDKeyInfo = HDKeyPair;
    type AddressFormat = MoneroFormat;
    type CryptoAmount = MoneroAmount;
    type BlockchainClient = reqwest::Client;
    type NetworkType = NetworkType;

    fn new_from_hd_keys(
        hd_keys: &HDKeyPair,
        address_format: Self::AddressFormat,
    ) -> Result<Self, anyhow::Error> {
        // uses BIP85 specification, https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki
        let mut entropy = HmacSha512::new_from_slice(b"bip-entropy-from-k")?;
        entropy.update(
            &hd_keys
                .extended_private_key
                .expect("extended private key data missing"),
        );

        // Monero uses 256 bits for the seed, 32 bytes
        let seed = &entropy.finalize().into_bytes()[..32];
        println!("seed hex {}", hex::encode(seed));
        let private_spend_key = Self::private_spend_key_from_seed(
            seed.try_into().expect("Slice with incorrect length"),
        )?;
        let private_view_key = Self::private_view_key_from_private_spend_key(&private_spend_key)?;
        let public_spend_key = Self::public_spend_key_from_private_spend_key(&private_spend_key)?;
        let public_view_key = Self::public_view_key_from_private_view_key(&private_view_key)?;
        let public_address: String;
        match address_format {
            MoneroFormat::Standard => {
                public_address = Self::public_standard_address_from_public_keys(
                    &public_spend_key,
                    &public_view_key,
                    &hd_keys.network,
                )?
            }
            _ => {
                return Err(anyhow!(
                    "Monero address functionality not currently set up for this address type"
                ))
            }
        }

        Ok(Self {
            crypto_type: CryptoCoin::XMR,
            address_format: MoneroFormat::Standard,
            private_spend_key,
            private_view_key,
            public_spend_key,
            public_view_key,
            public_address,
            network: hd_keys.network,
        })
    }

    fn new_from_non_hd_mnemonic_seed(
        mnemonic_seed: &Seed,
        network: NetworkType,
        address_format: MoneroFormat,
    ) -> Result<Self, anyhow::Error> {
        let seed = mnemonic_seed.as_bytes();
        let private_spend_key = Self::private_spend_key_from_seed(
            seed.try_into().expect("Slice with incorrect length"),
        )?;
        let private_view_key = Self::private_view_key_from_private_spend_key(&private_spend_key)?;
        let public_spend_key = Self::public_spend_key_from_private_spend_key(&private_spend_key)?;
        let public_view_key = Self::public_view_key_from_private_view_key(&private_view_key)?;
        let public_address: String;
        match address_format {
            MoneroFormat::Standard => {
                public_address = Self::public_standard_address_from_public_keys(
                    &public_spend_key,
                    &public_view_key,
                    &network,
                )?
            }
            _ => {
                return Err(anyhow!(
                    "Monero address functionality not currently set up for this address type"
                ))
            }
        }

        Ok(Self {
            crypto_type: CryptoCoin::XMR,
            address_format: MoneroFormat::Standard,
            private_spend_key,
            private_view_key,
            public_spend_key,
            public_view_key,
            public_address,
            network,
        })
    }

    fn public_address(&self) -> String {
        self.public_address.clone()
    }

    async fn confirmed_balance(
        &self,
        _blockchain_client: &Self::BlockchainClient,
    ) -> Result<Self::CryptoAmount, anyhow::Error> {
        Err(anyhow!(
            "Current balance is not currently implemented for Monero"
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

impl MoneroWallet {
    fn new_from_mnemonic(mnemonic: Mnemonic) -> Result<Self, anyhow::Error> {
        let seed = mnemonic.to_seed();
        let seed_bytes = seed.as_bytes();
        let public_key =
            PublicKey::from_secret_key(&Secp256k1::new(), &SecretKey::from_slice(&seed_bytes)?)
                .serialize();
        let network = NetworkType::MainNet;
        let private_spend_key = Self::private_spend_key_from_seed(seed_bytes.try_into()?)?;
        let private_view_key = Self::private_view_key_from_private_spend_key(&private_spend_key)?;
        let public_spend_key = Self::public_spend_key_from_private_spend_key(&private_spend_key)?;
        let public_view_key = Self::public_view_key_from_private_view_key(&private_view_key)?;
        let public_address = Self::public_standard_address_from_public_keys(
            &public_spend_key,
            &public_view_key,
            &network,
        )?;

        Ok(Self {
            crypto_type: CryptoCoin::XMR,
            address_format: MoneroFormat::Standard,
            private_spend_key,
            private_view_key,
            public_spend_key,
            public_view_key,
            public_address,
            network: network,
        })
    }
    pub fn public_standard_address_from_public_keys(
        public_spend_key: &String,
        public_view_key: &String,
        network_type: &NetworkType,
    ) -> Result<String, anyhow::Error> {
        let mut data: Vec<u8> = Vec::new();
        match network_type {
            NetworkType::MainNet => {
                data.push(18);
            }
            NetworkType::TestNet => {
                data.push(53);
            }
        }
        data.extend(hex::decode(public_spend_key)?);
        data.extend(hex::decode(public_view_key)?);
        let checksum_bytes: &[u8] = &data[0..65];
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(checksum_bytes);
        hasher.finalize(&mut output);

        let checksum = &output[..4];
        data.extend(checksum.to_vec());
        Ok(base58::encode(&data.as_slice())?)
    }

    pub fn public_view_key_from_private_view_key(
        private_view_key: &String,
    ) -> Result<String, anyhow::Error> {
        const G: &EdwardsBasepointTable = &ED25519_BASEPOINT_TABLE;
        let private_view = &Scalar::from_bits(
            hex::decode(private_view_key)?
                .as_slice()
                .try_into()
                .expect("Slice length is incorrect"),
        );
        let public_view = private_view * G;
        Ok(hex::encode(&public_view.compress().as_bytes()))
    }

    pub fn public_spend_key_from_private_spend_key(
        private_spend_key: &String,
    ) -> Result<String, anyhow::Error> {
        const G: &EdwardsBasepointTable = &ED25519_BASEPOINT_TABLE;
        let private_spend = &Scalar::from_bits(
            hex::decode(private_spend_key)?
                .as_slice()
                .try_into()
                .expect("Slice length is incorrect"),
        );
        let public_spend = private_spend * G;
        Ok(hex::encode(&public_spend.compress().as_bytes()))
    }
    pub fn private_view_key_from_private_spend_key(
        private_spend_key: &String,
    ) -> Result<String, anyhow::Error> {
        let private_spend_key_bytes = hex::decode(&private_spend_key)?;
        let mut hasher = Keccak::v256();
        let mut output = [0u8; 32];
        hasher.update(private_spend_key_bytes.as_slice());
        hasher.finalize(&mut output);
        let private_view_key = Scalar::from_bytes_mod_order(output).to_bytes();
        Ok(hex::encode(&private_view_key))
    }

    pub fn private_spend_key_from_seed(seed: [u8; 32]) -> Result<String, anyhow::Error> {
        let private_spend_key = Scalar::from_bytes_mod_order(seed).to_bytes();
        Ok(hex::encode(private_spend_key))
    }

    #[tokio::main]
    pub async fn main() -> Result<(), anyhow::Error> {
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

impl Display for MoneroWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Monero Wallet")?;
        writeln!(f, " Network: {}", self.network)?;
        writeln!(f, " Private Spend Key: {}", self.private_spend_key)?;
        writeln!(f, " Private View Key: {}", self.private_view_key)?;
        writeln!(f, " Public Spend Key: {}", self.public_spend_key)?;
        writeln!(f, " Public View Key: {}", self.public_view_key)?;
        writeln!(f, " Address Format: {}", self.address_format.to_string())?;
        writeln!(f, " Public Address: {}", self.public_address)?;
        Ok(())
    }
}

impl CryptoWalletGeneral for MoneroWallet {
    fn crypto_type(&self) -> CryptoCoin {
        self.crypto_type
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BlockchainClient {
    blockchain_client: reqwest::Client,
}

impl BlockchainClient {
    pub fn new(url: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            blockchain_client: reqwest::Client::new(),
        })
    }
}
