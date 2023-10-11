use ::core::fmt;
use std::fmt::LowerHex;
use std::str::FromStr;

use crate::Error;
use crate::EthClient;
use crate::{EthereumAmount, EthereumFormat};

use bdk::bitcoin::secp256k1::ffi::types::AlignedType;
use bdk::bitcoin::secp256k1::PublicKey;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::util::bip32::ExtendedPubKey;
use bdk::keys::bip39::Mnemonic;
use bdk::keys::{DerivableKey, ExtendedKey};
use ethers::middleware::gas_oracle::GasNow;
use ethers::prelude::gas_oracle::GasOracleMiddleware;
use ethers::prelude::*;
// use ethers::providers::{Middleware};
// use ethers::types::{TransactionRequest};
// use ethers::signers::{Signer};
use tiny_keccak::{Hasher, Keccak};

/// Represents an EthereumPublicKey, wraps a [PublicKey] from the secp256k1 crate
#[derive(Debug, Clone)]
pub struct EthereumPublicKey(PublicKey);

impl EthereumPublicKey {
    /// Converts the public key to a byte array
    pub fn to_bytes(&self) -> [u8; 33] {
        self.0.serialize()
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EthereumWalletBuilder {
    address_format: EthereumFormat,
    mnemonic: Option<Mnemonic>,
    chain_id: u64,
}

impl Default for EthereumWalletBuilder {
    /// Specifies the default options for the EthereumWalletBuilder
    /// The default address format is EthereumFormat::Checksummed
    /// By default the mnemonic seed are specified
    fn default() -> Self {
        Self {
            address_format: EthereumFormat::Checksummed,
            mnemonic: None,
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
        if self.mnemonic.is_none() {
            return Err(Error::UnableToImportWallet(
                "The mnemonic seed was provided".to_string(),
            ));
        }

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
        // println!("Child at {}: {}", path, child);
        let xpub = ExtendedPubKey::from_priv(&secp, &child);
        // println!("Public key at {}: {}", path, xpub);
        // println!("private key bytes: {:?}", &child.private_key.secret_bytes());
        let public_key =
            EthereumPublicKey(PublicKey::from_slice(&xpub.public_key.serialize()).unwrap());
        // println!("test2: {:?}", public_key);
        let public_address = public_key.to_public_address(self.address_format)?;
        let wallet = EthereumWallet {
            address_format: self.address_format,
            public_address,
            private_key: Some(child),
            public_key: Some(xpub),
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
}

/// Contains the information needed to interact with an Ethereum wallet with a single public address associated with it.
#[derive(Debug, Clone)]
pub struct EthereumWallet {
    address_format: EthereumFormat,
    public_address: String,
    private_key: Option<ExtendedPrivKey>,
    public_key: Option<ExtendedPubKey>,
}

impl EthereumWallet {
    /// Returns the builder for the [EthereumWallet].
    pub fn builder() -> EthereumWalletBuilder {
        EthereumWalletBuilder::new()
    }

    ///  Returns the balance for this Ethereum Wallet.
    pub async fn balance(&self, provider: &Provider<Http>) -> Result<EthereumAmount, Error> {
        let address = ethers::types::Address::from_str(&self.public_address())
            .map_err(|e| (Error::FromStr(e.to_string())))?;
        let balance = EthClient::balance(provider, address).await?;
        Ok(balance)
    }

    // TODO: take chain_id as a parameter
    // TODO: Take index as a parameter and use that for deriving the wallet we want (refactor keystore)
    /// This function creates and broadcasts a basic Ethereum transfer transaction to the Ethereum mempool.
    pub async fn transfer(
        &self,
        provider: &Provider<Http>,
        send_amount: EthereumAmount,
        to_address: &str,
    ) -> Result<String, Error> {
        let private_key_bytes = self.private_key.unwrap().private_key.secret_bytes();
        // EthereumWallet stores the private key as a 32 byte array
        //let secret_bytes = private_key.to_bytes();

        // Instantiate a ethers local wallet from the wallet's secret bytes
        let wallet_from_bytes = Wallet::from_bytes(&private_key_bytes).unwrap();

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

    /// Syncs the wallet with the blockchain by adding previously used addresses to the wallet.
    pub async fn sync(&mut self) -> Result<(), Error> {
        Ok(())
    }
    /// Retrieves the next recevie address of the wallet.
    pub fn receive_address(&self) -> Result<String, Error> {
        Ok(self.public_address())
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

    /// Returns the extended public key of the eth wallet
    pub fn public_key(&self) -> Result<ExtendedPubKey, Error> {
        match &self.public_key {
            Some(public_key) => Ok(*public_key),
            None => Err(Error::MissingPublicKey),
        }
    }
}
