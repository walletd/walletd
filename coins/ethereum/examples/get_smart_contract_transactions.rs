// /// This example shows how to use walletd to filter out all smart contracts
// from a specific block ///
// ///

// use walletd_bip39::{Language, Mnemonic, MnemonicExt};
// use walletd_coin_core::crypto_wallet::CryptoWallet;
// use walletd_ethereum::*;
// // use hex_literal::hex;
// use walletd_hd_keys::HDKeyPair;
// // use walletd_coin_core::CryptoWallet;
// use walletd_hd_keys::NetworkType;

use std::sync::Arc;

use ethers::prelude::*;
use ethers::{
    abi::Abi,
    types::{Address, H256},
};
use serde;
use walletd_coin_core::BlockchainConnector;
use walletd_ethereum::EthClient;
use serde_json::json;
use serde::{Deserialize, Serialize};

pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
pub const UNISWAP_V2_ROUTER_ADDRESS: &str = h"0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"; //Goerli
pub const UNISWAP_V2_ROUTER_ADDRESS_MAINNET: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"; //Mainnet
pub const UNISWAP_V2_ROUTER_ABI: &str = include_str!("../abi/uniswap_v2_router.json");

abigen!(
    UNISwapV2Router,
    "abi/uniswap_v2_router.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// async fn get_weth_address_from_uniswap(client: &client) -> () {
//     let contract = UNISwapV2Router::new(UNISWAP_V2_ROUTER_ADDRESS.clone(), Arc::new(client.clone()));
//     let value = contract.number().call().await?;
//     Ok()
// }

#[tokio::main]
async fn main() {

    let abigen = UNISWAP_V2_ROUTER_ABI.to_string();

    let eth_client = EthClient::new(PROVIDER_URL).unwrap();
    let provider = eth_client.ethers();
    let abi: Abi = serde_json::from_str(UNISWAP_V2_ROUTER_ABI).unwrap();

    let chain_id_connected = provider.get_chainid().await.unwrap();

    //let arc_client = Arc::new(eth_client);
    let client = Arc::new(provider);

    let address: H160 = WETH_ADDRESS.parse().unwrap();

    println!("client: {:?}", &client);
    println!("address: {:?}", &address);
    //let weth_address = get_weth_address_from_uniswap(&client);

    let uniswap_router: uni_swap_v2_router::UNISwapV2Router<_> = UNISwapV2Router::new(UNISWAP_V2_ROUTER_ADDRESS.clone(), &client.clone());

    // let router02_abi = serde_json::from_str(UNISWAP_V2_ROUTER_ABI).unwrap();

    // let router02_contract = Contract::from_json(
    //     web3s.eth(),
    //     router02_addr,
    //     include_bytes!("router02_abi.json"),
    // )
    // .unwrap()


    // let weth_addr: Address = router02_contract
    //     .query("WETH", (), None, Options::default(), None)
    //     .await
    //     .unwrap();
    // println!("WETH address: {:?}", &weth_addr);


    // let mut tx_raw = TypedTransaction::Legacy(TransactionRequest::new());

    // if chain_id_connected == U256::from(5) {

    //     let contract_address = "0x326C977E6efc84E512bB9C30f76E30c160eD06FB".parse::<Address>()?; //Chainlink Address Goerli

    //     let from_wallet = "0x66C1d8A5ee726b545576A75380391835F8AAA43c";

    //     let hex_string_function = "balanceOf(address)";
    //     let hex_string_function_hashed = ethers::utils::keccak256(hex_string_function);

    //     let function_selector:           String = prefix_hex::encode(&hex_string_function_hashed[0..4]);
    //     let padded_zeroes:               &str = "000000000000000000000000";
    //     let slice_wallet_to_add_to_data: &str = &from_wallet[2..42];
        
    //     let raw_string = function_selector + padded_zeroes + slice_wallet_to_add_to_data;

    //     let raw_call_data = Bytes::from_str(&raw_string).unwrap();

    //     tx_raw.set_to(contract_address);
    //     tx_raw.set_data(raw_call_data);

    // }
    // if chain_id_connected == U256::from(8081) {
    //     let contract_address = "0x8f01876ccd727fd703787ed477b7f71e1e3adbb1".parse::<Address>()?;
   
    //     let raw_call_data = Bytes::from_str("0x8da5cb5b").unwrap(); //ERC-20: balanceOf(0x66C1d8A5ee726b545576A75380391835F8AAA43c)

    //     tx_raw.set_to(contract_address);
    //     tx_raw.set_data(raw_call_data);

    // }
       
    // println!("{:?}", tx_raw);

    // let call_return_data = provider.call(&tx_raw,None).await?;
    
    // println!("{:?}", call_return_data);
    // println!("{:?}", call_return_data.whatisthis); //Shows type as "ethers::types::Bytes" in error message. Credit: https://stackoverflow.com/a/21747400


    //     //let block_data = EthClient::block_data_from_numeric_string(&eth_client, &bn)
    //         .await
    //         .unwrap();

    //     // let sct = smart_contract_transactions(block_data, Block) {
    //    // eth_client.smart_contract_transactions(&block_data).await;
    //     for transaction_hash in &block_data.transactions {
    //         let tx = match eth_client
    //             .eth()
    //             .transaction(TransactionId::Hash(*transaction_hash))
    //             .await
    //             .unwrap()
    //         {
    //             Some(tx) => Ok(tx),
    //             None => Err(Error),
    //         };
    //         // println!("transaction data {:#?}", tx);
    //         let _smart_contract_addr = match tx.unwrap().to {
    //             Some(addr) => match eth_client.eth().code(addr, None).await {
    //                 Ok(code) => {
    //                     if code == web3::types::Bytes::from([]) {
    //                         // "Empty code, skipping
    //                         continue;
    //                     } else {
    //                         // Non empty code, this address has bytecode we have retrieved
    //                         // Attempt to initialise an instance of an ERC20 contract at this
    //                         // address
    //                         let smart_contract = eth_client.initialise_contract(addr).unwrap();
    //                         let token_name: String =
    //                             eth_client.get_token_name(&smart_contract).await.unwrap();

    //                         // Attempt to get and print the total supply of an ERC20-compliant contract
    //                         let total_supply: U256 =
    //                             eth_client.total_supply(&smart_contract).await.unwrap();

    //                         println!("token name {:#?}", token_name);
    //                         println!("token supply {:#?}", total_supply);
    //                     }
    //                 }
    //                 _ => {
    //                     continue;
    //                 }
    //             },
    //             _ => {
    //                 // println!("To address is not a valid address,
    //                 // skipping.");
    //                 continue;
    //             }
    //         };
    //     }
}
