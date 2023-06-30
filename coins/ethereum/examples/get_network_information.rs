extern crate walletd_ethereum;
// https://goerli.etherscan.io/block/8455626

pub const INFURA_GOERLI_ENDPOINT: &str =
    "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
use walletd_coin_core::BlockchainConnector;
use walletd_ethereum::EthClient;

// Works with ethers.js
#[tokio::main]
async fn main() -> web3::Result<()> {
    // Transport can be one of Http, WebSocket, Ipc
    // let transport = web3::transports::Http::new(INFURA_GOERLI_ENDPOINT)?;

    let ethclient_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
    let eth_client = EthClient::new(ethclient_url)?;
    let tx_hash = "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b";
    let tx = eth_client.transaction_data_from_hash(tx_hash).await?;
    let block_number = eth_client.current_block_number().await;
    let gas_price = eth_client.gas_price().await;
    let balance = ethereum_wallet.balance().await?;

    println!("Block number: {:#?}", block_number);
    println!("Gas price: {:#?}", gas_price);
    println!("transaction data: {:?}", tx);
    //ethereum_wallet.set_blockchain_client(eth_client);
    
    Ok(())
}
