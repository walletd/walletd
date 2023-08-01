use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;
// pub async fn get_rate() {
//     let body = reqwest::get("https://www.bitstamp.net/api/v2/ticker/btcusd/")
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();
//     // let block_count = body
//     //     .parse::<u64>()
//     //     .map_err(|e| Error::FromStr(e.to_string()))?;
//     println!("{:?}", body);
// }

/// List of available Fiat providers
#[derive(Debug, PartialEq, Eq)]
pub enum Providers {
    ExchangeRateApi,
    ExchangeRateHost,
}

/// The common format fiat rates are returned in
#[derive(Debug, PartialEq)]
pub struct FiatRates {
    provider: Providers,
    base: String,
    date: String,
    rates: HashMap<String, f32>,
}

#[derive(Deserialize, Debug)]
struct ExchangeRateApiResponse {
    //result: String,
    // documentation: String
    // terms_of_use: String,
    time_last_update_unix: u64,
    #[serde(rename = "base_code")]
    base: String,
    #[serde(rename = "conversion_rates")]
    rates: HashMap<String, f32>,
}

/// The ExchangeRateApi provider
pub struct ExchangeRateApi {
    api_key: String,
}

impl ExchangeRateApi {
    /// Creates a new ExchangeRateApi provider - requires an API key
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
    /// Returns the current fiat rates for USD base currency
    pub async fn get_rate(&self) -> Result<FiatRates, Error> {
        let body = reqwest::get(format!(
            "https://v6.exchangerate-api.com/v6/{}/latest/USD",
            self.api_key,
        ))
        .await?
        .json::<ExchangeRateApiResponse>()
        .await?;

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
struct ExchangeRateHostResponse {
    //motd: Motd,
    //success: bool,
    base: String,
    date: String,
    rates: HashMap<String, f32>,
}

/// ExchangeRateHost provider
pub struct ExchangeRateHost {}

impl ExchangeRateHost {
    /// Creates a new ExchangeRateHost provider
    pub fn new() -> Self {
        Self {}
    }
    /// Returns the current fiat rates for USD base currency
    pub async fn get_rate(&self) -> Result<FiatRates, Error> {
        let body = reqwest::get("https://api.exchangerate.host/latest?base=usd")
            .await?
            .json::<ExchangeRateHostResponse>()
            .await?;

        let fiat_rates = FiatRates {
            provider: Providers::ExchangeRateHost,
            base: body.base,
            date: body.date,
            rates: body.rates,
        };
        Ok(fiat_rates)
    }
}

impl Default for ExchangeRateHost {
    fn default() -> Self {
        Self::new()
    }
}

/// Custom error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error("request error: {0}")]
    Connection(#[from] reqwest::Error),
}
