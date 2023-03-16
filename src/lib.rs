pub use ::walletd_bip39::{
    Language as Bip39Language, Mnemonic as Bip39Mnemonic, MnemonicHandler,
    MnemonicType as Bip39MnemonicType, Seed,
};
pub use ::walletd_monero_mnemonic::{
    Language as MoneroLanguage, Mnemonic as MoneroMnemonic, MnemonicType as MoneroMnemonicType,
};
use anyhow::anyhow;
pub use walletd_coin_model::{BlockchainConnector, CryptoWallet, CryptoWalletGeneral};
pub use walletd_hd_key::{DeriveType, HDKey};
use walletd_hd_key::{NetworkType, SlipCoin};
pub use {
    ::walletd_bip39, walletd_bitcoin, walletd_coin_model, walletd_ethereum, walletd_hd_key,
    walletd_monero, walletd_monero_mnemonic, walletd_solana,
};
pub mod onboard;

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
    pub associated_derived_info: Vec<HDKey>,
    pub network_type: NetworkType,
}

impl KeyPair {
    pub fn new(
        mnemonic_seed: Seed,
        mnemonic_phrase: String,
        style: MnemonicKeyPairType,
        passphrase_str: Option<&str>,
        network_type: NetworkType,
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
            network_type,
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

pub struct AssociatedWallets {
    pub wallets: Vec<Box<dyn CryptoWalletGeneral>>,
    pub derived_info: Vec<HDKey>,
    pub any_transaction_history: bool,
}

impl AssociatedWallets {
    /// Discovers wallets with in sequential order based on derivation path,
    /// stopping discover when gap limit (n consecutive wallets without
    /// transaction history) has been met Only considers change index = 0
    /// (the receiving/external chain) when considering the gap limit but if
    /// there is transaction history with change index = 1 it is added
    pub async fn new_discover_associated_wallets(
        crypto_coin: SlipCoin,
        bip32_master: &HDKey,
        deriv_type: &DeriveType,
        network_type: &NetworkType,
        gap_limit_specified: Option<usize>,
    ) -> Result<Self, anyhow::Error> {
        match crypto_coin {
            SlipCoin::BTC => {
                let blockchain_client;
                match network_type {
                    NetworkType::TestNet => {blockchain_client = walletd_bitcoin::Blockstream::new(walletd_bitcoin::BLOCKSTREAM_TESTNET_URL)?;},
                    NetworkType::MainNet => {blockchain_client = walletd_bitcoin::Blockstream::new(walletd_bitcoin::BLOCKSTREAM_URL)?;},
                }
                Self::new_search_blockchain_for_associated_wallets(blockchain_client, crypto_coin, bip32_master, deriv_type, network_type, gap_limit_specified).await
             }
            // Haven't implemented for others yet including ethereum 
            _ => return Err(anyhow!("Blockchain connection default not currently set up for {} so cannot scan for associated wallets", crypto_coin)),
        }
    }

    /// Helper function for
    /// new_discover_associated_wallets_sequential_with_gap_limit by search
    /// along a particular blockchain Discovers wallets with in sequential
    /// order based on derivation path, stopping discover when gap limit (n
    /// consecutive wallets without transaction history) has been met
    /// Only considers change index = 0 (the receiving/external chain) when
    /// considering the gap limit but if there is transaction history with
    /// change index = 1 it is added
    pub async fn new_search_blockchain_for_associated_wallets(
        blockchain_client: impl BlockchainConnector + std::marker::Sync,
        crypto_coin: SlipCoin,
        bip32_master: &HDKey,
        deriv_type: &DeriveType,
        network_type: &NetworkType,
        gap_limit_specified: Option<usize>,
    ) -> Result<Self, anyhow::Error> {
        let mut associated_wallets: Vec<Box<dyn CryptoWalletGeneral>> = Vec::new();
        let mut derived_info: Vec<HDKey> = Vec::new();
        let mut any_transaction_history = false;
        let coin_type = SlipCoin::BTC;
        let mut gap_limit = 20; // default gap limit
        if let Some(limit) = gap_limit_specified {
            gap_limit = limit
        }
        let mut current_gap = 0;
        let mut search_next_account = true;
        let mut account_index = 0; // hardened
        let mut address_index = 0; // not hardened

        while search_next_account {
            search_next_account = false;
            // println!("account_index: {}", account_index);
            while current_gap < gap_limit {
                for change_index in 0..2 {
                    let derived = deriv_type.derive_specify_change_account_address_indices(
                        &bip32_master,
                        &coin_type,
                        change_index,
                        account_index,
                        address_index,
                    )?;
                    let exists: bool;
                    match crypto_coin {
                        SlipCoin::BTC => {
                            let wallet = walletd_bitcoin::BitcoinWallet::from_hd_key(
                                &derived,
                                walletd_bitcoin::AddressType::P2wpkh,
                            )?;
                            exists = blockchain_client
                                .check_if_past_transactions_exist(&wallet.public_address_string())
                                .await?;
                            if exists {
                                any_transaction_history = true;
                            }
                            if exists || change_index == 0 {
                                associated_wallets.push(Box::new(wallet));
                                derived_info.push(derived);
                                println!("account_index: {}, address_index: {}, previous transaction history: {}", account_index, address_index, exists);
                            }
                        }
                        // couldn't figure out how to check for past transaction history for
                        // ethereum and others, not implemented yet
                        _ => {
                            return Err(anyhow!(
                                "Currently not handling scanning for associated wallets for {}",
                                crypto_coin
                            ))
                        }
                    }

                    if exists {
                        search_next_account = true;
                    } else if change_index == 0 {
                        current_gap += 1;
                    }
                }
                address_index += 1;
            }
            account_index += 1;
            address_index = 0;
            current_gap = 0;
        }

        Ok(Self {
            wallets: associated_wallets,
            derived_info,
            any_transaction_history,
        })
    }
}
