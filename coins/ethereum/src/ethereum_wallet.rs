//! # Ethereum Wallet (walletd implementation)
//!

use core::fmt;
use std::any::Any;
use std::fmt::LowerHex;
use std::str::FromStr;

use crate::Error;
use crate::EthClient;
use async_trait::async_trait;
use secp256k1::{PublicKey, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use walletd_bip39::Seed;
use walletd_coin_model::{
    BlockchainConnectorGeneral, CryptoWallet, CryptoWalletBuilder, CryptoWalletGeneral,
};
use walletd_hd_key::{slip44, HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPurpose};
use web3::types::{Address, TransactionParameters};

use crate::{EthereumAmount, EthereumFormat};

#[derive(Debug, Clone)]
pub struct EthereumPrivateKey(SecretKey);

impl EthereumPrivateKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.serialize_secret()
    }

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

#[derive(Debug, Clone)]
pub struct EthereumPublicKey(PublicKey);

impl EthereumPublicKey {
    pub fn to_bytes(&self) -> [u8; 33] {
        self.0.serialize()
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let public_key = PublicKey::from_slice(bytes)?;
        Ok(EthereumPublicKey(public_key))
    }

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

pub struct EthereumWalletBuilder {
    /// The address format used to generate the wallet, if the address format is not specified the default is used
    address_format: EthereumFormat,
    /// The master HD key used to import the wallet
    master_hd_key: Option<HDKey>,
    /// The blockchain client used to connect to the blockchain, if the blockchain client is not provided the wallet will be created without an associated blockchain client
    /// and the blockchain client can be set later using the `set_blockchain_client` method
    blockchain_client: Option<Box<dyn BlockchainConnectorGeneral>>,
    /// The mnemonic seed used to import the wallet, if the mnemonic seed is not provided, the master_hd_key must be provided
    /// If the master_hd_key is provided, the mnemonic seed will be ignored
    mnemonic_seed: Option<Seed>,
    /// The specified network type to use, if the master_hd_key is provided, the network type will be inferred from the master_hd_key and this network_type will be ignored
    network_type: HDNetworkType,
    /// Specifiyng a HDPathBuilder allows for customizing the derivation path used including which indices are hardened and will override the default
    /// The default HDPathBuilder uses hardened indices for the purpose, coin type, account ,and non-hardened indices for the change and address indices
    /// The default HDPathBuilder is `m/44'/60'/0'/0/0`
    hd_path_builder: HDPathBuilder,
}

impl Default for EthereumWalletBuilder {
    fn default() -> Self {
        let mut hd_path_builder = HDPathBuilder::default();
        hd_path_builder
            .with_purpose(Self::default_hd_purpose().to_shortform_num())
            .with_coin_type(slip44::Coin::from(slip44::Symbol::ETH).id());
        Self {
            address_format: EthereumFormat::Checksummed,
            master_hd_key: None,
            blockchain_client: None,
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

        let hd_purpose_num = self
            .hd_path_builder
            .purpose
            .unwrap_or(Self::default_hd_purpose().to_shortform_num());
        let coin_type_id = slip44::Coin::Ether.id();
        let mut hd_path_builder = HDPath::builder();
        hd_path_builder
            .with_purpose(hd_purpose_num)
            .with_purpose_hardened(true)
            .with_coin_type(coin_type_id)
            .with_coin_type_hardened(true);

        let derived_key = master_hd_key.derive(hd_path_builder.build().to_string())?;
        let private_key =
            EthereumPrivateKey::from_slice(&derived_key.extended_private_key()?.to_bytes())?;
        let public_key =
            EthereumPublicKey::from_slice(&derived_key.extended_public_key()?.to_bytes())?;
        let public_address = public_key.to_public_address(self.address_format)?;

        let mut wallet = EthereumWallet {
            address_format: self.address_format,
            public_address,
            private_key: Some(private_key),
            public_key: Some(public_key),
            network: master_hd_key.network(),
            blockchain_client: None,
            master_hd_key: Some(master_hd_key),
        };
        if let Some(blockchain_client) = &self.blockchain_client {
            wallet.blockchain_client = Some(blockchain_client.try_into()?);
        };
        Ok(wallet)
    }

    /// Allows specification of the master HD key for the wallet
    fn with_master_hd_key(&mut self, master_hd_key: HDKey) -> &mut Self {
        self.master_hd_key = Some(master_hd_key);
        self
    }

    /// Allows specification of the address format for the wallet
    fn with_address_format(&mut self, address_format: EthereumFormat) -> &mut Self {
        self.address_format = address_format;
        self
    }

    /// Allows specification of the blockchain client for the wallet
    fn with_blockchain_client(
        &mut self,
        blockchain_client: Box<dyn BlockchainConnectorGeneral>,
    ) -> &mut Self {
        self.blockchain_client = Some(blockchain_client);
        self
    }

    /// Allows specification of the mnemonic seed for the wallet
    fn with_mnemonic_seed(&mut self, mnemonic_seed: Seed) -> &mut Self {
        self.mnemonic_seed = Some(mnemonic_seed);
        self
    }

    /// Allows specification of the network type for the wallet, the default is HDNetworkType::MainNet
    fn with_network_type(&mut self, network_type: HDNetworkType) -> &mut Self {
        self.network_type = network_type;
        self
    }

    fn with_hd_path_builder(&mut self, hd_path_builder: HDPathBuilder) -> &mut Self {
        self.hd_path_builder = hd_path_builder;
        self
    }
}

impl EthereumWalletBuilder {
    fn default_hd_purpose() -> HDPurpose {
        HDPurpose::BIP44
    }
}

/// The EthereumWallet struct contains the information needed to interact with an Ethereum wallet with a single public address associated with it.
#[derive(Debug, Clone)]
pub struct EthereumWallet {
    address_format: EthereumFormat,
    public_address: String,
    private_key: Option<EthereumPrivateKey>,
    public_key: Option<EthereumPublicKey>,
    network: HDNetworkType,
    blockchain_client: Option<EthClient>,
    master_hd_key: Option<HDKey>,
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
        let address = web3::types::H160::from_str(&self.public_address())
            .map_err(|e| (Error::FromStr(e.to_string())))?;
        let balance = blockchain_client.balance(address).await?;
        Ok(balance)
    }

    async fn transfer(
        &self,
        send_amount: &Self::CryptoAmount,
        to_address: &str,
    ) -> Result<String, Error> {
        let blockchain_client = self.blockchain_client()?;
        let to = Address::from_str(to_address).map_err(|e| Error::FromStr(e.to_string()))?;
        let amount = send_amount.wei();

        let tx_object = TransactionParameters {
            to: Some(to),
            value: amount,
            ..Default::default()
        };

        let secret_key = self.private_key()?.0;

        // sign the tx
        let signed = blockchain_client
            .web3()
            .accounts()
            .sign_transaction(tx_object, &secret_key)
            .await?;

        let result = blockchain_client
            .eth()
            .send_raw_transaction(signed.raw_transaction)
            .await?;

        let hash = hex::encode(result.as_bytes());

        Ok(hash)
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

/// Technically speaking, an "EthereumWallet" is a public address, public key and
/// private key
impl EthereumWallet {
    /// Returns the address format used by the wallet
    pub fn address_format(&self) -> EthereumFormat {
        self.address_format
    }

    /// Returns the public address of the wallet
    pub fn public_address(&self) -> String {
        self.public_address.clone()
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
        if let Some(key) = self.private_key.clone() {
            Ok(key)
        } else {
            Err(Error::MissingPrivateKey)
        }
    }

    /// Returns the master HD key of the wallet if it exists, otherwise returns an error
    pub fn master_hd_key(&self) -> Result<HDKey, Error> {
        if let Some(key) = self.master_hd_key.clone() {
            Ok(key)
        } else {
            Err(Error::MissingMasterHDKey)
        }
    }
}

impl fmt::Display for EthereumWallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.public_address())
    }
}

impl CryptoWalletGeneral for EthereumWallet {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> Box<dyn CryptoWalletGeneral> {
        Box::new(self.clone())
    }
}

impl TryFrom<Box<dyn CryptoWalletGeneral>> for EthereumWallet {
    type Error = Error where Error: std::fmt::Display;

    fn try_from(value: Box<dyn CryptoWalletGeneral>) -> Result<Self, Self::Error> {
        match value.as_any().downcast_ref::<EthereumWallet>() {
            Some(wallet) => Ok(wallet.clone()),
            None => Err(Error::UnableToDowncastWallet),
        }
    }
}

impl From<EthereumWallet> for Box<dyn CryptoWalletGeneral> {
    fn from(wallet: EthereumWallet) -> Self {
        Box::new(wallet)
    }
}
