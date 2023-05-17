use super::*;
use walletd_hd_key::prelude::{FromStr, Seed};
const BTC_WALLET_TEST_SEED: &str = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";

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
    let seed = Seed::from_str(BTC_WALLET_TEST_SEED)?;
    let address_format = AddressType::P2wpkh;
    let hd_key = HDKey::new(seed, HDNetworkType::TestNet, "m/84'/1'/0'/0/0")?;
    let bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    let associated = AssociatedAddress::new(bitcoin_address.clone(), hd_key.clone());
    assert_eq!(associated.address(), &bitcoin_address);
    assert_eq!(associated.hd_key(), &hd_key);
    Ok(())
}

#[test]
fn test_bitcoin_wallet() -> Result<(), Error> {
    let mut btc_wallet = BitcoinWallet::default();
    assert_eq!(btc_wallet.address_format(), AddressType::P2wpkh);
    assert_eq!(btc_wallet.default_hd_purpose()?, HDPurpose::BIP84);
    let master_hd_key = HDKey::new_master(
        Seed::from_str(BTC_WALLET_TEST_SEED)?,
        HDNetworkType::TestNet,
    )?;
    let derived_hd_key = master_hd_key.derive("m/84'/1'/0'/0/0")?;
    let first_address_hd_key = HDKey::new(
        Seed::from_str(BTC_WALLET_TEST_SEED)?,
        HDNetworkType::TestNet,
        "m/84'/1'/0'/0/0",
    )?;
    assert_eq!(derived_hd_key, first_address_hd_key);
    let expected_first_address = BitcoinAddress::from_hd_key(&derived_hd_key, AddressType::P2wpkh)?;
    btc_wallet.set_master_hd_key(master_hd_key);
    assert_eq!(btc_wallet.network()?, Network::Testnet);
    assert_eq!(btc_wallet.coin_type_id()?, 1);
    assert_eq!(btc_wallet.gap_limit(), 20);
    assert_eq!(btc_wallet.account_discovery(), true);
    btc_wallet.set_account_discovery(false);
    assert_eq!(btc_wallet.account_discovery(), false);
    btc_wallet.set_gap_limit(10);
    assert_eq!(btc_wallet.gap_limit(), 10);
    assert_eq!(btc_wallet.addresses().len(), 0);
    btc_wallet.add_address_index(0)?;
    assert_eq!(
        btc_wallet.associated_info()[0].address(),
        &expected_first_address
    );
    assert_eq!(btc_wallet.addresses().len(), 1);
    let next_address = btc_wallet.next_address()?;
    btc_wallet.add_address_index(1)?;
    assert_eq!(&next_address, btc_wallet.associated_info()[1].address());

    // test zeroize
    btc_wallet.zeroize();
    assert_eq!(btc_wallet.addresses().len(), 0);
    Ok(())
}

#[test]
fn test_default_builder() {
    let btc_builder = BitcoinWallet::builder();
    let default_btc_builder = BitcoinWalletBuilder::default();
    assert_eq!(btc_builder, default_btc_builder);
}
