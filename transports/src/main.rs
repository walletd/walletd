//!
//! # Generic transports for walletd that allow for the retrieval of blockchain information in a currency-agnostic manner
//! 
//! ## Starting Ganache CLI
//!  Ethereum uses BIP32, BIP39 and BIP44 to generate a master seed and derive additional addresses
//! 
//! ```bash
//! ganache-cli -b 3 -m "hamster coin cup brief quote trick stove draft hobby strong caught unable"
//! ```
//! 
//! 
//! 
//! TODO: Provide hardhat examples?
//! ```rust
//! eth_get_net_version();  
//! 
//! ```
extern crate reqwest; // Library

/**
 * //! use reqwest::header::HeaderMap;
//! use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
//! let bcc = BlockchainClient {
//!     transport_type: Https {
//!        endpoint: "127.0.0.1",
//!        headers: HeaderMap::new(),
//!     }
//!     coin_type: "monero",

 */

// use std::io::Read;
// use std::collections::HashMap;
// use reqwest::Client;
// use reqwest::header::HeaderMap;
// use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};

// use walletd_coins::*;

// trait BlockchainClient {
//     fn get(&self, endpoint: &str) -> Result<reqwest::Response, reqwest::Error>;
//     fn new(coin_type: &str, endpoint: &str) -> Box<Self>;
//     fn set_coin_type(&self) -> Result<Box<Self>, reqwest::Error>;
//     fn default() -> Self;
// }

// use error_chain::error_chain;
// use std::io::Read;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }
use std::io::Read; 
use reqwest::Error;
use reqwest::header::*;
// use reqwest::blocking::*;
// use reqwest::blocking::Response;
use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::*;
use std::collections::HashMap;

// fn main() -> () {
//     let mut res = reqwest::blocking::get("http://127.0.0.1:8545")?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;

//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//     println!("Body:\n{}", body);
//     println!("Test");
//     ()
// }

// what futures look like:

// trait Future {
//     type Output;
//     fn poll_transport(&mut self, wake: fn()) -> Poll<Self::Output>;
// }

// enum Poll<T> {
//     Ready(T),
//     Pending,
// }

// syntactic sugar for implementing futures
// fn eth_get_net_version() -> impl Future<Output = ()> {
pub async fn eth_get_net_version(eth_client: &EthClient) -> Result<(String)> {    
    let mut map = HashMap::new();
    map.insert("jsonrpc", "2.0");
    map.insert("id", "0");
    map.insert("method", &eth_client.method);
    let client = reqwest::Client::new();
    let response = client
        .post(&eth_client.endpoint_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", client);
    println!("{:#?}", response);
    Ok((response))
}

// pub async fn eth_get_remote_data(eth_client: &EthClient) -> Result<String> {
//     let result_string: Result<String> = eth_get_net_version(eth_client: &EthClient);
//     //result_string.unwrap();
//     Ok(result_string)
// }

#[derive(Default, Debug)]
pub struct EthClient {
    endpoint_url: String,
    method: String,
    transport_type: TransportType
}

pub async fn eth_get_gas_price(eth_client: &EthClient) -> Result<String> {
    let mut map = HashMap::new();
    map.insert("jsonrpc", "2.0");
    map.insert("id", "0");
    map.insert("method", &eth_client.method);
    let client = reqwest::Client::new();
    let response = client
        .post(&eth_client.endpoint_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", client);
    println!("{:#?}", response);
    Ok((response))
}

// async fn poll_transport() {
//     println!("return net version here");
//     let response_future = : String = eth_get_net_version().await;

// }
// a runtime that allows async / await without having to implement futures ourselves
#[tokio::main]
async fn main() {
    let eth_client = EthClient { 
        endpoint_url: String::from("http://127.0.0.1:8545"),
        method: String::from("net_listening"),
        transport_type: TransportType::HTTP
    };
    let result: Result<(String)> = eth_get_net_version(&eth_client).await;
    // let result: Result<String> = eth_get_remote_data(&eth_client).await;
    println!("{:#?}", result);
    println!("Test");
    let second_eth_client = EthClient { 
        endpoint_url: String::from("http://127.0.0.1:8545"),
        method: String::from("eth_gasPrice"),
        transport_type: TransportType::HTTP
    };
    let result: Result<(String)> = eth_get_gas_price(&second_eth_client).await;
    println!("{:#?}", result);
    println!("Test");

    let result: Result<String> = eth_get_accounts(&second_eth_client).await;
    println!("{:#?}", result);
    println!("Test");

    let result: Result<String> = eth_get_balances(&second_eth_client).await;
    println!("{:#?}", result);
    println!("Test");

    // let result: Result<String> = eth_get_addresses(&second_eth_client).await;
    // println!("{:#?}", result);
    // println!("Test");
}

pub async fn eth_get_accounts(eth_client: &EthClient) -> Result<String> {
    let mut map = HashMap::new();
    map.insert("jsonrpc", "2.0");
    map.insert("id", "0");
    map.insert("method", "eth_accounts");
    let client = reqwest::Client::new();
    let response = client
        .post(&eth_client.endpoint_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

        Ok(response)
}


pub async fn eth_get_balances(eth_client: &EthClient) -> Result<String> {
    let mut map = HashMap::new();
    map.insert("jsonrpc", "2.0");
    map.insert("id", "0");
    map.insert("method", "eth_getBalance");
    map.insert("params", "[\"0xd028d24f16a8893bd078259d413372ac01580769\", \"latest\"]");
    let client = reqwest::Client::new();
    let response = client
        .post(&eth_client.endpoint_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

        Ok(response)
}

// pub async fn eth_get_net_version() -> Result<reqwest::blocking::Response, std::io::Error> {
//     let client = Client::new();
//     // let data_map = HashMap::new();
//     // data_map.insert("jsonrpc", "2.0");
//     // let mut response = client.post("127.0.0.1:8545")
//     //     .header(CONTENT_TYPE, "application/json")
//     //     .header(ACCEPT, "application/json")
//     //     .json(&data_map) 
//     //     .send()
//     //     .await?;

//     // println!("Response: {:?}", response);
//     // response
//         let mut res = reqwest::blocking::get("http://127.0.0.1:8545/");
//         let mut body = String::new();
//         res.read_to_string(&mut body)?;
    
//         // println!("Status: {}", res.status());
//         // println!("Headers:\n{:#?}", res.headers());
//         // println!("Body:\n{}", body);
    
//         res
// }

#[derive(Default, Debug, PartialEq)]
pub enum TransportType {
    #[default]
    HTTP,
    HTTPS,
    //WS,
    //WSS,
    //IPC,
}

// #[derive(Default, Debug, PartialEq)]
// struct Https {
//     transport_type: TransportType,
//     endpoint: String,
//     headers: HeaderMap,
//     client: reqwest::Client,
// }

// impl BlockchainClient for Https {    
//     fn get(&self) -> Result<reqwest::Response, reqwest::Error> {
//         let client = reqwest::Client::new();
//         let mut headers = self.headers.clone();
//         headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
//         headers.insert(ACCEPT, "application/json".parse().unwrap());
//         let res = client.get(&self.endpoint)
//             .headers(&self.headers)
//             .send();
//         res
//     }
// }

// struct Ws {
//     transport_type: WSS,
//     endpoint: String,
//     headers: HeaderMap,
//     client: reqwest::Client
// }

// impl Transport for Ipc {
//     fn get(&self) -> Result<reqwest::Response, reqwest::Error> {
//         let client = reqwest::Client::new();
//         let mut headers = self.headers.clone();
//         headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
//         headers.insert(ACCEPT, "application/json".parse().unwrap());
//         let res = client.get(&self.endpoint)
//             .headers(&self.headers)
//             .send();
//         res
//     }
// }

// impl TransportFactory {
//     fn new_transport(transport: TransportType) -> Box<dyn Transport> {
//         match transport {
//             TransportType::HTTPS => Box::new(Https {
//                 transport_type: Https,
//                 endpoint: "http://localhost:8545",
//                 headers: HeaderMap::new(),
//             }),
//             TransportType::IPC => Box::new(Ipc {
//                 transport_type: Ipc,
//                 endpoint: "/tmp/geth.ipc",
//                 headers: HeaderMap::new(),
//             }),
//             _ => panic!("Not implemented")
//         }
//     }
// }
