use std::str::FromStr;

use anyhow::anyhow;
use walletd_bitcoin::Blockstream;
use walletd_coin_model::BlockchainConnector;
use walletd_hd_key::{HDNetworkType, SlipCoin};

use crate::{
    Bip39Language, Bip39Mnemonic, Bip39MnemonicType, KeyPair, MnemonicHandler,
};



// TODO(AS): Need to streamline this onboard module


// TODO(AS): add this functionality back in in a streamlined way
// pub fn recover_existing_keypair(
//     mnemonic: Bip39Mnemonic,
//     network_type: HDNetworkType,
// ) -> Result<KeyPair, anyhow::Error> {

//             let mnemonic = Bip39Mnemonic::detect_language(mnemonic_phrase, passphrase)?;
//             Ok(KeyPair::new(
//                 mnemonic.to_seed(),
//                 mnemonic.phrase(),
//                 mnemonic_keypair_type,
//                 passphrase,
//                 network_type,
//             ))

// }


// /// TODO(AS): Not sure this belongs here
// /// Capitalizes the first character in s.
// /// Commenting out for now, uncomment and move to a more appropropriate location
// pub fn capitalize(s: &str) -> String {
//     let mut c = s.chars();
//     match c.next() {
//         None => String::new(),
//         Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
//     }
// }


// TODO(AS): refactor this in a streamlined way
// pub fn create_new_keypair(
//     mnemonic_keypair_type: MnemonicKeyPairType,
//     specified_language: Option<String>,
//     specified_num_words: Option<usize>,
//     passphrase: Option<&str>,
//     network_type: HDNetworkType,
// ) -> Result<KeyPair, anyhow::Error> {
//     // branches based on mnemonic type
//     match mnemonic_keypair_type {
//         MnemonicKeyPairType::HdBip39 => {
//             // defaults if user does not specify
//             let mut mnemonic_type = Bip39MnemonicType::Words12;
//             let mut mnemonic_language = Bip39Language::English;

//             if let Some(num_words_mnemonic) = specified_num_words {
//                 mnemonic_type = Bip39MnemonicType::from_word_count(num_words_mnemonic)?;
//             }
//             if let Some(language) = specified_language {
//                 mnemonic_language = Bip39Language::from_str(&capitalize(
//                     language.to_lowercase().replace("_", " ").as_str(),
//                 ))?;
//             }
//             let mnemonic = Bip39Mnemonic::new(mnemonic_language, mnemonic_type, passphrase);

//             Ok(KeyPair::new(
//                 mnemonic.to_seed(),
//                 mnemonic.phrase(),
//                 mnemonic_keypair_type,
//                 passphrase,
//                 network_type,
//             ))
//         }
//         _ => Err(anyhow!("Mnemonic type not supported")),
//     }
// }

// TODO(AS): this should be handled in the individual wallet level
// pub fn initilize_blockchain_client(
//     slip_coin: SlipCoin,
//     network_type: HDNetworkType,
// ) -> Result<Box<dyn BlockchainConnector>, anyhow::Error> {
//     match slip_coin {
//         SlipCoin::BTC => {
//             let blockchain_client = match network_type {
//                 HDNetworkType::MainNet => {
//                     Box::new(Blockstream::new(walletd_bitcoin::BLOCKSTREAM_URL)?)
//                         as Box<dyn BlockchainConnector>
//                 }
//                 HDNetworkType::TestNet => {
//                     Box::new(Blockstream::new(walletd_bitcoin::BLOCKSTREAM_TESTNET_URL)?)
//                         as Box<dyn BlockchainConnector>
//                 }
//             };
//             Ok(blockchain_client)
//         }
//         SlipCoin::ETH => {
//             let blockchain_client = match network_type {
//                 HDNetworkType::MainNet => Box::new(walletd_ethereum::BlockchainClient::new(
//                     walletd_ethereum::INFURA_MAINNET_ENDPOINT,
//                 )?) as Box<dyn BlockchainConnector>,
//                 HDNetworkType::TestNet => Box::new(walletd_ethereum::BlockchainClient::new(
//                     walletd_ethereum::INFURA_GOERLI_ENDPOINT,
//                 )?) as Box<dyn BlockchainConnector>,
//             };
//             Ok(blockchain_client)
//         }
//         _ => Err(anyhow!(
//             "Blockchain client not currently implemented for {}",
//             slip_coin
//         )),
//     }
// }
