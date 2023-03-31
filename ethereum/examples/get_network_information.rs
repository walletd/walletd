extern crate walletd_ethereum;
// https://goerli.etherscan.io/block/8455626

pub const INFURA_GOERLI_ENDPOINT: &str =
    "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
use walletd_ethereum::EthClient;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Transport can be one of Http, WebSocket, Ipc
    // let transport = web3::transports::Http::new(INFURA_GOERLI_ENDPOINT)?;
    let eth_client = EthClient::new(&INFURA_GOERLI_ENDPOINT.to_string());

    let block_number = eth_client.current_block_number().await;
    let gas_price = eth_client.gas_price().await;

    println!("Block number: {:#?}", block_number);
    println!("Gas price: {:#?}", gas_price);

    Ok(())
}
