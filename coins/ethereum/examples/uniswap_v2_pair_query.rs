extern crate walletd_ethereum;
// https://goerli.etherscan.io/block/8455626
use std::sync::Arc;

pub const PROVIDER_URL: &str = "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";

use ethers::{
    contract::abigen,
    providers::{Http, Provider},
    types::Address,
};

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

abigen!(
    ERC20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
    ]"#
);

#[tokio::main]
async fn main() {
    let client = Provider::<Http>::try_from(PROVIDER_URL).unwrap();
    let client = Arc::new(client);

    // ETH/USDT pair on Uniswap V2
    let address = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852"
        .parse::<Address>()
        .unwrap();
    let pair = IUniswapV2Pair::new(address, Arc::clone(&client));

    // getReserves -> get_reserves
    let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await.unwrap();
    println!("Reserves (ETH, USDT): ({reserve0}, {reserve1})");

    let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;
    println!("ETH/USDT price: {mid_price:.2}");

    let address = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852"
        .parse::<Address>()
        .unwrap();

    let instance = ERC20::new(address, Arc::clone(&client));

    let balance = instance.balance_of(address).call().await.unwrap();

    println!("balance: {:?}", balance);

    
}
