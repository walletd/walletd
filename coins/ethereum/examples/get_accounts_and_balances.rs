// EthereumAmount.new_from_eth(u64)
// use std::str::FromStr;

use walletd_bip39::{Bip39Mnemonic, Mnemonic, MnemonicBuilder};

use walletd_coin_core::{BlockchainConnector, CryptoWallet, CryptoWalletBuilder};
use walletd_ethereum::{EthClient, EthereumAmount, EthereumWallet};
use walletd_hd_key::HDNetworkType;


use ethers::{
    core::{types::TransactionRequest, utils::Anvil},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address
};
//use eyre::Result;
use std::convert::TryFrom;
const GOERLI_TEST_ADDRESS: &str = "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
#[tokio::main]
async fn main() -> () {
    println!("Main");

    let mnemonic_phrase: &str =
        "mandate rude write gather vivid inform leg swift usual early bamboo element";
    let restored_mnemonic = Bip39Mnemonic::builder()
        .mnemonic_phrase(mnemonic_phrase)
        .detect_language()
        .build()
        .unwrap();

    let seed = restored_mnemonic.to_seed();

    println!("seed as bytes: {:?}", seed.as_bytes());

    let blockchain_client =
        EthClient::new("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161").unwrap();

    println!("blockchain_client: {:?}", &blockchain_client);

    let mut wallet = EthereumWallet::builder()
        .mnemonic_seed(seed)
        .network_type(HDNetworkType::TestNet)
        .build()
        .unwrap();

    
    let from: Address = wallet.public_address().as_str().parse().unwrap();
    print!("from: {:?}", &from);
    let balance_before = &blockchain_client.ethers().get_balance(from, None).await.unwrap();
    print!("balance_before: {:?}", &balance_before);

    // Not that we need to, but we can determine the nonce manually
    let nonce= &blockchain_client.ethers().get_transaction_count(from, None).await.unwrap();
    print!("nonce: {:?}", &nonce);
   

    let to: Address = GOERLI_TEST_ADDRESS.parse().unwrap();
    let tx = TransactionRequest::new().to(to).value(1000).from(from); // specify the `from` field so that the client knows which account to use

    let tx = blockchain_client.ethers().send_transaction(tx, None).await.unwrap().await.unwrap();
    
    println!("tx: {:?}", &tx);
   
    let nonce2 = blockchain_client.ethers().get_transaction_count(from, None).await.unwrap();
    
    // let sa = ethers::types::U256::from(10000);
    // let send_amount = EthereumAmount::from_wei(sa);
    //assert!(nonce < *nonce2);

    let balance_after = blockchain_client.balance(from).await.unwrap();
    //assert!(balance_after < balance_before);

    println!("Balance before {balance_before}");
    // let tx_hash = wallet
    //     .transfer(&send_amount, GOERLI_TEST_ADDRESS)
    //     .await
    //     .unwrap();

    return ();
}