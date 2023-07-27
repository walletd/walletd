use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct CryptoRate {
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct BitstampResponse {
    timestamp: String,
    ask: String,
}

/// Bitstamp provider
#[derive(Default)]
pub struct Bitstamp {}

impl Bitstamp {
    /// Creates a new Bitstamp provider
    pub fn new() -> Self {
        Self {}
    }

    /// Returns the current BTC/USD rate from Bitstamp
    pub async fn get_rate(base: &str, quote: &str) -> Result<CryptoRate, Error> {
        let pair = format!("{}{}", base, quote).to_lowercase();
        let response = reqwest::get(format!("https://www.bitstamp.net/api/v2/ticker/{}/", pair))
            .await?
            .json::<BitstampResponse>()
            .await?;

        let mut map = HashMap::new();
        // (1.0 / response.ask.parse::<f32>()?).to_string() // for inverting rates
        map.insert(quote.to_string(), response.ask.to_string());
        let rate = CryptoRate {
            base: base.to_string().to_uppercase(),
            date: response.timestamp,
            rates: map,
        };
        Ok(rate)
    }
}

/// ExchangeRateHost for crypto provider
#[derive(Default)]
pub struct ExchangeRateHostCrypto {}

impl ExchangeRateHostCrypto {
    /// Creates a new ExchangeRateHostCrypto provider
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_rate(base: &str, quote: &str) -> Result<CryptoRate, Error> {
        let response = reqwest::get(format!(
            "https://api.exchangerate.host/latest?base={}&symbols={}&source=crypto",
            base, quote
        ))
        .await?
        .json::<ExchangeRateHostResponse>()
        .await?;
        let mut map = HashMap::new();
        for (key, value) in response.rates.into_iter() {
            map.insert(key, value.to_string());
        }
        let rate = CryptoRate {
            base: response.base,
            date: response.date,
            rates: map,
        };
        Ok(rate)
    }
}

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
#[derive(Default)]
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

/// Custom error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error("request error: {0}")]
    Connection(#[from] reqwest::Error),
    #[error("invalid response: {0}")]
    InvalidResult(#[from] std::num::ParseFloatError),
}
