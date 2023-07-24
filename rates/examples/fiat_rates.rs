use walletd_rates::{ExchangeRateApi, ExchangeRateHost};

#[tokio::main]
async fn main() {
    let result = ExchangeRateApi::new("78b121ef7b842f85c127c1ab")
        .get_rate()
        .await
        .unwrap();
    dbg!(result);

    // let result = ExchangeRateHost::new().get_rate().await.unwrap();
    // dbg!(result);
    //walletd_rates::get_rate2().await;
}
