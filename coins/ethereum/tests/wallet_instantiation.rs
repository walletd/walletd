use std::str::FromStr;
use walletd_bip39::Seed;
use walletd_coin_model::{CryptoWallet, CryptoWalletBuilder};
use walletd_ethereum::EthereumWallet;
use walletd_hd_key::HDNetworkType;

#[test]
fn test_wallet_instantiation_from_mnemonic_seed() {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex).unwrap();
    let wallet = EthereumWallet::builder()
        .with_mnemonic_seed(seed)
        .with_network_type(HDNetworkType::TestNet)
        .build()
        .unwrap();

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
