extern crate walletd_ethereum;

use walletd_bip39::{Bip39Language, Bip39Mnemonic, MnemonicExt};
use walletd_coin_core::{CryptoWallet, CryptoWalletBuilder};
use walletd_ethereum::EthereumWallet;
use walletd_hd_key::HDNetworkType;

// const GOERLI_TEST_ADDRESS: &str =
// "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
#[tokio::main]
async fn main() -> web3::Result<()> {
    // main_wip()?;

    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let passphrase: Option<&str> = Some("mypassphrase");
    let restored_mnemonic =
        Bip39Mnemonic::from_phrase(Bip39Language::English, mnemonic_phrase, passphrase).unwrap();
    let seed = restored_mnemonic.to_seed();

    println!("seed as bytes: {:?}", seed.as_bytes());

    let wallet = match EthereumWallet::builder()
        .mnemonic_seed(seed)
        .network_type(HDNetworkType::TestNet)
        .build()
    {
        Ok(wallet) => Ok(wallet),
        Err(e) => Err(e),
    };

    println!("wallet: {:?}", &wallet);
    Ok(())
}
