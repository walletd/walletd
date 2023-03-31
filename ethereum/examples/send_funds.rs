// EthereumAmount.new_from_eth(u64)
// use std::str::FromStr;

use walletd_bip39::{Mnemonic, MnemonicHandler, MnemonicStyleBuilder};
use web3::types::U256;

use walletd_coin_model::BlockchainConnector;
use walletd_ethereum::*;
use walletd_hd_key::HDNetworkType;
use walletd_coin_model::CryptoWallet;

const GOERLI_TEST_ADDRESS: &str =
"0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
#[tokio::main]
async fn main()  {
    // main_wip()?;

    let mnemonic_phrase: &str =
        "joy tail arena mix other envelope diary achieve short nest true vocal";
    let restored_mnemonic =
        Mnemonic::builder().set_phrase(mnemonic_phrase).detect_language().restore().unwrap();

    let seed = restored_mnemonic.to_seed();

    println!("seed as bytes: {:?}", seed.as_bytes());

    let blockchain_client = walletd_ethereum::BlockchainClient::new(
        "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161",
    ).unwrap();

    println!("blockchain_client: {:?}", &blockchain_client);
    let wallet = EthereumWallet::from_mnemonic(
        &seed,
        HDNetworkType::TestNet,
        EthereumFormat::Checksummed, Some(blockchain_client)).unwrap();

    // This example now assumes that the wallet has been funded with some testnet ETH 
    println!("wallet: {:#?}", &wallet);
    
    println!("balance: {:?}", &wallet.balance().await.unwrap());

    let sa = U256::from(10000);
    let send_amount = EthereumAmount::new_from_wei(sa);
    println!("send_amount: {:?}", &send_amount);

    let tx_hash = wallet.transfer(
        &send_amount,
        GOERLI_TEST_ADDRESS,
    ).await.unwrap();

    println!("tx_hash: 0x{}", &tx_hash);
}
