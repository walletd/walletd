extern crate walletd_ethereum;
// https://goerli.etherscan.io/block/8455626

use ethers::prelude::*;

use walletd_coin_core::BlockchainConnector;
use walletd_ethereum::EthClient;

pub const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";

#[tokio::main]
async fn main() {
    // Transport can be one of Http, WebSocket, Ipc
    // let transport = web3::transports::Http::new(PROVIDER_URL)?;

    let ethclient_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
    let eth_client = EthClient::new(ethclient_url).unwrap();

    let block_data = eth_client
        .ethers()
        .get_block_with_txs(ethers::types::BlockId::Number(
            ethers::types::BlockNumber::Latest,
        ))
        .await
        .unwrap()
        .unwrap();

    let output_block_data = block_data.clone();
    for i in 0..10 {
        println!("{:?}", output_block_data.transactions[i]);
    }
    // Ok(output_block_data)

    let tx_hash = output_block_data.transactions[0].hash;
    let tx_data = eth_client
        .get_transaction_data_from_tx_hash(tx_hash)
        .await
        .unwrap();

    println!("tx_data: {:?}", tx_data);
}
