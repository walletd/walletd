#![allow(unused_imports)]
use walletd_rates::{Bitstamp, Error, ExchangeRateHostCrypto};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let result = ExchangeRateHostCrypto::get_rate("BTC", "USD").await?;
    dbg!(result);

    let result = Bitstamp::get_rate("BTC", "USD").await?;
    dbg!(result);
    Ok(())
}
