
//! # Ethereum Wallet (walletd implementation)
//!

use core::fmt;
use std::any::Any;
use std::fmt::LowerHex;
use std::str::FromStr;

use async_trait::async_trait;
use secp256k1::{PublicKey, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use walletd_bip39::Seed;
use walletd_coin_model::{CryptoWallet, CryptoWalletGeneral, BlockchainConnectorGeneral};
use walletd_hd_key::{HDKey, HDNetworkType, HDPurpose, slip44};
use web3::types::{Address, TransactionParameters};
use crate::Error;

use crate::{EthereumFormat, EthBlockchainClient, EthereumAmount};

const DEFAULT_PURPOSE: HDPurpose = HDPurpose::BIP44;

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

    pub fn to_public_address(&self,
        address_format: EthereumFormat,
    ) -> Result<String, Error> {
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

            },
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



#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EthereumWallet {
    address_format: EthereumFormat,
    public_address: String,
    private_key: Option<EthereumPrivateKey>,
    public_key: Option<EthereumPublicKey>,
    network: HDNetworkType,
    blockchain_client: Option<EthBlockchainClient>,
    hd_key: Option<HDKey>,
}

#[async_trait]
impl CryptoWallet for EthereumWallet {
    type ErrorType = Error;
    type BlockchainClient = EthBlockchainClient;
    type CryptoAmount = EthereumAmount;
    type NetworkType = HDNetworkType;

    fn new(master_hd_key: &HDKey, blockchain_client: Option<Box<dyn BlockchainConnectorGeneral>>) -> Result<Self, Error> {
        let derived_key = master_hd_key.derive(DEFAULT_PURPOSE.full_deriv_path(slip44::Coin::from(slip44::Symbol::ETH).id(), 0, 0, 0))?;
        let address_format = EthereumFormat::default();

        let private_key = EthereumPrivateKey::from_slice(&derived_key
            .extended_private_key()?.to_bytes())?;
        let public_key =  EthereumPublicKey::from_slice(&derived_key
            .extended_public_key()?.to_bytes())?;
        let public_address = public_key.to_public_address(address_format)?;
        
        let mut wallet = EthereumWallet {
            address_format,
            public_address,
            private_key: Some(private_key),
            public_key: Some(public_key),
            network: master_hd_key.network(),
            hd_key: Some(master_hd_key.clone()),
            blockchain_client: None,
        };

        let blockchain_client = match blockchain_client {
            Some(blockchain_client) => Some(blockchain_client.try_into()?),
            None => None,
        };
        wallet.blockchain_client = blockchain_client;
        Ok(wallet)
    }

    async fn balance(
        &self,
    ) -> Result<Self::CryptoAmount, Error> {
        let blockchain_client = self.blockchain_client()?;
        let address = web3::types::H160::from_str(&self.public_address()).map_err(|e| (Error::FromStr(e.to_string())))?;
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
            .client()
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

    fn blockchain_client(&self) -> Result<&EthBlockchainClient, Error> {
        match &self.blockchain_client {
            Some(client) => Ok(client),
            None => Err(Error::MissingBlockchainClient),
        }
    }
}

/// Technically speaking, an "EthereumWallet" is a public address, public key and
/// private key
impl EthereumWallet {

   
    
    pub fn public_address(&self) -> String {
        self.public_address.clone()
    }

    pub fn network(&self) -> HDNetworkType {
        self.network
    }

    // TODO(AS): need to refactor from_hd_key and from_mnemonic when implementing a builder pattern
    pub fn from_hd_key(hd_key: &HDKey, address_format: EthereumFormat, blockchain_client: Option<EthBlockchainClient>) -> Result<Self, Error> {
       

        let private_key = EthereumPrivateKey::from_slice(&hd_key
            .extended_private_key()?.to_bytes())?;
        let public_key =  EthereumPublicKey::from_slice(&hd_key
            .extended_public_key()?.to_bytes())?;
        let public_address = public_key.to_public_address(address_format)?;

        Ok(Self {
            address_format,
            public_address,
            private_key: Some(private_key),
            public_key: Some(public_key),
            network: hd_key.network,
            hd_key: Some(hd_key.clone()),
            blockchain_client,
        })
    }

    pub fn from_mnemonic(
        mnemonic_seed: &Seed,
        network_type: HDNetworkType,
        address_format: EthereumFormat,
        blockchain_client: Option<EthBlockchainClient>,
    ) -> Result<Self, Error> {
        let seed_bytes = mnemonic_seed.as_bytes();
        let master_hd_key = HDKey::new(seed_bytes, network_type)?;
        let derived_key = master_hd_key.derive(DEFAULT_PURPOSE.full_deriv_path(slip44::Coin::from(slip44::Symbol::ETH).id(), 0, 0, 0))?;
        Self::from_hd_key(&derived_key, address_format, blockchain_client)
    }

    pub fn public_key(&self) -> Result<EthereumPublicKey, Error> {

        if let Some(key) = self.public_key.clone() {
            Ok(key)
        } else {
            Err(Error::MissingPublicKey)
        }
    }

    pub fn private_key(&self) -> Result<EthereumPrivateKey, Error> {
        if let Some(key) = self.private_key.clone() {
            Ok(key)
        } else {
            Err(Error::MissingPrivateKey)
        }
    }

    pub fn hd_key(&self) -> Result<HDKey, Error> {
        if let Some(key) = self.hd_key.clone() {
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
