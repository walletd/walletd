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

pub trait FiatRateProvider {
    fn get_rate(&self) -> Result<FiatRates, reqwest::Error>;
}

#[derive(Debug)]
pub enum Providers {
    ExchangeRateApi,
    ExchangeRateHost,
}

#[derive(Debug)]
pub struct FiatRates {
    provider: Providers,
    base: String,
    date: String,
    rates: HashMap<String, f32>,
}

#[derive(Deserialize, Debug)]
pub struct BaseResponse2 {
    result: String,
    // documentation: String
    // terms_of_use: String,
    time_last_update_unix: u64,
    #[serde(rename = "base_code")]
    base: String,
    #[serde(rename = "conversion_rates")]
    rates: HashMap<String, f32>,
}

pub struct ExchangeRateApi {
    api_key: String,
}

impl ExchangeRateApi {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    pub async fn get_rate(&self) -> Result<FiatRates, reqwest::Error> {
        let body = reqwest::get(format!(
            "https://v6.exchangerate-api.com/v6/{}/latest/USD",
            self.api_key,
        ))
        .await
        .unwrap()
        .json::<BaseResponse2>()
        .await
        .unwrap();

        let fiat_rates = FiatRates {
            provider: Providers::ExchangeRateApi,
            base: body.base,
            date: body.time_last_update_unix.to_string(),
            rates: body.rates,
        };
        Ok(fiat_rates)
    }
}

#[derive(Deserialize, Debug)]
pub struct BaseResponse {
    //motd: Motd,
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

pub struct ExchangeRateHost {}

impl ExchangeRateHost {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_rate(&self) -> Result<FiatRates, reqwest::Error> {
        let body = reqwest::get("https://api.exchangerate.host/latest?base=usd")
            .await
            .unwrap()
            .json::<BaseResponse>()
            .await
            .unwrap();

        let fiat_rates = FiatRates {
            provider: Providers::ExchangeRateHost,
            base: body.base,
            date: body.date,
            rates: body.rates,
        };
        Ok(fiat_rates)
    }
}
