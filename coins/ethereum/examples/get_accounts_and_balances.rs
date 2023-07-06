use walletd_bip39::{Bip39Mnemonic, Mnemonic, MnemonicBuilder};

use walletd_coin_core::prelude::*;
use walletd_ethereum::{EthClient, EthereumAmount, EthereumWallet};
use walletd_hd_key::HDNetworkType;

use ethers::prelude::*;

const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
const GOERLI_TEST_ADDRESS: &str = "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
#[tokio::main]
async fn main() {
    let mnemonic_phrase: &str =
        "mandate rude write gather vivid inform leg swift usual early bamboo element";
    let restored_mnemonic = Bip39Mnemonic::builder()
        .mnemonic_phrase(mnemonic_phrase)
        .detect_language()
        .build()
        .unwrap();
    let _eth_client = EthClient::new(PROVIDER_URL).unwrap();
    let _address: H160 = GOERLI_TEST_ADDRESS.parse().unwrap();

    let seed = restored_mnemonic.to_seed();

    let _eth_client = EthClient::new(PROVIDER_URL);

    let blockchain_client = EthClient::new(PROVIDER_URL).unwrap();

    println!("blockchain_client: {:?}", &blockchain_client);

    let wallet = EthereumWallet::builder()
        .mnemonic_seed(seed)
        .network_type(HDNetworkType::TestNet)
        .build()
        .unwrap();

    let from: Address = wallet.public_address().as_str().parse().unwrap();
    print!("from: {:?}", &from);
    let balance = &blockchain_client
        .ethers()
        .get_balance(from, None)
        .await
        .unwrap();
    print!("balance: {:?}", &balance);

    let eth_amount: EthereumAmount = EthereumAmount::from_wei(*balance);
    println!(
        "ethereum wallet balance: {} ETH, ({} wei)",
        eth_amount.eth(),
        eth_amount.wei()
    );

    // Not that we need to, but we can determine the nonce manually if we want
    let nonce = &blockchain_client
        .ethers()
        .get_transaction_count(from, None)
        .await
        .unwrap();
    print!("nonce: {:?}", &nonce);
}
