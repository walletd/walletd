use super::*;

use crate::{AddressType, BitcoinWalletBuilder, Error, HDKey, Network, Seed};
use std::str::FromStr;
use walletd_hd_key::HDNetworkType;

#[test]
fn test_default() -> Result<(), Error> {
    let builder = BitcoinWalletBuilder::default();
    assert_eq!(builder.address_format, AddressType::P2wpkh);
    assert!(builder.mnemonic_seed.is_none());
    assert_eq!(builder.network_type, bdk::bitcoin::Network::Testnet);
    Ok(())
}

#[test]
fn test_new() -> Result<(), Error> {
    let builder = BitcoinWalletBuilder::new();
    let default = BitcoinWalletBuilder::default();
    assert_eq!(builder.address_format, default.address_format);
    assert!(builder.mnemonic_seed.is_none());
    assert_eq!(builder.network_type, default.network_type);
    Ok(())
}

#[test]
fn test_with_mnemonic_seed() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex).unwrap();
    let mut builder = BitcoinWalletBuilder::default();
    builder.mnemonic_seed(seed.clone());
    assert!(builder.mnemonic_seed.is_some());
    assert_eq!(
        builder
            .mnemonic_seed
            .clone()
            .expect("should be some due to previous check"),
        seed
    );
    Ok(())
}

#[test]
fn test_with_address_format() -> Result<(), Error> {
    let mut builder = BitcoinWalletBuilder::default();
    builder.address_format(AddressType::P2pkh);
    assert_eq!(builder.address_format, AddressType::P2pkh);
    Ok(())
}

#[test]
fn test_with_network_type() -> Result<(), Error> {
    let mut builder = BitcoinWalletBuilder::default();
    builder.network_type(bdk::bitcoin::Network::Testnet);
    assert_eq!(builder.network_type, bdk::bitcoin::Network::Testnet);
    Ok(())
}
