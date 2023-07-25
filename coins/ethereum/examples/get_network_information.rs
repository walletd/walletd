extern crate walletd_ethereum;
// https://goerli.etherscan.io/block/8455626

pub const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
use walletd_coin_core::BlockchainConnector;
use walletd_ethereum::EthClient;

#[tokio::main]
async fn main() {
    let ethclient_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
    let eth_client = EthClient::new(ethclient_url).unwrap();
    let tx_hash = "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b";
    let tx_hash = tx_hash.parse().unwrap();
    println!("tx_hash: {:?}", tx_hash);
    let tx = eth_client
        .get_transaction_data_from_tx_hash(tx_hash)
        .await
        .unwrap();
    let block_number = eth_client.current_block_number().await;
    let gas_price = eth_client.gas_price().await;

    println!("Latest block number: {:#?}", block_number);
    println!("Gas price: {:#?}", gas_price);
    println!("transaction data: {:?}", tx);
}
