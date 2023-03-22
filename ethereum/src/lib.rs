//! # Ethereum Wallet (walletd implementation)
//!
//! This crate implements Ethereum functionality for wallet-specific and
//! chain-specific functionality. We should consider moving wallet-agnostic
//! chain-specific functionality to a different module later
// https://mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073 -- Mainnet L1 Blockchain
// https://celo-mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073 -- This is an Infura blockchain that is an `Ethereum L1` client that connects to Infura's Celo Mainnet node.
// https://goerli.infura.io/v3/933b67502c4340a7bf3e873f0de62073 -- Goerli Testnet L1 Blockchain
// Goerli is an Ethereum test network that allows for blockchain development
// testing before deployment on Mainnet Adding Goerli to Metamask
// Step 1: Log in to your Metamask wallet and click on the dropdown of networks:
// Step 2: Click on Add Network
// Step 3: A following new window will pop up:
// Step 4: Now in left sidebar click on “Networks”, a new window will appear.
// There you can see all the available Testnet networks that are available but
// not visible. Step 5: Now click on the “Advanced” in left side bar
// Step 6: In “Advanced” tab when you scroll down, you will find “Show test
// networks” option, just put it “ON” Step 7: After completing these steps you
// can find the custom network in the dropdown list. For testing purposes, a Goerli faucet is available at https://goerlifaucet.com/.
// Arbitrary Mainnet address: 0x7a37eadaf5db28e2079f984e923ae14d485b9617
// 0xc8874652cd7cc403f0c7ae4cfb420012d0de3afef0041ad255ce12ee5344f13a
// address 0 - 0x9524D3834d09031d87B0192ad52caedc30d92d44 -- key
// 0x8db5f4b68fbba64a4b8034a9824d2c36b12387491f48f94a71743034ec8ebc7b faucet to address https://goerli.etherscan.io/tx/0x88ab1ff9c26d886309a9943dc58391e265c7e0e31d592e936457aca323c3977c
//
// Candidate functions for possible unified public interface:
//
// new() -- generates new mnemonic and derives the first address, outputing
// new_from_mnemonic() -- takes a mnemonic and derives the first address,
// outputing the address and private key new_from_address_and_keys(address,
// private key) -- takes an address and private key and outputs the address and
// private key wallet.balance() -- returns the balance of the wallet
// public_blockchain.get_balance(abritrary address) -- returns the balance of
// the arbitrary address) -- BlockchainClient is a possible good place to
// implement this
// Uniswap V2 factory client: 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f (https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)

use core::fmt;
use core::fmt::Display;
use std::any::Any;
use std::str::FromStr;

use async_trait::async_trait;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use walletd_bip39::Seed;
use walletd_coin_model::{BlockchainConnector, CryptoWallet, CryptoAddressGeneral};
use walletd_hd_key::{HDKey, HDNetworkType, SlipCoin};
use web3::api::Eth;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters};
use web3::Error;

pub mod ethclient;
pub use ethclient::EthClient;
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

#[derive(Default, Debug, Clone)]
pub enum EthereumFormat {
    #[default]
    Checksummed,
    NonChecksummed,
}

impl fmt::Display for EthereumFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EthereumFormat::Checksummed => write!(f, "Checksummed"),
            EthereumFormat::NonChecksummed => write!(f, "NonChecksummed"),
        }
    }
}

#[derive(Default, Debug, Clone)]
#[allow(dead_code)]
pub struct EthereumWallet {
    crypto_type: SlipCoin,
    address_format: EthereumFormat,
    public_address: String,
    private_key: String,
    public_key: String,
    network: HDNetworkType,
}

#[async_trait]
impl CryptoWallet for EthereumWallet {
    type AddressFormat = EthereumFormat;
    type BlockchainClient = BlockchainClient;
    type CryptoAmount = EthereumAmount;
    type HDKeyInfo = HDKey;
    type MnemonicSeed = Seed;
    type NetworkType = HDNetworkType;

    fn crypto_type(&self) -> SlipCoin {
        SlipCoin::ETH
    }

    fn from_hd_key(hd_keys: &HDKey, address_format: EthereumFormat) -> Result<Self, anyhow::Error> {
        let public_key_bytes = &hd_keys
            .extended_public_key
            .expect("extended public key data not available")
            .to_vec();
        let public_address: String = match address_format {
            EthereumFormat::Checksummed => {
                Self::public_address_checksummed_from_public_key(public_key_bytes)?
            }
            EthereumFormat::NonChecksummed => {
                Self::public_address_nonchecksummed_from_public_key(public_key_bytes)?
            }
        };
        Ok(Self {
            crypto_type: SlipCoin::ETH,
            address_format,
            public_address,
            private_key: hd_keys.private_key()?,
            public_key: hd_keys.public_key()?,
            network: hd_keys.network,
        })
    }

    // async fn balance(
    //     &self,
    //     blockchain_client: &Self::BlockchainClient,
    // ) -> Result<Self::CryptoAmount, anyhow::Error>;

    // async fn balance(&self, &Self.BlockchainClient) -> <Self::CryptoAmount,
    // anyhow::Error> { todo!() }

    fn from_mnemonic(
        mnemonic_seed: &Seed,
        network_type: HDNetworkType,
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
        let public_address: String = match address_format {
            EthereumFormat::Checksummed => {
                Self::public_address_checksummed_from_public_key(&public_key_bytes)?
            }
            EthereumFormat::NonChecksummed => {
                Self::public_address_nonchecksummed_from_public_key(&public_key_bytes)?
            }
        };

        Ok(Self {
            crypto_type: SlipCoin::ETH,
            address_format,
            public_address,
            private_key: Self::to_0x_hex_format(&private_key_bytes)?,
            public_key: Self::to_0x_hex_format(&public_key_bytes)?,
            network: network_type,
        })
    }

    fn public_address_string(&self) -> String {
        self.public_address.clone()
    }

    async fn balance(
        &self,
        blockchain_client: &Self::BlockchainClient,
    ) -> Result<Self::CryptoAmount, anyhow::Error> {
        let address = web3::types::H160::from_str(&self.public_address_string())?;
        let balance = blockchain_client.balance(address).await?;
        Ok(balance)
    }

    async fn transfer(
        &self,
        blockchain_client: &Self::BlockchainClient,
        send_amount: &Self::CryptoAmount,
        to_address: &str,
    ) -> Result<String, anyhow::Error> {
        // let mut send_string = send_amount;
        let to = Address::from_str(to_address)?;
        let amount = send_amount.wei();
        
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
        let hash = hex::encode(result.as_bytes());

        Ok(hash)
    }

    fn address_by_index(
        &self,
        bip32_master: &HDKey,
        index: usize,
    ) -> Result<Box<dyn CryptoAddressGeneral>, anyhow::Error> {
        let derived_key = HDKey::derive(
            bip32_master,
            format!("m/44'/60'/0'/0/{}", index),
        )?;
        Ok(Box::new(EthereumWallet::from_hd_key(
            &derived_key,
            EthereumFormat::Checksummed,
        )?))
    }
}

// Technically speaking, an "EthereumWallet" is a public address, public key and
// private key
impl EthereumWallet {
    // CryptoCoin::ETH => {
    // let wallet = wallet.as_any().downcast_ref::<EthereumWallet>()
    // .expect("Wallet with CryptoCoin::ETH should be able to be downcast to
    // EthereumWallet struct"); let blockchain_client;
    // match keypair.network_type {
    // NetworkType::MainNet => {
    // blockchain_client = walletd_ethereum::BlockchainClient::new(
    // walletd_ethereum::INFURA_MAINNET_ENDPOINT,
    // )?
    // }
    // NetworkType::TestNet => {
    // blockchain_client = walletd_ethereum::BlockchainClient::new(
    // walletd_ethereum::INFURA_GOERLI_ENDPOINT,
    // )?
    // }
    // }
    //
    // let transport = web3::transports::Http::new("http://localhost:8545")?;
    // let web3 = web3::Web3::new(transport);
    // get instance of mnemonic
    // get instance of deriv_path
    // get instance of deriv_type
    // let bip32: BIP32 = BIP32::new();
    // let ethereum_wallet: EthereumWallet = new_from_hd_keys(
    //     &bip32,
    //     EthereumFormat::Checksummed,
    // );
    //

    // pub async fn from_keypair(key_pair: HDKeyPair, deriv_path: DerivType,
    // deriv_type: DerivType) -> Result<Self, tinyerror::Error> {     let bip32:
    // BIP32 = walletd_hd_keys::DerivType::BIP32;     let ethereum_wallet:
    // EthereumWallet = EthereumWallet::from_hd_key(         &bip32,
    //         EthereumFormat::Checksummed,
    //     );
    //     Ok(ethereum_wallet)
    // }

    fn private_key(&self) -> String {
        self.private_key.clone()
    }

    // pub fn new(&self) -> Self {
    //     let wallet = EthereumWallet::from_phrase(
    //         &keypair.mnemonic_seed,
    //         keypair.network_type,
    //         EthereumFormat::Checksummed,
    //     );
    //     println!("Wallet Info: {}", &wallet);
    //     keypair.associated_wallets.push(Box::new(wallet));
    //     wallet
    // }

    pub fn public_address_checksummed_from_public_key(
        public_key: &[u8],
    ) -> Result<String, anyhow::Error> {
        let public_key_full = PublicKey::from_slice(public_key)?;
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(&public_key_full.serialize_uncompressed()[1..]);
        hasher.finalize(&mut output);
        let address = hex::encode(&output[12..]).to_lowercase();

        let mut checksum_address = String::new();
        let mut digest_out2 = [0u8; 32];
        let mut hasher2 = Keccak::v256();
        let address_bytes = address.as_bytes();
        hasher2.update(address_bytes);
        hasher2.finalize(&mut digest_out2);
        let keccak_digest_hex = hex::encode(digest_out2);

        for (i, address_char) in address.chars().enumerate() {
            let keccak_char = &keccak_digest_hex[i..i + 1];
            if u8::from_str_radix(keccak_char, 16)? >= 8 {
                checksum_address.push(address_char.to_ascii_uppercase());
            } else {
                checksum_address.push(address_char);
            }
        }
        checksum_address = format!("{}{}", "0x", checksum_address);
        Ok(checksum_address)
    }

    pub fn public_address_nonchecksummed_from_public_key(
        public_key: &[u8],
    ) -> Result<String, anyhow::Error> {
        let public_key_full = PublicKey::from_slice(public_key)?;
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
        writeln!(f, " Address Format: {}", self.address_format)?;
        writeln!(f, " Public Address: {}", self.public_address)?;
        Ok(())
    }
}

impl CryptoAddressGeneral for EthereumWallet {
    fn crypto_type(&self) -> SlipCoin {
        SlipCoin::ETH
    }

    fn as_any(&self) -> &dyn Any {
        self     
    }

    fn box_clone(&self) -> Box<dyn CryptoAddressGeneral> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct BlockchainClient {
    client: web3::Web3<Http>,
    eth: Eth<Http>,
    url: String,
}

impl BlockchainConnector for BlockchainClient {
    fn new(url: &str) -> Result<Self, anyhow::Error> {
        let transport = web3::transports::Http::new(url)?;
        let web3 = web3::Web3::new(transport);
        let web3_eth = web3.eth();

        Ok(Self {
            client: web3,
            eth: web3_eth,
            url: url.to_string(),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BlockchainClient {
    
    pub fn to_eth_client(&self) -> EthClient {
        EthClient::new(&self.url)
    }

    pub async fn balance(&self, address: web3::types::H160) -> Result<EthereumAmount, Error> {
        let balance = self.eth.balance(address, None).await?;
        Ok(EthereumAmount { wei: balance })
    }

    pub async fn gas_price(&self) -> Result<String, Error> {
        let gas_price = self.eth.gas_price().await?;
        Ok(gas_price.to_string())
    }
}

#[cfg(test)]
mod tests {

    use walletd_bip39::{Language, Mnemonic, MnemonicHandler};

    use super::*;

    #[test]
    fn test_initialise_blockchain_client() {
        let url = "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
        let client = BlockchainClient::new(url);
        assert_eq!(client.is_ok(), true);
    }

    #[test]
    fn test_wallet_instantiation_from_mnemonic() {
        let mnemonic_phrase: &str =
            "outer ride neither foil glue number place usage ball shed dry point";
        let passphrase: Option<&str> = Some("mypassphrase");
        let restored_mnemonic =
            Mnemonic::from_phrase(Language::English, mnemonic_phrase, passphrase).unwrap();
        let seed = restored_mnemonic.to_seed();

        let wallet = match EthereumWallet::from_mnemonic(
            &seed,
            HDNetworkType::MainNet,
            EthereumFormat::Checksummed,
        ) {
            Ok(wallet) => Ok(wallet),
            Err(e) => Err(e),
        };

        assert_eq!(wallet.is_ok(), true);
        assert_eq!(
            &wallet.as_ref().unwrap().public_address,
            "0xba57086A5CF8295449B9014D9ca3de538D70f665"
        );
        assert_eq!(
            &wallet.as_ref().unwrap().private_key,
            "0x3c536b023d71d81e6abc58b0b91c64caff8bb08fabf0c9f3cf948a9f3a494e8e"
        );
        assert_eq!(&wallet.as_ref().unwrap().crypto_type, &SlipCoin::ETH);
        assert_eq!(&wallet.as_ref().unwrap().network, &HDNetworkType::MainNet);
    }
}
