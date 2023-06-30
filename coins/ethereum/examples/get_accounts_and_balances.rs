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
    let mnemonic_phrase: &str =
    "mandate rude write gather vivid inform leg swift usual early bamboo element";
    let restored_mnemonic = Bip39Mnemonic::builder()
        .mnemonic_phrase(mnemonic_phrase)
        .detect_language()
        .build()
        .unwrap();
    let eth_client = EthClient::new(INFURA_GOERLI_ENDPOINT).unwrap();
    let address: H160 = "00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap();

    let seed = restored_mnemonic.to_seed();

    let _eth_client = EthClient::new(INFURA_GOERLI_ENDPOINT);

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
    let balance = &blockchain_client.ethers().get_balance(from, None).await.unwrap();
    print!("balance_before: {:?}", &balance_before);

    // Not that we need to, but we can determine the nonce manually if we want
    let nonce= &blockchain_client.ethers().get_transaction_count(from, None).await.unwrap();
    print!("nonce: {:?}", &nonce);

    println!(
        "ethereum wallet balance: {} ETH, ({} wei)",
        balance.eth(),
        balance.wei()
    );

    ()
}