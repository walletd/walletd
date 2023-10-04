use bdk::keys::bip39::Mnemonic;
use walletd_ethereum::prelude::*;

#[tokio::main]
async fn main() {
    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let wallet = EthereumWallet::builder().mnemonic(mnemonic).build();

    println!("wallet: {:?}", &wallet);
}
