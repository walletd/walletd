use std::str::FromStr;

use walletd_hd_key::NetworkType;
use crate::{Bip39Language, Bip39Mnemonic, Bip39MnemonicType, KeyPair, MnemonicHandler, MnemonicKeyPairType,
    MoneroLanguage, MoneroMnemonic, MoneroMnemonicType,
};

pub fn recover_existing_keypair(
    mnemonic_keypair_type: MnemonicKeyPairType,
    mnemonic_phrase: &String,
    passphrase: Option<&str>,
    network_type: NetworkType,
) -> Result<KeyPair, anyhow::Error> {
    match mnemonic_keypair_type {
        MnemonicKeyPairType::HdBip39 | MnemonicKeyPairType::Bip39 => {
            let mnemonic = Bip39Mnemonic::detect_language(mnemonic_phrase, passphrase)?;
            println!("Recovered BIP39 Mnemonic: \n{}", mnemonic);
            Ok(KeyPair::new(
                mnemonic.to_seed(),
                mnemonic.phrase(),
                mnemonic_keypair_type,
                passphrase,
                network_type,
            ))
        }
        MnemonicKeyPairType::Monero => {
            let mnemonic = MoneroMnemonic::detect_language(mnemonic_phrase, passphrase)?;
            println!("Recovered Monero Mnemonic: \n{}", mnemonic);
            Ok(KeyPair::new(
                mnemonic.to_seed(),
                mnemonic.phrase(),
                mnemonic_keypair_type,
                passphrase,
                network_type,
            ))
        }
    }
}

/// Capitalizes the first character in s.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn create_new_keypair(
    mnemonic_keypair_type: MnemonicKeyPairType,
    specified_language: Option<String>,
    specified_num_words: Option<usize>,
    passphrase: Option<&str>,
    network_type: NetworkType,
) -> Result<KeyPair, anyhow::Error> {
    // branches based on mnemonic type
    match mnemonic_keypair_type {
        MnemonicKeyPairType::HdBip39 | MnemonicKeyPairType::Bip39 => {
            // defaults if user does not specify
            let mut mnemonic_type = Bip39MnemonicType::Words12;
            let mut mnemonic_language = Bip39Language::English;

            if let Some(num_words_mnemonic) = specified_num_words {
                mnemonic_type = Bip39MnemonicType::from_word_count(num_words_mnemonic)?;
            }
            if let Some(language) = specified_language {
                mnemonic_language = Bip39Language::from_str(&capitalize(
                    language.to_lowercase().replace("_", " ").as_str(),
                ))?;
            }
            let mnemonic = Bip39Mnemonic::new(mnemonic_language, mnemonic_type, passphrase);
            
            Ok(KeyPair::new(
                mnemonic.to_seed(),
                mnemonic.phrase(),
                mnemonic_keypair_type,
                passphrase,
                network_type,
            ))
        }

        MnemonicKeyPairType::Monero => {
            // defaults if user does not specify
            let mut mnemonic_type = MoneroMnemonicType::Words25;
            let mut mnemonic_language = MoneroLanguage::English;

            if let Some(num_words_mnemonic) = specified_num_words {
                mnemonic_type = MoneroMnemonicType::from_word_count(num_words_mnemonic)?;
            }
            if let Some(language) = specified_language {
                mnemonic_language = MoneroLanguage::from_str(language.to_lowercase().as_str())?;
            }
            let mnemonic = MoneroMnemonic::new(mnemonic_language, mnemonic_type, passphrase);
            
            Ok(KeyPair::new(
                mnemonic.to_seed(),
                mnemonic.phrase(),
                mnemonic_keypair_type,
                passphrase,
                network_type,
            ))
        }
    }
}
