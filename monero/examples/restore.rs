use walletd_monero_mnemonic::{Language, Mnemonic, MnemonicHandler};

fn main() -> () {
    // create a new randomly generated mnemonic phrase
    let phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
    let mnemonic = Mnemonic::from_phrase(Language::English, phrase, None).unwrap_or_else(|error| {
        panic!("Problem creating the mnemonic: {:?}", error);
    });
    println!("{:?}", mnemonic);
    // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{:?}", seed);
}
