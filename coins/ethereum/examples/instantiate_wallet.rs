extern crate walletd_ethereum;

use walletd_bip39::{Bip39Language, Bip39Mnemonic, Mnemonic};
use walletd_ethereum::EthereumWallet;
use walletd_hd_key::HDNetworkType;

#[tokio::main]
async fn main() {
    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let passphrase: Option<&str> = Some("mypassphrase");
    let restored_mnemonic =
        Bip39Mnemonic::from_phrase(Bip39Language::English, mnemonic_phrase, passphrase).unwrap();
    let seed = restored_mnemonic.to_seed();

    println!("seed as bytes: {:?}", seed.as_bytes());

    let wallet = EthereumWallet::builder()
        .mnemonic_seed(seed)
        .network_type(HDNetworkType::TestNet)
        .build();

    println!("wallet: {:?}", &wallet);
}
