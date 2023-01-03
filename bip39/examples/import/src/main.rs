use walletd_bip39::{ Language, Mnemonic, MnemonicHandler };

fn main() -> () {
  // create a new randomly generated mnemonic phrase
  let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
  let mnemonic = Mnemonic::from_phrase(Language::English, phrase, None).unwrap_or_else(|error| {
    panic!("Problem creating the mnemonic: {:?}", error);
  });
  println!("{:?}", mnemonic);
  // get the wallet seed
  let seed = mnemonic.to_seed();
  println!("{:?}", seed);
}
