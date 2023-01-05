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
// use reqwest::Error;

// fn main() -> Result<(), Error> {
//     // let mut res = reqwest::blocking::get("127.0.0.1:8545")?;
//     // let mut body = String::new();
//     // res.read_to_string(&mut body)?;

//     // println!("Status: {}", res.status());
//     // println!("Headers:\n{:#?}", res.headers());
//     // println!("Body:\n{}", body);
//     println!("Test");
//     Ok(())
// }


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
enum TransportType {
    //HTTP,
    #[default]
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
