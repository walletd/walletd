//! # WalletD Ethereum Library
//!
//! This library provides a wallet implementation for Ethereum and blockchain-specific functionality.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use core::fmt;


mod ethclient;
pub use ethclient::EthClient;
mod ethereum_amount;
pub use ethereum_amount::EthereumAmount;
mod ethereum_wallet;
pub use ethereum_wallet::EthereumWallet;
mod error;
pub use error::Error;
pub use web3;

/// EthereumFormat is an enum that represents the format of an Ethereum address (checksummed or non-checksummed)
#[derive(Default, Debug, Clone, Copy)]
pub enum EthereumFormat {
    #[default]
    /// Checksummed is the checksummed format of an Ethereum address where the case of each letter is mixed using the checksum algorithm
    Checksummed,
    /// NonChecksummed is the non-checksummed format of an Ethereum address where the letters are all lowercase
    NonChecksummed,
}

impl fmt::Display for EthereumFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EthereumFormat::Checksummed => write!(f, "Checksummed"),
            EthereumFormat::NonChecksummed => write!(f, "NonChecksummed"),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use walletd_coin_model::{CryptoWallet, CryptoWalletBuilder};
    use walletd_bip39::{Mnemonic, MnemonicHandler, Language, MnemonicStyleBuilder, Seed};
    use walletd_hd_key::HDNetworkType;
    use std::str::FromStr;

    #[test]
    fn test_wallet_instantiation_from_mnemonic_seed() {

        let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
        let seed = Seed::from_str(seed_hex).unwrap();
        let wallet = EthereumWallet::builder().with_mnemonic_seed(
            seed).with_network_type(
            HDNetworkType::TestNet).build().unwrap();
        
        assert_eq!(
            &wallet.public_address(),
            "0x6EEb11eA2905fEe101f72BF94F792dbc2dfB42B7"
        );
        assert_eq!(
            format!("{:#x}", &wallet.private_key().unwrap()),
            "0xa5dcdaefa08013092ca37d3f60d46f27510df8777a3a7dd6a1b9f373352caa75"
        );
        assert_eq!(wallet.network(), HDNetworkType::TestNet);
    }
}


