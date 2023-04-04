use ::walletd_bip39::Seed;

use walletd_coin_model::{BlockchainConnector, CryptoWallet};

use walletd_hd_key::{HDKey, HDNetworkType};

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
        let passphrase;
        match passphrase_str {
            Some(p) => passphrase = Some(p.to_string()),
            None => passphrase = None,
        }
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
        let passphrase_str;
        match &self.passphrase {
            Some(p) => passphrase_str = Some(p.as_str()),
            None => passphrase_str = None,
        }
        passphrase_str
    }

    /// Returns the master HD key
    pub fn to_master_key(&self) -> HDKey {
        let seed = self.mnemonic_seed.as_bytes();
        HDKey::new(seed, self.network_type).expect("Failed to create master key")
    }

    /// Returns the HD network type
    pub fn network_type(&self) -> HDNetworkType {
        self.network_type
    }

    /// Derives a wallet of the specified generic type T, given a blockchain client as an argument
    /// T must implement the CryptoWallet trait
    pub fn derive_wallet<T>(
        &self,
        blockchain_client: Box<dyn BlockchainConnector>,
    ) -> Result<T, anyhow::Error>
    where
        T: CryptoWallet,
    {
        let wallet = T::new(&self.to_master_key(), Some(blockchain_client))?;
        Ok(wallet)
    }
}
