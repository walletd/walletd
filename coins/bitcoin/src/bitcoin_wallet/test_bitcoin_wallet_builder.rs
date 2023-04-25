use crate::{
    blockstream::Blockstream, AddressType, BitcoinWalletBuilder, Error, HDKey, Network, Seed,
};
use std::str::FromStr;
use walletd_coin_core::{BlockchainConnector, CryptoWalletBuilder};
use walletd_hd_key::HDNetworkType;

#[test]
fn test_default() -> Result<(), Error> {
    let builder = BitcoinWalletBuilder::default();
    assert_eq!(builder.address_format, AddressType::P2wpkh);
    assert_eq!(builder.account_discovery, true);
    assert!(builder.gap_limit_specified.is_some());
    assert_eq!(
        builder
            .gap_limit_specified
            .expect("should be some due to previous check"),
        20
    );
    assert!(builder.master_hd_key.is_none());
    assert!(builder.mnemonic_seed.is_none());
    assert_eq!(builder.network_type, Network::Bitcoin);
    Ok(())
}

#[test]
fn test_new() -> Result<(), Error> {
    let builder = BitcoinWalletBuilder::new();
    let default = BitcoinWalletBuilder::default();
    assert_eq!(builder.address_format, default.address_format);
    assert_eq!(builder.account_discovery, default.account_discovery);
    assert!(builder.gap_limit_specified.is_some());
    assert!(default.gap_limit_specified.is_some());
    assert_eq!(
        builder
            .gap_limit_specified
            .expect("should be some due to previous check"),
        default
            .gap_limit_specified
            .expect("should be some due to previous check")
    );
    assert!(builder.master_hd_key.is_none());
    assert!(builder.mnemonic_seed.is_none());
    assert_eq!(builder.network_type, default.network_type);
    assert_eq!(builder.hd_path_builder, default.hd_path_builder);
    assert!(builder.blockchain_client.is_none());
    assert_eq!(
        builder.blockchain_client.is_none(),
        default.blockchain_client.is_none()
    );
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
            .expect("should be some due to previous check"),
        seed
    );
    Ok(())
}

#[test]
fn test_with_master_hd_key() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex).unwrap();
    let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
    let mut builder = BitcoinWalletBuilder::default();
    builder.master_hd_key(master_hd_key.clone());
    assert!(builder.master_hd_key.is_some());
    assert_eq!(
        builder
            .master_hd_key
            .expect("should be some due to previous check"),
        master_hd_key
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
fn test_with_blockchain_client() -> Result<(), Error> {
    let mut builder = BitcoinWalletBuilder::default();
    builder.blockchain_client(Box::new(Blockstream::new(
        "https://blockstream.info/testnet/api",
    )?));
    assert!(builder.blockchain_client.is_some());
    Ok(())
}

#[test]
fn test_with_network_type() -> Result<(), Error> {
    let mut builder = BitcoinWalletBuilder::default();
    builder.network_type(Network::Testnet);
    assert_eq!(builder.network_type, Network::Testnet);
    Ok(())
}

#[test]
fn test_with_hd_path_builder() -> Result<(), Error> {
    let mut builder = BitcoinWalletBuilder::default();
    assert!(builder.hd_path_builder.purpose.is_some());
    assert!(builder.hd_purpose.is_some());
    assert_eq!(
        builder.hd_path_builder.purpose.expect("checked is some"),
        builder
            .hd_purpose
            .expect("checked is some")
            .to_shortform_num()
    );
    let mut hd_path_builder = builder.hd_path_builder.clone();
    hd_path_builder.coin_type_index(0).address_index(1);
    builder.hd_path_builder(hd_path_builder);
    assert!(builder.hd_path_builder.coin_type.is_some());
    assert!(builder.hd_path_builder.address_index.is_some());
    assert_eq!(
        builder.hd_path_builder.coin_type.expect("checked is some"),
        0
    );
    assert_eq!(
        builder
            .hd_path_builder
            .address_index
            .expect("checked is some"),
        1
    );

    Ok(())
}
