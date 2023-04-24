use crate::{
    BitcoinWallet, Error, EthereumWallet, HDNetworkType, KeyPair, MnemonicKeyPairType, Seed,
};
use std::str::FromStr;

#[test]
fn test_keypair() -> Result<(), Error> {
    let mnemonic_phrase =
        "outer ride neither foil glue number place usage ball shed dry point".to_string();
    let mnemonic_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
    let passphrase = "mypassphrase";
    let network = HDNetworkType::TestNet;
    let keypair = KeyPair::builder()
        .with_mnemonic_phrase(mnemonic_phrase.clone())
        .with_mnemonic_seed(mnemonic_seed.clone())
        .with_passphrase(passphrase.to_string())
        .with_network_type(network)
        .build()?;

    assert_eq!(keypair.mnemonic_phrase(), mnemonic_phrase);
    assert_eq!(keypair.mnemonic_seed(), mnemonic_seed);
    assert_eq!(keypair.passphrase(), Some(passphrase));
    assert_eq!(keypair.network_type(), network);
    assert_eq!(keypair.style(), MnemonicKeyPairType::HDBip39);

    let keypair_new = KeyPair::new(
        mnemonic_seed.clone(),
        mnemonic_phrase.clone(),
        MnemonicKeyPairType::HDBip39,
        Some(passphrase),
        network,
    );

    assert_eq!(keypair, keypair_new);

    // Test deriving a BitcoinWallet from the KeyPair
    let bitcoin_wallet_result = keypair.derive_wallet::<BitcoinWallet>();
    assert!(bitcoin_wallet_result.is_ok());

    // Test deriving a EthereumWallet from the KeyPair
    let ethereum_wallet_result = keypair.derive_wallet::<EthereumWallet>();
    assert!(ethereum_wallet_result.is_ok());

    Ok(())
}
