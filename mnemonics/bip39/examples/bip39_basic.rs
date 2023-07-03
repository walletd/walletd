use walletd_bip39::{Bip39Language, Bip39Mnemonic, Bip39MnemonicType, Mnemonic, MnemonicBuilder};

fn main() {
    // example of how to generate a mnemonic phrase using the builder pattern
    println!("Example of generating a mnemonic phrase using the builder pattern");
    println!("{:?}", Bip39Language::English);
    // create a new randomly generated mnemonic phrase
    let passphrase: &str = "mypassphrase"; // this is an optional passphrase, if you don't to use a passphrase, you don't
                                           // need to use the .set_passphrase() method
                                           // The default language is English, so we don't need to specify it here
                                           // The default mnemonic type is Words12, if we don't specify anything for
                                           // mnemonic type we will get a 12 word mnemonic phrase here
    let mnemonic = Bip39Mnemonic::builder()
        .passphrase(passphrase)
        .generate()
        .expect("should be valid mnemonic");
    println!("{:?}", mnemonic);
    // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{}", seed);

    // example of how to generate a mnemonic phrase without using the builder
    // pattern
    println!("Example of generating a mnemonic phrase without using the builder pattern");
    println!("{:?}", Bip39Language::English);
    // create a new randomly generated mnemonic phrase
    let passphrase: &str = "mypassphrase";
    let mnemonic = Bip39Mnemonic::new(
        Bip39Language::English,
        Bip39MnemonicType::Words12,
        Some(passphrase),
    ); // If you don't want to use a passphrase, you can use None instead of
       // Some(passphrase)
    println!("{:?}", mnemonic);
    // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{}", seed);
}
