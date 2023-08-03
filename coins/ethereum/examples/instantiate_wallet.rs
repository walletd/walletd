use bdk::keys::bip39::Mnemonic;
use walletd_ethereum::prelude::*;
use walletd_hd_key::HDNetworkType;

#[tokio::main]
async fn main() {
    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
    let seed = mnemonic.to_seed("");
    let seed = Seed::new(seed.to_vec());

    println!("seed as bytes: {:?}", seed.as_bytes());

    let wallet = EthereumWallet::builder()
        .mnemonic_seed(seed)
        .network_type(HDNetworkType::TestNet)
        .build();

    println!("wallet: {:?}", &wallet);
}
