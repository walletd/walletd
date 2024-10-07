use walletd_monero_mnemonic::{Language, Mnemonic, MnemonicExt, MnemonicStyleBuilder};

fn main() -> () {
    // Restore a mnemonic struct from a phrase using the builder pattern
    println!("Example of restoring a mnemonic struct from a phrase using the builder pattern");
    let phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
    let mnemonic = Mnemonic::builder()
        .set_phrase(phrase)
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
    let phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
    let mnemonic = Mnemonic::from_phrase(Language::English, phrase, None).unwrap_or_else(|error| {
        panic!("Problem creating the mnemonic: {:?}", error);
    });
    println!("{}", mnemonic);
    // get the wallet seed
    let seed = mnemonic.to_seed();
    println!("{}", seed);
}
