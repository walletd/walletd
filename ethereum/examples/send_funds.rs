// EthereumAmount.new_from_eth(u64)
// use std::str::FromStr;

use walletd_bip39::{Language, Mnemonic, MnemonicHandler};
use walletd_coin_model::crypto_wallet::CryptoWallet;
use walletd_coin_model::BlockchainConnector;
use walletd_ethereum::*;
// use hex_literal::hex;
// use walletd_hd_key::HDKey;
// use walletd_coin_model::CryptoWallet;
use walletd_hd_key::NetworkType;

// const GOERLI_TEST_ADDRESS: &str =
// "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
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

    let blockchain_client = walletd_ethereum::BlockchainClient::new(
        "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161",
    );

    println!("blockchain_client: {:?}", &blockchain_client);
    let _wallet = match EthereumWallet::from_mnemonic(
        &seed,
        NetworkType::TestNet,
        EthereumFormat::Checksummed) 
    {
        Ok(wallet) => Ok(wallet),
        Err(e) => Err(e),
        Err(e) => Err(e),
    };

    // This example now assumes that the wallet has been funded with some testnet
    // ETH let sa = 10000.0;
    // let send_amount: EthereumAmount::new_from_eth(sa);
    // println!("send_amount: {:?}", &send_amount);

    // println!("wallet: {:?}", &wallet);
    Ok(())
}
