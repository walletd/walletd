extern crate walletd_ethereum;
// https://goerli.etherscan.io/block/8455626
use std::sync::Arc;

use ethers::prelude::*;
use ethers::{
    abi::Abi,
    types::{Address, H256},
};
use serde;
use walletd_coin_core::BlockchainConnector;
use walletd_ethereum::EthClient;
use serde_json::json;
use serde::{Deserialize, Serialize};

pub const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";


// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

#[tokio::main]
async fn main() {
    let ethclient_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
    let eth_client = EthClient::new(ethclient_url).unwrap();

    let eth_client = Arc::new(eth_client);

    // ETH/USDT pair on Uniswap V2
    let address = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852".parse::<Address>().unwrap();
    let pair = IUniswapV2Pair::new(address, Arc::clone(&eth_client.ethers()));

    // getReserves -> get_reserves
    let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await.unwrap();
    println!("Reserves (ETH, USDT): ({reserve0}, {reserve1})");

    let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;
    println!("ETH/USDT price: {mid_price:.2}");
}
