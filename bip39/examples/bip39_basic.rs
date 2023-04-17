use walletd_bip39::{Language, Mnemonic, MnemonicHandler, MnemonicStyleBuilder, MnemonicType};

fn main() -> () {
    // example of how to generate a mnemonic phrase using the builder pattern
    println!("Example of generating a mnemonic phrase using the builder pattern");
    let test = format!("{}", Language::English);
    println!("{}", test);
    // create a new randomly generated mnemonic phrase
    let passphrase: &str = "mypassphrase"; // this is an optional passphrase, if you don't to use a passphrase, you don't
                                           // need to use the .set_passphrase() method
                                           // The default language is English, so we don't need to specify it here
                                           // The default mnemonic type is Words12, if we don't specify anything for
                                           // mnemonic type we will get a 12 word mnemonic phrase here
    let mnemonic = Mnemonic::builder()
        .with_passphrase(passphrase)
        .generate()
        .expect("should be valid mnemonic");
    println!("{}", mnemonic);
    // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{}", seed);

    // example of how to generate a mnemonic phrase without using the builder
    // pattern
    println!("Example of generating a mnemonic phrase without using the builder pattern");
    let test = format!("{}", Language::English);
    println!("{}", test);
    // create a new randomly generated mnemonic phrase
    let passphrase: &str = "mypassphrase";
    let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words12, Some(passphrase)); // If you don't want to use a passphrase, you can use None instead of
                                                                                              // Some(passphrase)
    println!("{}", mnemonic);
    // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{}", seed);
}
