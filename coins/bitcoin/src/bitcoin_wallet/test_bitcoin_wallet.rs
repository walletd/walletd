use super::*;
use walletd_hd_key::prelude::{FromStr, Seed};
const BTC_WALLET_TEST_SEED: &str = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";

#[test]
fn test_default() -> Result<(), Error> {
    let expected_default = BitcoinWallet {
        wallet: None,
        address_format: AddressType::P2wpkh,
    };
    let wallet = BitcoinWallet::default();
    assert_eq!(wallet.address_format, expected_default.address_format);
    Ok(())
}

#[test]
fn test_bitcoin_wallet() -> Result<(), Error> {
    let btc_wallet = BitcoinWallet::default();
    assert_eq!(btc_wallet.address_format(), AddressType::P2wpkh);
    assert_eq!(btc_wallet.default_hd_purpose()?, HDPurpose::BIP84);
    assert_eq!(btc_wallet.network()?, Network::Testnet);
    assert_eq!(btc_wallet.coin_type_id()?, 1);

    Ok(())
}

#[test]
fn test_default_builder() {
    let btc_builder = BitcoinWallet::builder();
    let default_btc_builder = BitcoinWalletBuilder::default();
    assert_eq!(btc_builder, default_btc_builder);
}
