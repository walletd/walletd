extern crate walletd_ethereum;
// https://goerli.etherscan.io/block/8455626
use web3::transports::Http;
pub const INFURA_GOERLI_ENDPOINT: &str =
    "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
use walletd_ethereum::ethclient::*;

use crate::ethclient::EthClient;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Transport can be one of Http, WebSocket, Ipc
    let transport = web3::transports::Http::new(INFURA_GOERLI_ENDPOINT)?;
    let eth_client = EthClient::new(transport, &INFURA_GOERLI_ENDPOINT.to_string());
    let bn = "8455626";
    let block_data = EthClient::get_block_by_number(&eth_client, &bn);

    let latest_block_data = EthClient::get_latest_block(&eth_client);
    Ok(())
}
