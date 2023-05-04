use walletd_bip39::prelude::*;

fn main() -> Result<(), walletd_bip39::Error> {
    let mnemonic = Bip39Mnemonic::builder().build()?;
    // display the generated mnemonic phrase
    println!("mnemonic phrase: {}", mnemonic.phrase());
    // can use the hex format specifier to print the seed as hex
    println!("mnemonic seed hex: {:x}", mnemonic.to_seed());
    // can use the as_bytes method to get the seed as a byte array
    println!(
        "mnemonic seed as bytes: {:?}",
        mnemonic.to_seed().as_bytes()
    );
    assert_eq!(mnemonic.language(), Bip39Language::English);
    assert_eq!(mnemonic.mnemonic_type(), Bip39MnemonicType::Words12);
    let mut mnemonic_builder = Bip39Mnemonic::builder();
    // specify that the mnemonic phrase should consist of 24 words
    let mnemonic_1 = mnemonic_builder
        .mnemonic_type(Bip39MnemonicType::Words24)
        .build()?;
    println!("mnemonic_1 phrase: {}", mnemonic_1.phrase());
    println!("mnemonic_1 seed hex: {:x}", mnemonic_1.to_seed());
    assert_eq!(mnemonic_1.mnemonic_type(), Bip39MnemonicType::Words24);
    // see the number of entropy bits for the specified mnemonic type
    assert_eq!(mnemonic_1.mnemonic_type().entropy_bits(), 256);
    println!(
        "mnemonic_1 number of entropy bits: {}",
        mnemonic_1.mnemonic_type().entropy_bits()
    );
    // reuse builder but now specify 18 words in the mnemonic phrase
    let mnemonic_2 = mnemonic_builder
        .mnemonic_type(Bip39MnemonicType::Words18)
        .build()?;
    assert_eq!(mnemonic_2.mnemonic_type(), Bip39MnemonicType::Words18);
    assert_eq!(mnemonic_2.mnemonic_type().entropy_bits(), 192);
    println!("mnemonic_2 phrase: {}", mnemonic_2.phrase());
    println!("mnemonic_2 seed hex: {:x}", mnemonic_2.to_seed());
    println!(
        "mnemonic_2 number of entropy bits: {}",
        mnemonic_2.mnemonic_type().entropy_bits()
    );
    let mnemonic_3 = Bip39Mnemonic::builder()
        .passphrase("mypassphrase")
        .mnemonic_type(Bip39MnemonicType::Words12)
        .language(Bip39Language::English)
        .build()?;
    assert_eq!(mnemonic_3.mnemonic_type(), Bip39MnemonicType::Words12);
    assert_eq!(mnemonic_3.language(), Bip39Language::English);
    println!("mnemonic_3 phrase: {}", mnemonic_3.phrase());
    println!("mnemonic_3 seed hex: {:x}", mnemonic_3.to_seed());

    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    let restored_mnemonic_1 = Bip39Mnemonic::builder()
        .mnemonic_phrase(mnemonic_phrase)
        .build()?;
    assert_eq!(
        restored_mnemonic_1.mnemonic_type(),
        Bip39MnemonicType::Words12
    );
    assert_eq!(restored_mnemonic_1.language(), Bip39Language::English);
    assert_eq!(restored_mnemonic_1.phrase(), mnemonic_phrase);
    let seed_hex_1 = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    assert_eq!(format!("{:x}", restored_mnemonic_1.to_seed()), seed_hex_1);
    println!(
        "restored_mnemonic_1 phrase: {}",
        restored_mnemonic_1.phrase()
    );
    println!(
        "restored_mnemonic_1 seed hex: {:x}",
        restored_mnemonic_1.to_seed()
    );
    let specified_passphrase = "mypassphrase";
    let restored_mnemonic_2 = Bip39Mnemonic::builder()
        .mnemonic_phrase(mnemonic_phrase)
        .passphrase(specified_passphrase)
        .build()?;
    assert_eq!(
        restored_mnemonic_2.mnemonic_type(),
        Bip39MnemonicType::Words12
    );
    assert_eq!(restored_mnemonic_2.language(), Bip39Language::English);
    assert_eq!(restored_mnemonic_2.phrase(), mnemonic_phrase);
    let seed_hex_2 = "3c536b023d71d81e6abc58b0b91c64caff8bb08fabf0c9f3cf948a9f3a494e8ecb0790b6e933834796c930a2d437170bd6071c00bc0553d06235d02315f2c229";
    assert_eq!(format!("{:x}", restored_mnemonic_2.to_seed()), seed_hex_2);
    println!(
        "restored_mnemonic_2 phrase: {}",
        restored_mnemonic_2.phrase()
    );
    println!(
        "restored_mnemonic_2 seed hex: {:x}",
        restored_mnemonic_2.to_seed()
    );

    Ok(())
}
