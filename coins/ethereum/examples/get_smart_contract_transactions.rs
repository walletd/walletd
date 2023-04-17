// /// This example shows how to use walletd to filter out all smart contracts
// from a specific block ///
// ///

// use walletd_bip39::{Language, Mnemonic, MnemonicHandler};
// use walletd_coin_model::crypto_wallet::CryptoWallet;
// use walletd_ethereum::*;
// // use hex_literal::hex;
// use walletd_hd_keys::HDKeyPair;
// // use walletd_coin_model::CryptoWallet;
// use walletd_hd_keys::NetworkType;
use core::fmt::Error;

use walletd_coin_model::BlockchainConnector;
use walletd_ethereum::EthClient;
use web3::types::*;

pub const INFURA_GOERLI_ENDPOINT: &str =
    "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
#[tokio::main]
async fn main() {
    let eth_client = EthClient::new(&INFURA_GOERLI_ENDPOINT.to_string()).unwrap();
    let bn = "8455626";
    let block_data = EthClient::block_data_from_numeric_string(&eth_client, &bn)
        .await
        .unwrap();

    // let sct = smart_contract_transactions(block_data, Block) {
    eth_client.smart_contract_transactions(&block_data).await;
    for transaction_hash in &block_data.transactions {
        let tx = match eth_client
            .eth()
            .transaction(TransactionId::Hash(*transaction_hash))
            .await
            .unwrap()
        {
            Some(tx) => Ok(tx),
            None => Err(Error),
        };
        // println!("transaction data {:#?}", tx);
        let _smart_contract_addr = match tx.unwrap().to {
            Some(addr) => match eth_client.eth().code(addr, None).await {
                Ok(code) => {
                    if code == web3::types::Bytes::from([]) {
                        // "Empty code, skipping
                        continue;
                    } else {
                        // Non empty code, this address has bytecode we have retrieved
                        // Attempt to initialise an instance of an ERC20 contract at this
                        // address
                        let smart_contract = eth_client.initialise_contract(addr).unwrap();
                        let token_name: String =
                            eth_client.get_token_name(&smart_contract).await.unwrap();

                        // Attempt to get and print the total supply of an ERC20-compliant contract
                        let total_supply: U256 =
                            eth_client.total_supply(&smart_contract).await.unwrap();

                        println!("token name {:#?}", token_name);
                        println!("token supply {:#?}", total_supply);
                    }
                }
                _ => {
                    continue;
                }
            },
            _ => {
                // println!("To address is not a valid address,
                // skipping.");
                continue;
            }
        };
    }
}
