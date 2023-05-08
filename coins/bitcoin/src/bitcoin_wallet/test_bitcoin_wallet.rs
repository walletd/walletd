use crate::{AddressType, BitcoinAddress, BitcoinWallet, Error, Network};
use walletd_hd_key::prelude::*;

use super::AssociatedAddress;

#[test]
fn test_default() -> Result<(), Error> {
    let expected_default = BitcoinWallet {
        associated: Vec::new(),
        blockchain_client: None,
        address_format: AddressType::P2wpkh,
        master_hd_key: None,
        gap_limit: 20,
        account_discovery: true,
        hd_path_builder: None,
    };
    let wallet = BitcoinWallet::default();
    assert_eq!(wallet.address_format, expected_default.address_format);
    assert_eq!(wallet.associated, expected_default.associated);
    assert_eq!(
        wallet.blockchain_client.is_none(),
        expected_default.blockchain_client.is_none()
    );
    assert_eq!(
        wallet.master_hd_key.is_none(),
        expected_default.master_hd_key.is_none()
    );
    assert_eq!(wallet.gap_limit, expected_default.gap_limit);
    assert_eq!(wallet.account_discovery, expected_default.account_discovery);
    assert_eq!(
        wallet.hd_path_builder.is_none(),
        expected_default.hd_path_builder.is_none()
    );
    Ok(())
}

#[test]
fn test_associated_address() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex)?;
    let address_format = AddressType::P2wpkh;
    let hd_key = HDKey::new(seed, HDNetworkType::TestNet, "m/84'/1'/0'/0/0".to_string())?;
    let bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    let associated = AssociatedAddress::new(bitcoin_address.clone(), hd_key.clone());
    assert_eq!(associated.address(), &bitcoin_address);
    assert_eq!(associated.hd_key(), &hd_key);
    Ok(())
}
