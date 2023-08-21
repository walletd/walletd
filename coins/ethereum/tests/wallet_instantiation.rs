use bdk::keys::bip39::Mnemonic;
use walletd_ethereum::EthereumWallet;
use walletd_hd_key::HDNetworkType;

#[test]
fn test_wallet_instantiation_from_mnemonic_seed() {
    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let wallet = EthereumWallet::builder()
        .mnemonic(mnemonic)
        .network_type(HDNetworkType::TestNet)
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

    // assert!(&wallet.private_key().is_err());
}
