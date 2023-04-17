use ::walletd_bip39::Seed;

use crate::{
    BlockchainConnectorGeneral, CryptoWallet, CryptoWalletBuilder, CryptoWalletGeneral, HDKey,
    HDNetworkType,
};

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPair {
    pub style: MnemonicKeyPairType,
    pub mnemonic_seed: Seed,
    pub mnemonic_phrase: String,
    pub passphrase: Option<String>,
    pub network_type: HDNetworkType,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MnemonicKeyPairType {
    HDBip39,
}

impl KeyPair {
    pub fn new(
        mnemonic_seed: Seed,
        mnemonic_phrase: String,
        style: MnemonicKeyPairType,
        passphrase_str: Option<&str>,
        network_type: HDNetworkType,
    ) -> Self {
        let passphrase = passphrase_str.map(|p| p.to_string());

        Self {
            style,
            mnemonic_seed,
            mnemonic_phrase,
            passphrase,
            network_type,
        }
    }

    /// Returns mnemonic phrase as a &str type
    pub fn mnemonic_phrase(&self) -> &str {
        self.mnemonic_phrase.as_str()
    }

    /// Returns passphrase as a Option<&str> type
    pub fn passphrase(&self) -> Option<&str> {
        self.passphrase.as_deref()
    }

    /// Returns the master HD key
    pub fn to_master_key(&self) -> HDKey {
        HDKey::new_master(self.mnemonic_seed.to_owned(), self.network_type)
            .expect("Failed to create master key")
    }

    /// Returns the HD network type
    pub fn network_type(&self) -> HDNetworkType {
        self.network_type
    }

    /// Derives a wallet of the specified generic type T, given a blockchain client as an argument
    /// T must implement the CryptoWallet trait
    pub fn derive_wallet<T>(
        &self,
        blockchain_client: Box<dyn BlockchainConnectorGeneral>,
    ) -> Result<T, Error>
    where
        T: CryptoWallet,
        T::WalletBuilder: CryptoWalletBuilder<T>,
        T::ErrorType: std::fmt::Display,
        <T as TryFrom<Box<dyn CryptoWalletGeneral>>>::Error: std::fmt::Display,
    {
        let wallet: T = T::builder()
            .with_master_hd_key(self.to_master_key())
            .with_blockchain_client(blockchain_client)
            .build()
            .map_err(|e| Error::DeriveWallet(e.to_string()))?;
        Ok(wallet)
    }
}
