use bdk::keys::bip39::Mnemonic;
use walletd_ethereum::EthereumWallet;

#[test]
fn test_wallet_instantiation_from_mnemonic_seed() {
    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let wallet = EthereumWallet::builder()
        .mnemonic(mnemonic)
        .build()
        .unwrap();

    assert_eq!(
        &wallet.public_address(),
        "0x6EEb11eA2905fEe101f72BF94F792dbc2dfB42B7"
    );
}
