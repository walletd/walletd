use std::str::FromStr;
use walletd_hd_key::{HDKey, HDNetworkType, Seed};

fn main() -> () {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    println!("seed_hex: {}", seed_hex);

    let keys =
        HDKey::new_master(Seed::from_str(&seed_hex).unwrap(), HDNetworkType::MainNet).unwrap();
    println!("{:#?}", keys);

    println!("wif of master hd key {}", keys.to_wif().unwrap());
}
