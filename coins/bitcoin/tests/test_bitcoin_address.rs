use std::str::FromStr;
use walletd_bitcoin::{BitcoinAddress, Seed, Error, HDKey, CryptoAddress, HDNetworkType, AddressType};

#[test]
fn test_from_hd_key() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex)?;
    let address_format = AddressType::P2wpkh;
    let hd_key = HDKey::new(seed, HDNetworkType::TestNet, "m/84'/1'/0'/0/0".to_string())?;
    let bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    let expected_address = "tb1q2knvzpjltz4uwh6j5wrmqn7lnzccsphpd85jp9";
    assert_eq!(bitcoin_address.public_address(), expected_address);
    Ok(())
}