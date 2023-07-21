abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

// async fn get_uniswap_pair_liquidity(&self, pair_address: Address) {
//     let client = &self.ethers();
//     let client = Arc::new(client);

//     // ETH/USDT pair on Uniswap V2 (Mainnet)
//     let address = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852".parse::<Address>().unwrap();
//     let pair = IUniswapV2Pair::new(address, Arc::clone(&client));

//     // getReserves -> get_reserves
//     let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await.unwrap();
//     println!("Reserves (ETH, USDT): ({reserve0}, {reserve1})");

//     let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;
//     println!("ETH/USDT price: {mid_price:.2}");

// }