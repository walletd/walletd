use core::fmt;
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
use web3::types::{Address, TransactionParameters};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{EthereumAmount, EthereumFormat};

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
        let private_key =
            EthereumPrivateKey::from_slice(&derived_key.extended_private_key()?.to_bytes())?;
        let public_key =
            EthereumPublicKey::from_slice(&derived_key.extended_public_key()?.to_bytes())?;
        let public_address = public_key.to_public_address(self.address_format)?;

        let wallet = EthereumWallet {
            address_format: self.address_format,
            public_address,
            private_key: Some(private_key),
            public_key: Some(public_key),
            network: master_hd_key.network(),
            blockchain_client: None,
            derived_hd_key: Some(derived_key),
        };
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
