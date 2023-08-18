use karl_solana::solanaclient::SolanaClient;

// // https://goerli.etherscan.io/block/8455626
// // const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
// // use ethers::prelude::*;
// // use walletd_coin_core::BlockchainConnector;
// // use walletd_ethereum::EthClient;
// use karl_solana::SolanaClient;
// // Works with ethers
// #[tokio::main]
// async fn main() {
//     let solana_client = SolanaClient::new(PROVIDER_URL).unwrap();
//     println!("{:#?}", solana_client);
//     // let _block_number: U64 = U64::from(8455626);
//     // print!("block_number: {:?}", &_block_number);
//     // let _latest_block_data = EthClient::latest_block(&eth_client).await.is_err();

//     // assert!(!_latest_block_data);
//     // print!("If you see this, it means that block 8455626 was retrieved without error.");
// }

const URL: &str = "http://127.0.0.1:8899";
#[tokio::main]
async fn main() {
    let solana_client = SolanaClient::new(URL).unwrap();
    println!("Ok, invoked");
    let latest_blockhash = solana_client.rpc_client().get_latest_blockhash().await.unwrap();
    println!("{:?}", latest_blockhash);

    let data_len = 300;
    let balance = solana_client.rpc_client().get_minimum_balance_for_rent_exemption(data_len).await.unwrap();
    println!("Minimum fee to be rent-exempt: {}", balance);
}