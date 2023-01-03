use walletd_bip39::{ Language, Mnemonic, MnemonicType, MnemonicHandler };

fn main() -> () {
  let test = format!("{}", Language::English);
  println!("{}", test);
  // create a new randomly generated mnemonic phrase
  let passphrase: &str = "mypassphrase";
  let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words12, Some(passphrase));
  // get the wallet seed
  let seed = mnemonic.to_seed();
  println!("{:?}", seed);
}
