use anyhow::anyhow;
use async_trait::async_trait;
use core::{fmt, fmt::Display};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::any::Any;
use std::str::FromStr;
use tiny_keccak::{Hasher, Keccak};
use walletd_bip39::Seed;
use walletd_coins::{BlockchainConnector, CryptoCoin, CryptoWallet, CryptoWalletGeneral};
use walletd_hd_keys::{HDKeyPair, NetworkType};
use web3::api::Eth;
use web3::transports::Http;
use web3::{
    ethabi::ethereum_types::U256,
    types::{Address, TransactionParameters},
};

mod ethereum_amount;
pub use ethereum_amount::EthereumAmount;

// run ganache-cli
pub const URL: &str = "http://localhost:8545";

// run ganache-cli to use localhost
pub const LOCALHOST_URL: &str = "http://localhost:8545";
pub const INFURA_MAINNET_ENDPOINT: &str =
    "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
pub const INFURA_ROPSTEN_ENDPOINT: &str =
    "https://ropsten.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
pub const INFURA_GOERLI_ENDPOINT: &str =
    "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
pub const GOERLI_TEST_ADDRESS: &str = "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";

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

#[async_trait]
impl CryptoWallet for EthereumWallet {
    type MnemonicSeed = Seed;
    type HDKeyInfo = HDKeyPair;
    type AddressFormat = EthereumFormat;
    type CryptoAmount = EthereumAmount;
    type BlockchainClient = BlockchainClient;
    type NetworkType = NetworkType;

    fn crypto_type(&self) -> CryptoCoin {
        CryptoCoin::ETH
    }

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

    fn new_from_mnemonic_seed(
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

    async fn balance(
        &self,
        blockchain_client: &Self::BlockchainClient,
    ) -> Result<Self::CryptoAmount, anyhow::Error> {
        let address = web3::types::H160::from_str(&self.public_address())?;
        blockchain_client.balance(address).await
    }

    async fn transfer(
        &self,
        blockchain_client: &Self::BlockchainClient,
        send_amount: &Self::CryptoAmount,
        to_address: &str,
    ) -> Result<(), anyhow::Error> {
        let to = Address::from_str(to_address)?;
        let amount = U256::from_dec_str("1000000")?; // hack hard code


        // Build tx object
        let tx_object = TransactionParameters {
            to: Some(to),
            value: amount,
            ..Default::default()
        };

        let private_key = self.private_key();
        // Chop off the 0x prefix
        let private_key_slice = &private_key[2..];
        let key = SecretKey::from_str(private_key_slice)?;

        // sign the tx
        let signed = blockchain_client
            .client
            .accounts()
            .sign_transaction(tx_object, &key)
            .await?;
        

        let result = blockchain_client
            .eth
            .send_raw_transaction(signed.raw_transaction)
            .await?;
            
        println!("Tx succeeded: Hash: {:#?}, EtherScan address: https://goerli.etherscan.io/tx/{:#?}", &result, &result);
        Ok(())
    }
}

impl EthereumWallet {
    fn private_key(&self) -> String {
        self.private_key.clone()
    }

    pub fn public_address_checksummed_from_public_key(
        public_key: &Vec<u8>,
    ) -> Result<String, anyhow::Error> {
        let public_key_full = PublicKey::from_slice(&public_key)?;
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(&public_key_full.serialize_uncompressed()[1..]);
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
        hasher.update(&public_key_full.serialize_uncompressed()[1..]);
        hasher.finalize(&mut output);
        let mut address = hex::encode(&output[12..]).to_lowercase();
        address = format!("{}{}", "0x", address);
        Ok(address)
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
    client: web3::Web3<Http>,
    eth: Eth<Http>,
}

impl BlockchainConnector for BlockchainClient {
    fn new(url: &str) -> Result<Self, anyhow::Error> {
        let transport = web3::transports::Http::new(url)?;
        let web3 = web3::Web3::new(transport);
        let web3_eth = web3.eth();

        Ok(Self {
            client: web3,
            eth: web3_eth,
        })
    }
}

impl BlockchainClient {
    pub async fn balance(
        &self,
        address: web3::types::H160,
    ) -> Result<EthereumAmount, anyhow::Error> {
        let balance = self.eth.balance(address, None).await?;
        Ok(EthereumAmount { wei: balance })
    }
}
