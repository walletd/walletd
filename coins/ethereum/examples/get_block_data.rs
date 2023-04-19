extern crate walletd_ethereum;
// https://goerli.etherscan.io/block/8455626
pub const INFURA_GOERLI_ENDPOINT: &str =
    "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
use walletd_coin_model::BlockchainConnector;
use walletd_ethereum::EthClient;
use web3::types::U64;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Transport can be one of Http, WebSocket, Ipc
    let eth_client = EthClient::new(&INFURA_GOERLI_ENDPOINT.to_string()).unwrap();
    let _block_number: U64 = U64::from(8455626);

    let _latest_block_data = EthClient::latest_block(&eth_client);
    Ok(())
}
