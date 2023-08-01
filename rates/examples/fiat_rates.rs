#![allow(unused_imports)]
use walletd_rates::Error;
use walletd_rates::{ExchangeRateApi, ExchangeRateHost};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // replace "your_api_key" with your api key from exchangerate-api.com
    // let result = ExchangeRateApi::new("your_api_key").get_rate().await?;
    // dbg!(result);

    // fetches the current fiat rates from exchangerate.host
    let result = ExchangeRateHost::new().get_rate().await?;
    dbg!(result);
    Ok(())
}
