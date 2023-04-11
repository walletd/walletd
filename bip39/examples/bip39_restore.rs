use walletd_bip39::{Language, Mnemonic, MnemonicHandler, MnemonicStyleBuilder};

fn main() -> () {
    // Restore a mnemonic struct from a phrase using the builder pattern
    println!("Example of restoring a mnemonic struct from a phrase using the builder pattern");
    let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
    // if a passphrase was used when the mnemonic was created, it must be set here,
    // this example assumes no passphrase was used
    let mnemonic = Mnemonic::builder()
        .with_phrase(phrase)
        .restore()
        .unwrap_or_else(|error| {
            panic!("Problem creating the mnemonic: {:?}", error);
        });
    println!("{}", mnemonic);
    // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{}", seed);

    // Restore a mnemonic struct from a phrase without using the builder pattern
    println!(
        "Example of restoring a mnemonic struct from a phrase without using the builder pattern"
    );
    let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::from_phrase(Language::English, phrase, None).unwrap();
    println!("{}", mnemonic);
    // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{}", seed);
}
