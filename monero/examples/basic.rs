use walletd_monero_mnemonic::{Language, Mnemonic, MnemonicHandler, MnemonicType};

fn main() -> () {
    let test = format!("{}", Language::English);
    println!("{}", test);
    // create a new randomly generated mnemonic phrase
    let passphrase: &str = "mypassphrase";
    let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words25, Some(passphrase));
    println!("{:?}", mnemonic);
    // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{:?}", seed);
}
