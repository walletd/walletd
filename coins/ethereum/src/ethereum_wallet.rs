use ::core::fmt;
use std::fmt::LowerHex;
use std::str::FromStr;

use crate::Error;
use crate::EthClient;
use async_trait::async_trait;
use secp256k1::{PublicKey, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use walletd_bip39::Seed;
use walletd_coin_core::{CryptoWallet, CryptoWalletBuilder};
use walletd_hd_key::{slip44, HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPurpose};
//use web3::types::{Address, TransactionParameters};
use zeroize::{Zeroize, ZeroizeOnDrop};
use walletd_hd_key::prelude::*;
use walletd_hd_key::slip44::{Coin, Symbol};
use crate::{EthereumAmount, EthereumFormat};

use web3::types::{ H160 as oldH160, H256 as oldH256, U64 as oldU64};
use ethers::prelude::*;
use ethers::providers::{Middleware, Provider};
use ethers::providers::Http as ethersHttp;
use ethers::types::{Address, TransactionRequest, U256};
use ethers::types::{BlockId, Block, BlockNumber, H160, H256, U64};
use ethers::signers::Wallet as EthersWallet;

use ethers::core::rand::thread_rng;
use ethers::signers::{LocalWallet, Signer};

/// Represents a private key for an Ethereum wallet, wraps a [SecretKey] from the secp256k1 crate
#[derive(Debug, Clone)]
pub struct EthereumPrivateKey(SecretKey);

impl Zeroize for EthereumPrivateKey {
    fn zeroize(&mut self) {
        self.0 = SecretKey::from_slice(&[1u8; 32])
            .expect("Should be able to create a default EthereumPrivateKey for zeroize");
    }
}

impl Drop for EthereumPrivateKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl EthereumPrivateKey {
    /// Represent the private key as a byte array
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.serialize_secret()
    }

    /// Instantiate the private key from a slice of bytes, errors if given invalid bytes
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let secret_key = SecretKey::from_slice(bytes)?;
        Ok(EthereumPrivateKey(secret_key))
    }
}

impl LowerHex for EthereumPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.to_bytes() {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

/// Represents an EthereumPublicKey, wraps a [PublicKey] from the secp256k1 crate
#[derive(Debug, Clone)]
pub struct EthereumPublicKey(PublicKey);

impl EthereumPublicKey {
    /// Converts the public key to a byte array
    pub fn to_bytes(&self) -> [u8; 33] {
        self.0.serialize()
    }
    /// Constructs the public key from a slice of bytes, returns an [error][Error] if given invalid bytes
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let public_key = PublicKey::from_slice(bytes)?;
        Ok(EthereumPublicKey(public_key))
    }

    /// Returns the public address of the public key in the specified format
    pub fn to_public_address(&self, address_format: EthereumFormat) -> Result<String, Error> {
        let public_key_full = self.0;

        match address_format {
            EthereumFormat::Checksummed => {
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
            EthereumFormat::NonChecksummed => {
                let mut output = [0u8; 32];
                let mut hasher = Keccak::v256();
                hasher.update(&public_key_full.serialize_uncompressed()[1..]);
                hasher.finalize(&mut output);
                let mut address = hex::encode(&output[12..]).to_lowercase();
                address = format!("{}{}", "0x", address);
                Ok(address)
            }
        }
    }
}

impl LowerHex for EthereumPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.to_bytes() {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

/// Builder for [EthereumWallet], allows for specification of options for the ethereum wallet
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct EthereumWalletBuilder {
    #[zeroize(skip)]
    address_format: EthereumFormat,
    master_hd_key: Option<HDKey>,
    mnemonic_seed: Option<Seed>,
    #[zeroize(skip)]
    network_type: HDNetworkType,
    #[zeroize(skip)]
    hd_path_builder: HDPathBuilder,
}

impl Default for EthereumWalletBuilder {
    /// Specifies the default options for the EthereumWalletBuilder
    /// The default address format is EthereumFormat::Checksummed
    /// The default network type is HDNetworkType::MainNet
    /// The default HDPathBuilder is `m/44'/60'/0'/0/0`
    /// By default neither the master HD key nor the mnemonic seed are specified
    fn default() -> Self {
        let mut hd_path_builder = HDPathBuilder::default();
        hd_path_builder
            .purpose_index(Self::default_hd_purpose().to_shortform_num())
            .coin_type_index(slip44::Coin::from(slip44::Symbol::ETH).id());
        Self {
            address_format: EthereumFormat::Checksummed,
            master_hd_key: None,
            mnemonic_seed: None,
            network_type: HDNetworkType::MainNet,
            hd_path_builder,
        }
    }
}

impl CryptoWalletBuilder<EthereumWallet> for EthereumWalletBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn build(&self) -> Result<EthereumWallet, <EthereumWallet as CryptoWallet>::ErrorType> {
        let master_hd_key = match (&self.master_hd_key, &self.mnemonic_seed) {
            (None, None) => {
                return Err(Error::UnableToImportWallet(
                    "Neither the master HD key nor the mnemonic seed was provided".to_string(),
                ))
            }
            (Some(key), _) => key.clone(),
            (None, Some(seed)) => HDKey::new_master(seed.clone(), self.network_type)?,
        };
        println!("{:?}", &self.mnemonic_seed);
        let hd_purpose_num = self
            .hd_path_builder
            .purpose
            .unwrap_or(Self::default_hd_purpose().to_shortform_num());
        let coin_type_id = slip44::Coin::Ether.id();
        let mut hd_path_builder = HDPath::builder();
        hd_path_builder
            .purpose_index(hd_purpose_num)
            .hardened_purpose()
            .coin_type_index(coin_type_id)
            .hardened_coin_type();

        let derived_key = master_hd_key.derive(&hd_path_builder.build().to_string())?;
        
        let private_key: EthereumPrivateKey =
            EthereumPrivateKey::from_slice(&derived_key.extended_private_key()?.to_bytes())?;
        println!("pk: {:?}", private_key);

        let private_key2 = &derived_key.extended_private_key_serialized().unwrap();
        println!("pk2: {:?}", private_key2);

        let public_key =
            EthereumPublicKey::from_slice(&derived_key.extended_public_key()?.to_bytes())?;
        let public_address = public_key.to_public_address(self.address_format)?;
        println!("public k 3: {:?}", public_key);

        println!("We want this for signing:");
        println!("{:?}", public_key);
        println!("{:?}", private_key);
        println!("{:?}", derived_key);

        let wallet = EthereumWallet {
            address_format: self.address_format,
            public_address,
            private_key: Some(private_key),
            public_key: Some(public_key),
            network: master_hd_key.network(),
            blockchain_client: None,
            derived_hd_key: Some(derived_key),
        };
        println!("wallet: {:?}", wallet);
        Ok(wallet)
    }

    /// Allows specification of the master HD key for the wallet
    fn master_hd_key(&mut self, master_hd_key: HDKey) -> &mut Self {
        self.master_hd_key = Some(master_hd_key);
        self
    }

    /// Allows specification of the address format for the wallet
    fn address_format(&mut self, address_format: EthereumFormat) -> &mut Self {
        self.address_format = address_format;
        self
    }

    /// Allows specification of the mnemonic seed for the wallet
    fn mnemonic_seed(&mut self, mnemonic_seed: Seed) -> &mut Self {
        self.mnemonic_seed = Some(mnemonic_seed);
        self
    }

    // TODO: This network type is an oversimplification that we should consider refactoring. Eth has chain_ids and network_ids
    /// Allows specification of the network type for the wallet, the default is HDNetworkType::MainNet
    fn network_type(&mut self, network_type: HDNetworkType) -> &mut Self {
        self.network_type = network_type;
        self
    }

    fn hd_path_builder(&mut self, hd_path_builder: HDPathBuilder) -> &mut Self {
        self.hd_path_builder = hd_path_builder;
        self
    }
}

impl EthereumWalletBuilder {
    fn default_hd_purpose() -> HDPurpose {
        HDPurpose::BIP44
    }
}

/// Contains the information needed to interact with an Ethereum wallet with a single public address associated with it.
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct EthereumWallet {
    #[zeroize(skip)]
    address_format: EthereumFormat,
    #[zeroize(skip)]
    public_address: String,
    private_key: Option<EthereumPrivateKey>,
    #[zeroize(skip)]
    public_key: Option<EthereumPublicKey>,
    #[zeroize(skip)]
    network: HDNetworkType,
    #[zeroize(skip)]
    blockchain_client: Option<EthClient>,
    derived_hd_key: Option<HDKey>,
}

#[async_trait]
impl CryptoWallet for EthereumWallet {
    type ErrorType = Error;
    type BlockchainClient = EthClient;
    type CryptoAmount = EthereumAmount;
    type NetworkType = HDNetworkType;
    type WalletBuilder = EthereumWalletBuilder;
    type AddressFormat = EthereumFormat;

    fn builder() -> Self::WalletBuilder {
        EthereumWalletBuilder::new()
    }

    async fn balance(&self) -> Result<Self::CryptoAmount, Error> {
        let blockchain_client = self.blockchain_client()?;
        let address = ethers::types::Address::from_str(&self.public_address())
            .map_err(|e| (Error::FromStr(e.to_string())))?;
        let balance = blockchain_client.balance(address).await?;
        Ok(balance)
    }

    // TODO: take chain_id as a parameter
    // TODO: Take index as a parameter and use that for deriving the wallet we want (refactor keystore)
    async fn transfer(
        &self,
        send_amount: &Self::CryptoAmount,
        to_address: &str,
    ) -> Result<String, Error> {
        println!("self: {:?}", &self);
        let secret_key: &Result<EthereumPrivateKey, Error> = &self.private_key();
        println!("secret_key: {:?}", secret_key);

        let derived_hd_key = &self.derived_hd_key()?;
        let private_key =
                EthereumPrivateKey::from_slice(&derived_hd_key.extended_private_key()?.to_bytes())?;
        let address_derivation_path = &derived_hd_key.derivation_path.clone();
        
        // EthereumWallet stores the private key as a 32 byte array
        let secret_bytes = private_key.to_bytes();

        let wallet_from_bytes = Wallet::from_bytes(&secret_bytes).unwrap();

        let provider = &self.blockchain_client().unwrap().ethers();

        // Instantiate a ethers local wallet from the wallet's secret bytes

        // 5 = goerli chain id 

        // Link our wallet instance to our provider for signing our transactions
        let client = SignerMiddleware::new(provider, wallet_from_bytes.with_chain_id(5u64));    

        // Create a transaction request to send 10000 wei to the Goerli address
        let tx = TransactionRequest::new()
            .to("0x681dA56258fF429026449F1435aE87e1B6e9F85b")
            .gas(21000)
            .value(10000)
            .chain_id(5u64);


        println!("Initialising send: {:?}", &tx);
        
        let pending_tx = client.send_transaction(tx, None).await.unwrap();
        let receipt = pending_tx.await.unwrap().ok_or_else(|| println!("tx dropped from mempool")).unwrap();
        
        let tx = client.get_transaction(receipt.transaction_hash).await.unwrap();
        Ok("tx_id".to_string())
        // let wallet: LocalWallet = LocalWallet::from(secret_key).with_chain_id(5);
        // println!("the wallet {:?}", wallet);
        // let wallet_nonlocal = MnemonicBuilder::<English>::default()
        //     .phrase(phrase)
        //     .index(index)
        //     .unwrap()
        //     .build()
        //     .unwrap();
        // LocalWallet = Wallet<ethers_core::k256::ecdsa::SigningKey>;
        // let secp = Secp256k1::new();
        // let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        //let local_wallet = LocalWallet::from() 
        // let key = "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
        //     .parse::<LocalWallet>()
        //     .unwrap();
        // println!("{:?}", &wallet_nonlocal);
        

        // 
        // let ourhdkey = &self.derived_hd_key()?;

        // let key_we_want: &SecretKey = &private_key.0;
        // let public_key: &EthereumPublicKey = &self.public_key()?;
        // let private_key = self.private_key()?.0;

        // let derived = &self.master_hd_key()?;
        // println!("derived: {:?}", derived);
        // //let wallet_nonlocal = LocalWallet::from(key_we_want.clone());
        // // a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee
        // // 6ccb54994cfdc2f95895b51989f0fcaa37266421c6ec39dbdc3f2026209efd83988a9b19d7ea245fde0de423d5af973d733c4dc317dc551ca5fdecad4bd47447
        // // println!("wallet_nonlocal: {:?}", wallet_nonlocal);

        // //let Wallet = Wallet::from(private_key).unwrap();

        // //println!("Wallet: {:?}", Wallet);

        // // println!("kww: {:?}", key_we_want);
        // // println!("{:?}", private_key);
        // // println!("{:?}", &self);
        // // println!("private key: {:?}", &self.private_key());
        // let test = private_key.clone();
        // // println!("test: {:?}", test);
        // // let secret_key = test.unwrap();

        // // let wallet_nonlocal: Wallet = Wallet::from(&test); 
        // let wallet = LocalWallet::new(&mut thread_rng());
        // let test2 = &mut thread_rng();
        // // println!("thread_rng: {:?}", test2);

        
        // // let blockchain_client = self.blockchain_client()?;
        // let to = Address::from_str(to_address).map_err(|e| Error::FromStr(e.to_string()))?;
        // let amount: ethers::types::U256 = send_amount.wei();

        // // // TODO for Ethers
        // // let tx_object = TransactionParameters {
        // //     to: Some(to),
        // //     value: amount,
        // //     ..Default::default()
        // // };
        
        // let tx = ethers::types::TransactionRequest::new()
        //     .to("0x681dA56258fF429026449F1435aE87e1B6e9F85b")
        //     .gas(21000)
        //     .value(10000);

        

        // // let wallet: LocalWallet = EthersWallet::from_bytes(&private_key.to_bytes()).unwrap();
        // // println!("wallet: {:?}", wallet);

        // let i = &self.index(0);
        // println!("i: {:?}", i);

        // let derived_hd_key = &self.derived_hd_key()?;
        // //println!("account_deriv_path: {:?}", &account_deriv_path);
        // println!("self: {:?}", &self);

        // println!("master hd key {:?}", &self.master_hd_key());
        // println!("derived hd key {:?}", derived_hd_key);
        
        // println!("derived: {:?}", derived);
        // //println!("address derivation path: {}", address_derivation_path);

        // let eth_first_account_key = &self.master_hd_key().unwrap().derive("m/44'/60'/0'/0")?;
        // // the master seed should be the same for a child HD Key and it's parent HD Key

        // println!("{:?}", eth_first_account_key);
        // println!(
        //     "eth_first_account_key depth {} {:?}",
        //     eth_first_account_key.depth(),
        //     eth_first_account_key.extended_private_key()
        // );
        // let secret_key = self.private_key()?.0;
        // println!("{:?}", secret_key);
        // let signed = &self.blockchain_client()?.ethers().sign_transaction(&tx, &secret_key).await.unwrap();
            // .ethers()
            // .accounts()
            // .sign_transaction(tx_object, &secret_key)
            // .await?;

        // // sign the tx

        // let result = blockchain_client
        //     .eth()
        //     .send_raw_transaction(signed.raw_transaction)
        //     .await?;

        // let hash = hex::encode(result.as_bytes());

        // Ok(hash)
    }

    fn set_blockchain_client(&mut self, client: Self::BlockchainClient) {
        self.blockchain_client = Some(client);
    }

    async fn sync(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn receive_address(&self) -> Result<String, Error> {
        Ok(self.public_address())
    }

    fn blockchain_client(&self) -> Result<&EthClient, Error> {
        match &self.blockchain_client {
            Some(client) => Ok(client),
            None => Err(Error::MissingBlockchainClient),
        }
    }
}

impl EthereumWallet {
    /// Returns the address format used by the wallet
    pub fn address_format(&self) -> EthereumFormat {
        self.address_format
    }

    /// Returns the public address of the wallet
    pub fn public_address(&self) -> String {
        self.public_address.clone()
    }

    /// A convenience method for retrieving the string of a public_address
    pub fn address(&self) -> String {
        return self.public_address().to_string();
    }

    /// Return the pub/priv keys at a specified index
    /// For now, we're assuming that the path we're using for derivation is the default (m/44'/60'/0'/0/{index})
    pub fn index(&self, index: u64) -> (String, String) {
        
        // // A wallet 

        // let account_deriv_path = HDPath::builder()
        //     .purpose_index(HDPurpose::BIP44.to_shortform_num()) // 44' for Eth
        //     .coin_type_index(Coin::from(Symbol::ETH).id()) // 60' for Eth
        //     .account_index(0) // we'll work from acc 0
        //     .change_index(0) 
        //     .no_address_index()
        //     .build().to_string();

        // println!("account_deriv_path: {:?}", &account_deriv_path);
        //println!("self: {:?}", &self);
        //let derived_hd_key = master_hd_key.derive("m/84'/1'/0'/0/0")?;
        // let first_address_hd_key = HDKey::new(
        //     Seed::from_str(BTC_WALLET_TEST_SEED)?,
        //     HDNetworkType::TestNet,
        //     "m/84'/1'/0'/0/0",
        // )?;
        ("Yes".to_string(), "No".to_string())
    }

    /// Returns the network type used by the wallet
    pub fn network(&self) -> HDNetworkType {
        self.network
    }

    /// Returns the public key of the wallet
    pub fn public_key(&self) -> Result<EthereumPublicKey, Error> {
        if let Some(key) = self.public_key.clone() {
            Ok(key)
        } else {
            Err(Error::MissingPublicKey)
        }
    }

    /// Returns the private key of the wallet if it exists, otherwise returns an error
    pub fn private_key(&self) -> Result<EthereumPrivateKey, Error> {
        println!("{:?}", &self.private_key);
        if let Some(key) = self.private_key.clone() {
            Ok(key)
        } else {
            Err(Error::MissingPrivateKey)
        }
    }

    /// Returns the master HD key of the wallet if it exists, otherwise returns an error
    pub fn master_hd_key(&self) -> Result<HDKey, Error> {
        if let Some(key) = self.derived_hd_key.clone() {
            let master_key = HDKey::new(key.master_seed.clone(), key.network, "m")?;
            Ok(master_key)
        } else {
            Err(Error::MissingHDKey)
        }
    }

    /// Returns the derived HD key of the wallet if it exists, otherwise returns an error
    pub fn derived_hd_key(&self) -> Result<HDKey, Error> {
        if let Some(key) = self.derived_hd_key.clone() {
            Ok(key)
        } else {
            Err(Error::MissingHDKey)
        }
    }
}