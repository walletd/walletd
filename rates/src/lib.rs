use std::collections::HashMap;

use serde::Deserialize;

pub async fn get_rate() {
    let body = reqwest::get("https://www.bitstamp.net/api/v2/ticker/btcusd/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // let block_count = body
    //     .parse::<u64>()
    //     .map_err(|e| Error::FromStr(e.to_string()))?;
    println!("{:?}", body);
}

#[derive(Deserialize, Debug)]
struct BaseResponse2 {
    result: String,
    documentation: String,
    terms_of_use: String,
    time_last_update_unix: u64,
    base_code: String,
    conversion_rates: HashMap<String, f32>,
}

pub async fn get_rate2() {
    let body =
        reqwest::get("https://v6.exchangerate-api.com/v6/78b121ef7b842f85c127c1ab/latest/USD")
            .await
            .unwrap()
            .json::<BaseResponse2>()
            .await
            .unwrap();
    // let block_count = body
    //     .parse::<u64>()
    //     .map_err(|e| Error::FromStr(e.to_string()))?;
    println!("{:?}", body);
}

#[derive(Deserialize, Debug)]
struct BaseResponse {
    motd: Motd,
    success: bool,
    base: String,
    date: String,
    rates: HashMap<String, f32>,
}
#[derive(Deserialize, Debug)]
struct Rate {
    currency: String,
    rate: f64,
}

#[derive(Deserialize, Debug)]
struct Motd {
    msg: String,
    url: String,
}

pub async fn get_rate3() {
    let body = reqwest::get("https://api.exchangerate.host/latest?base=usd")
        .await
        .unwrap()
        .json::<BaseResponse>()
        .await
        .unwrap();

    // let block_count = body
    //     .parse::<u64>()
    //     .map_err(|e| Error::FromStr(e.to_string()))?;
    println!("{:?}", body);
}
