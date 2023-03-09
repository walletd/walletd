// use std::str::FromStr;

use thiserror::Error;
use walletd_bip39::{Language, Mnemonic, MnemonicHandler};
use walletd_coin_model::crypto_wallet::CryptoWallet;
use walletd_ethereum::*;
// use hex_literal::hex;
use walletd_hd_keys::HDKeyPair;
// use walletd_coin_model::CryptoWallet;
use walletd_hd_keys::NetworkType;

const GOERLI_TEST_ADDRESS: &str = "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
#[tokio::main]
async fn main() -> web3::Result<()> {
    // main_wip()?;

    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let passphrase: Option<&str> = Some("mypassphrase");
    let restored_mnemonic =
        Mnemonic::from_phrase(Language::English, mnemonic_phrase, passphrase).unwrap();
    let seed = restored_mnemonic.to_seed();

    println!("seed as bytes: {:?}", seed.as_bytes());

    let wallet = match EthereumWallet::from_mnemonic(
        &seed,
        NetworkType::TestNet,
        EthereumFormat::Checksummed,
    ) {
        Ok(wallet) => Ok(wallet),
        Err(e) => Err(e),
    };

    println!("wallet: {:?}", &wallet);
    Ok(())
}
