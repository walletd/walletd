use core::fmt;
use std::any::Any;

use async_trait::async_trait;
use base58::ToBase58;
use hex;
use sha2::{Digest, Sha256};

use crate::CryptoCoin;

#[async_trait]
pub trait CryptoWallet: Sized {
    type MnemonicSeed;
    type HDKeyInfo;
    type AddressFormat;
    type CryptoAmount;
    type BlockchainClient;
    type NetworkType;

    fn new_from_hd_keys(
        hd_keys: &Self::HDKeyInfo,
        address_format: Self::AddressFormat,
    ) -> Result<Self, anyhow::Error>;

    fn new_from_mnemonic_seed(
        mnemonic_seed: &Self::MnemonicSeed,
        network: Self::NetworkType,
        address_format: Self::AddressFormat,
    ) -> Result<Self, anyhow::Error>;

    fn public_address(&self) -> String;

    fn to_private_key_wif(seed: &[u8], network_prefix: u8) -> Result<String, anyhow::Error> {
        // using wallet import format: https://en.bitcoin.it/wiki/Wallet_import_format
        let mut private_key: Vec<u8> = Vec::new();
        private_key.push(network_prefix);
        private_key.append(&mut seed.to_vec());
        // assuming public key is compressed
        private_key.push(0x01);
        let mut checksum =
            Sha256::digest(&Sha256::digest(&private_key.as_slice()).to_vec())[0..4].to_vec();
        private_key.append(&mut checksum);
        Ok(private_key.to_base58())
    }

    fn to_public_key_hex(public_key: &[u8]) -> Result<String, anyhow::Error> {
        Ok(hex::encode(public_key))
    }

    fn to_0x_hex_format(key: &[u8]) -> Result<String, anyhow::Error> {
        Ok(format!("0x{}", hex::encode(key)))
    }

    fn to_bytes_format(key: &[u8]) -> Result<String, anyhow::Error> {
        Ok(format!("{:?}", key))
    }

    async fn balance(
        &self,
        blockchain_client: &Self::BlockchainClient,
    ) -> Result<Self::CryptoAmount, anyhow::Error>;

    async fn transfer(
        &self,
        client: &Self::BlockchainClient,
        send_amount: &Self::CryptoAmount,
        public_address: &str,
    ) -> Result<(), anyhow::Error>;

    fn crypto_type(&self) -> CryptoCoin;
}

/// No associated types

pub trait CryptoWalletGeneral: fmt::Display {
    fn crypto_type(&self) -> CryptoCoin;
    fn as_any(&self) -> &dyn Any;
}
