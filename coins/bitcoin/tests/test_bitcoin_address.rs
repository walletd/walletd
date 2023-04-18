use std::str::FromStr;
use walletd_bitcoin::{BitcoinAddress, Seed, Error, HDKey, CryptoAddress, HDNetworkType, AddressType};

#[test]
fn test_from_hd_key_testnet() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex)?;
    let address_format = AddressType::P2wpkh;
    let hd_key = HDKey::new(seed, HDNetworkType::TestNet, "m/84'/1'/0'/0/0".to_string())?;
    let bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    let expected_address = "tb1q2knvzpjltz4uwh6j5wrmqn7lnzccsphpd85jp9";
    assert_eq!(bitcoin_address.public_address(), expected_address);
    Ok(())
}


#[test]
fn test_from_hd_key_mainnet_p2wpkh() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex)?;
    let address_format = AddressType::P2wpkh;
    let hd_key = HDKey::new(seed, HDNetworkType::MainNet, "m/84'/0'/0'/0/0".to_string())?;
    let bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    let expected_address = "bc1qqh5dyxhkqage7fnmn9mjhk4w072fekzsvpzzs7";
    assert_eq!(bitcoin_address.public_address(), expected_address);
    Ok(())
}

#[test]
fn test_from_hd_key_mainnet_p2sh() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex)?;
    let address_format = AddressType::P2sh;
    let compare_hd_key = HDKey::new(seed.clone(), HDNetworkType::MainNet, "m/49h".into())?;
    println!("compare_hd_key public_key {}", compare_hd_key.extended_public_key_serialized()?);
    let hd_key = HDKey::new(seed, HDNetworkType::MainNet, "m/49'/0'/0'/0/1".to_string())?;
    println!("HDKey: {:#?}", hd_key);
    println!("HDKey public key: {}", hd_key.extended_public_key_serialized()?);
    let bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    let expected_address = "32z7gdz9HL5PRbFQM8E6ar5xidHUxCfniW";
    assert_eq!(bitcoin_address.public_address(), expected_address);
    Ok(())
}