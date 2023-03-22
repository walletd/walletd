use core::fmt;
use std::any::Any;

use async_trait::async_trait;
use base58::ToBase58;
use hex;
use sha2::{Digest, Sha256};
use walletd_hd_key::SlipCoin;

#[async_trait]
pub trait CryptoWallet: Sized {
    // TODO(#61): create custom error type for each coin
    // and add a new associated type here for Error
    type MnemonicSeed;
    type HDKeyInfo;
    type AddressFormat;
    type CryptoAmount;
    type BlockchainClient;
    type NetworkType;

    fn from_hd_key(
        hd_keys: &Self::HDKeyInfo,
        address_format: Self::AddressFormat,
    ) -> Result<Self, anyhow::Error>;

    fn from_mnemonic(
        mnemonic_seed: &Self::MnemonicSeed,
        network: Self::NetworkType,
        address_format: Self::AddressFormat,
    ) -> Result<Self, anyhow::Error>;

    fn public_address_string(&self) -> String;

    fn to_private_key_wif(seed: &[u8], network_prefix: u8) -> Result<String, anyhow::Error> {
        // using wallet import format: https://en.bitcoin.it/wiki/Wallet_import_format
        let mut private_key: Vec<u8> = Vec::new();
        private_key.push(network_prefix);
        private_key.append(&mut seed.to_vec());
        // assuming public key is compressed
        private_key.push(0x01);
        let mut checksum = Sha256::digest(Sha256::digest(private_key.as_slice()))[0..4].to_vec();
        private_key.append(&mut checksum);
        Ok(private_key.to_base58())
    }
    // TODO(#61): Clean up these functions like the to_public_key_hex etc.
    // Provide good common interface for all coins using the CryptoWallet trait
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
    ) -> Result<String, anyhow::Error>;

    fn crypto_type(&self) -> SlipCoin;

    fn address_by_index(
        &self,
        bip32_master: &Self::HDKeyInfo,
        index: usize,
    ) -> Result<Box<dyn CryptoAddressGeneral>, anyhow::Error>;
}

// TODO(#61): Remove the fmt::Display requirement for CryptoWalletGeneral
/// General struct for a CryptoWallet with no associated types
/// This is used for the walletd to store a list of wallets of different types
/// This is needed because the walletd needs to store a list of wallets of
/// different types and the associated types are not allowed to be used in a
/// trait object
pub trait CryptoAddressGeneral: fmt::Display {
    fn crypto_type(&self) -> SlipCoin;
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn CryptoAddressGeneral>;
}
