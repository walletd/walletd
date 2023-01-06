pub use ::walletd_bip39;
pub use ::walletd_bip39::{
    Language as Bip39Language, Mnemonic as Bip39Mnemonic, MnemonicHandler,
    MnemonicType as Bip39MnemonicType, Seed,
};
pub use ::walletd_monero_mnemonic::{
    Language as MoneroLanguage, Mnemonic as MoneroMnemonic, MnemonicType as MoneroMnemonicType,
};
pub use walletd_bitcoin;
pub use walletd_coins;
pub use walletd_coins::{CryptoWallet, CryptoWalletGeneral};
pub use walletd_ethereum;
pub use walletd_hd_keypairs;
pub use walletd_hd_keypairs::HDKeyPair;
pub use walletd_monero;
pub use walletd_monero_mnemonic;
pub use walletd_solana;

#[derive(PartialEq, Eq)]
pub enum MnemonicKeyPairType {
    HdBip39,
    Bip39,
    Monero,
}

pub struct KeyPair {
    pub style: MnemonicKeyPairType,
    pub mnemonic_seed: Seed,
    pub mnemonic_phrase: String,
    pub passphrase: Option<String>,
    pub associated_wallets: Vec<Box<dyn CryptoWalletGeneral>>,
    pub associated_derived_info: Vec<HDKeyPair>,
}

impl KeyPair {
    pub fn new(
        mnemonic_seed: Seed,
        mnemonic_phrase: String,
        style: MnemonicKeyPairType,
        passphrase_str: Option<&str>,
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
            associated_wallets: Vec::new(),
            associated_derived_info: Vec::new(),
        }
    }
}

impl KeyPair {
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
}
