use walletd_monero_mnemonic::{
    Language, Mnemonic, MnemonicExt, MnemonicStyleBuilder, MnemonicType,
};

fn main() -> () {
    // example of how to generate a mnemonic phrase using the builder pattern
    println!("Example of generating a mnemonic phrase using the builder pattern");
    let test = format!("{}", Language::English);
    println!("{}", test);
    // create a new randomly generated mnemonic phrase
    // W
    let mnemonic = Mnemonic::builder().generate().unwrap();
    println!("{}", mnemonic); // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{}", seed);

    // example of how to generate a mnemonic phrase without using the builder
    // pattern
    println!(
        "Example of restoring a mnemonic struct from a phrase without using the builder pattern"
    );
    let test = format!("{}", Language::English);
    println!("{}", test);
    // create a new randomly generated mnemonic phrase
    let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words25, None);
    println!("{}", mnemonic); // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{}", seed);
}
