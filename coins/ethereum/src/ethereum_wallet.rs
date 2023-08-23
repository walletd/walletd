use ::core::fmt;
use std::fmt::LowerHex;
use std::str::FromStr;

use crate::Error;
use crate::EthClient;
use crate::{EthereumAmount, EthereumFormat};

use bdk::bitcoin::secp256k1::ffi::types::AlignedType;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::bitcoin::util::bip32::ExtendedPubKey;
use bdk::keys::bip39::Mnemonic;
use bdk::keys::{DerivableKey, ExtendedKey};
use ethers::middleware::gas_oracle::GasNow;
use ethers::prelude::gas_oracle::GasOracleMiddleware;
use ethers::prelude::*;
use ethers::signers::coins_bip39::mnemonic;
// use ethers::providers::{Middleware};
// use ethers::types::{TransactionRequest};
// use ethers::signers::{Signer};
use secp256k1::{PublicKey, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use walletd_hd_key::{slip44, HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPurpose};
use walletd_mnemonics_core::Seed;
use zeroize::{Zeroize, ZeroizeOnDrop};

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
#[derive(Debug, Clone)]
pub struct EthereumWalletBuilder {
    address_format: EthereumFormat,
    master_hd_key: Option<HDKey>,
    mnemonic: Option<Mnemonic>,
    network_type: HDNetworkType,
    hd_path_builder: HDPathBuilder,
    chain_id: u64,
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
            mnemonic: None,
            network_type: HDNetworkType::MainNet,
            hd_path_builder,
            chain_id: 5, // Goerli
        }
    }
}

impl EthereumWalletBuilder {
    /// Creates a new EthereumWalletBuilder with defaults.
    pub fn new() -> Self {
        Self::default()
    }
    /// Builds the EthereumWallet with the specified options
    pub fn build(&self) -> Result<EthereumWallet, Error> {
        let master_hd_key = match (&self.master_hd_key, &self.mnemonic) {
            (None, None) => {
                return Err(Error::UnableToImportWallet(
                    "Neither the master HD key nor the mnemonic seed was provided".to_string(),
                ))
            }
            (Some(key), _) => key.clone(),
            (None, Some(seed)) => {
                let seed = seed.to_seed("");
                let seed = Seed::new(seed.to_vec());
                HDKey::new_master(seed, self.network_type)?
            }
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
        println!("hd_path_builder: {:?}", hd_path_builder);
        let derived_key = master_hd_key.derive(&hd_path_builder.build().to_string())?;

        let private_key: EthereumPrivateKey =
            EthereumPrivateKey::from_slice(&derived_key.extended_private_key()?.to_bytes())?;
        println!("Private key: {:?}", private_key);
        println!("Private key: {:?}", private_key.to_bytes());
        let public_key =
            EthereumPublicKey::from_slice(&derived_key.extended_public_key()?.to_bytes())?;
        let public_address = public_key.to_public_address(self.address_format)?;
        println!("Public key: {:?}", public_key);
        println!("public_address: {:?}", public_address);
        // we need secp256k1 context for key derivation
        let mut buf: Vec<AlignedType> = Vec::new();
        buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
        let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

        let mnemonic = &self.mnemonic.clone().unwrap();
        let xkey: ExtendedKey = mnemonic.clone().into_extended_key().unwrap();
        // Get xprv from the extended key
        let xprv = xkey.into_xprv(bdk::bitcoin::Network::Bitcoin).unwrap();
        let path = DerivationPath::from_str("m/44h/60h/0h/0/0").unwrap();

        let child = xprv.derive_priv(&secp, &path).unwrap();
        println!("Child at {}: {}", path, child);
        let xpub = ExtendedPubKey::from_priv(&secp, &child);
        println!("Public key at {}: {}", path, xpub);
        println!("private key bytes: {:?}", &child.private_key.secret_bytes());
        let private_key: EthereumPrivateKey =
            EthereumPrivateKey::from_slice(&child.private_key.secret_bytes())?;
        println!("private key: {:?}", private_key.to_bytes());
        let public_key = EthereumPublicKey {
            0: PublicKey::from_slice(&xpub.public_key.serialize()).unwrap(),
        };
        println!("test: {:?}", private_key);
        println!("test2: {:?}", public_key);
        let public_address = public_key.to_public_address(self.address_format)?;
        let wallet = EthereumWallet {
            address_format: self.address_format,
            public_address,
            private_key: Some(private_key),
            public_key: Some(public_key),
            network: self.network_type.clone(),
            blockchain_client: None,
            derived_hd_key: Some(derived_key),
        };
        Ok(wallet)
    }

    /// Allows specification of the address format for the wallet
    pub fn address_format(&mut self, address_format: EthereumFormat) -> &mut Self {
        self.address_format = address_format;
        self
    }

    /// Allows specification of the mnemonic seed for the wallet
    pub fn mnemonic(&mut self, mnemonic: Mnemonic) -> &mut Self {
        self.mnemonic = Some(mnemonic);
        self
    }

    // TODO: This network type is an oversimplification that we should consider refactoring. Eth has chain_ids and network_ids
    /// Allows specification of the network type for the wallet, the default is HDNetworkType::MainNet
    pub fn network_type(&mut self, network_type: HDNetworkType) -> &mut Self {
        self.network_type = network_type;
        self
    }
    /// Allows specification of the HDPathBuilder for the wallet.
    pub fn hd_path_builder(&mut self, hd_path_builder: HDPathBuilder) -> &mut Self {
        self.hd_path_builder = hd_path_builder;
        self
    }
    /// Sets the default HD Purpose
    pub fn default_hd_purpose() -> HDPurpose {
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

impl EthereumWallet {
    /// Returns the builder for the [EthereumWallet].
    pub fn builder() -> EthereumWalletBuilder {
        EthereumWalletBuilder::new()
    }
    ///  Returns the blance for this Ethereum Wallet.
    pub async fn balance(&self) -> Result<EthereumAmount, Error> {
        let blockchain_client = self.blockchain_client()?;
        let address = ethers::types::Address::from_str(&self.public_address())
            .map_err(|e| (Error::FromStr(e.to_string())))?;
        let balance = blockchain_client.balance(address).await?;
        Ok(balance)
    }

    // TODO: take chain_id as a parameter
    // TODO: Take index as a parameter and use that for deriving the wallet we want (refactor keystore)
    /// This function creates and broadcasts a basic Ethereum transfer transaction to the Ethereum mempool.
    pub async fn transfer(
        &self,
        send_amount: EthereumAmount,
        to_address: &str,
    ) -> Result<String, Error> {
        //let secret_key: &Result<EthereumPrivateKey, Error> = &self.private_key();

        let derived_hd_key = &self
            .derived_hd_key
            .as_ref()
            .expect("Derived HD key should be set");
        let private_key =
            EthereumPrivateKey::from_slice(&derived_hd_key.extended_private_key()?.to_bytes())?;

        // EthereumWallet stores the private key as a 32 byte array
        let secret_bytes = private_key.to_bytes();

        // Retrieve instance of blockchain connector (provider) using the private key's secret bytes
        let provider = &self.blockchain_client().unwrap().ethers();

        // Instantiate a ethers local wallet from the wallet's secret bytes
        let wallet_from_bytes = Wallet::from_bytes(&secret_bytes).unwrap();

        // 5 = goerli chain id

        // Link our wallet instance to our provider for signing our transactions
        let client = SignerMiddleware::new(provider, wallet_from_bytes.with_chain_id(5u64));
        let client = GasOracleMiddleware::new(client, GasNow::new());
        // Create a transaction request to send 10000 wei to the Goerli address
        // TODO: Use gas oracle for more complex transactions where required gas is not known
        // 21000 = basic transfer
        let tx = TransactionRequest::new()
            .to(to_address)
            .gas(21000)
            .value(send_amount.wei())
            .chain_id(5u64);

        let pending_tx = client.send_transaction(tx, None).await.unwrap();
        let receipt = pending_tx
            .await
            .unwrap()
            .ok_or_else(|| println!("tx dropped from mempool"))
            .unwrap();

        let tx = client
            .get_transaction(receipt.transaction_hash)
            .await
            .unwrap();

        let tx_hash_string = tx.unwrap().hash.to_string();
        Ok(tx_hash_string)
    }
    /// Set the Blockchain Client on the Wallet
    pub fn set_blockchain_client(&mut self, client: EthClient) {
        self.blockchain_client = Some(client);
    }
    /// Syncs the wallet with the blockchain by adding previously used addresses to the wallet.
    pub async fn sync(&mut self) -> Result<(), Error> {
        Ok(())
    }
    /// Retrieves the next recevie address of the wallet.
    pub fn receive_address(&self) -> Result<String, Error> {
        Ok(self.public_address())
    }
    /// Returns the Blockchain client.
    pub fn blockchain_client(&self) -> Result<&EthClient, Error> {
        match &self.blockchain_client {
            Some(client) => Ok(client),
            None => Err(Error::MissingBlockchainClient),
        }
    }

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
        self.public_address()
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
}
