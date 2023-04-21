use std::str::FromStr;
use walletd::{Error, KeyPairBuilder, MnemonicKeyPairType, Seed};
use walletd_hd_key::HDNetworkType;

#[test]
fn test_new_and_default() {
    let builder = KeyPairBuilder::new();
    let default = KeyPairBuilder::default();
    assert_eq!(builder, default);
    assert!(builder.mnemonic_phrase.is_none());
    assert!(builder.mnemonic_seed.is_none());
    assert!(builder.passphrase.is_none());
    assert_eq!(builder.network_type, HDNetworkType::default());
    assert_eq!(builder.style, MnemonicKeyPairType::default());
}

#[test]
fn test_with_mnemonic_phrase() -> Result<(), Error> {
    let mnemonic_phrase =
        "outer ride neither foil glue number place usage ball shed dry point".to_string();
    let mut builder = KeyPairBuilder::new();
    builder.with_mnemonic_phrase(mnemonic_phrase.clone());
    let keypair = builder.clone().build()?;
    assert!(builder.clone().mnemonic_phrase.is_some());

    assert_eq!(
        builder
            .clone()
            .mnemonic_phrase
            .expect("due to previous check, should be some"),
        mnemonic_phrase
    );
    assert_eq!(keypair.mnemonic_phrase, mnemonic_phrase);

    let builder_phrase_none = builder.set_mnemonic_phrase_none();

    assert!(builder_phrase_none.clone().mnemonic_phrase.is_none());
    let keypair_build = builder_phrase_none.build();
    assert!(keypair_build.is_err());
    Ok(())
}

#[test]
fn test_with_mnemonic_seed() -> Result<(), Error> {
    let mnemonic_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
    let mut builder = KeyPairBuilder::new();
    builder.with_mnemonic_seed(mnemonic_seed.clone());
    assert!(builder.clone().mnemonic_seed.is_some());
    assert_eq!(
        builder
            .clone()
            .mnemonic_seed
            .expect("due to previous check, should be some"),
        mnemonic_seed
    );
    let builder_no_seed = builder.set_mnemonic_seed_none();
    assert!(builder_no_seed.mnemonic_seed.is_none());
    Ok(())
}

#[test]
fn test_with_passphrase() -> Result<(), Error> {
    let passphrase = "mypassphrase".to_string();
    let mut builder = KeyPairBuilder::new();
    builder.with_passphrase(passphrase.clone());
    assert!(builder.clone().passphrase.is_some());
    assert_eq!(
        builder
            .clone()
            .passphrase
            .expect("due to previous check, should be some"),
        passphrase
    );
    let builder_no_passphrase = builder.set_passphrase_none();
    assert!(builder_no_passphrase.passphrase.clone().is_none());
    Ok(())
}

#[test]
fn test_with_network_type() {
    let network_type = HDNetworkType::TestNet;
    let mut builder = KeyPairBuilder::new();
    builder.with_network_type(network_type);
    assert_eq!(builder.network_type, network_type);
}

#[test]
fn test_with_style() {
    let style = MnemonicKeyPairType::HDBip39;
    let mut builder = KeyPairBuilder::new();
    builder.with_style(style);
    assert_eq!(builder.style, style);
}
