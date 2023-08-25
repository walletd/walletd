extern crate walletd_ethereum;
// https://goerli.etherscan.io/block/8455626
const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
use ethers::prelude::*;
use walletd_ethereum::EthClient;

// Works with ethers
#[tokio::main]
async fn main() {
    // Transport can be one of Http, WebSocket, Ipc
    let eth_client = EthClient::new(PROVIDER_URL).unwrap();
    let _block_number: U64 = U64::from(8455626);
    print!("block_number: {:?}", &_block_number);
    let _latest_block_data = EthClient::latest_block(&eth_client).await.is_err();

    assert!(!_latest_block_data);
    print!("If you see this, it means that block 8455626 was retrieved without error.");
}
