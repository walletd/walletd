// // Mainnet Infura: https://mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// // Ropsten Infura: https://ropsten.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// // Goerli Infura: https://goerli.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// // Localhost Ganache: http://localhost:8545 by default

// mod ethclient;

// use std::io::Error;

// use walletd_ethereum::INFURA_GOERLI_ENDPOINT;
// use web3::contract::{Contract, Options};
// use web3::helpers as w3h;
// use web3::transports::Http;
// use web3::types::{Block, BlockId, BlockNumber, Transaction, TransactionId,
// H160, H256, U256, U64}; use crate::ethclient::ethclient::EthClient;

// const GOERLI_TEST_ADDRESS: &str =
// "0xFf7FD50BF684eb853787179cc9c784b55Ac68699"; #[tokio::main]
// async fn main() -> web3::Result<()> {
//     //main_wip()?;

//     let transport = web3::transports::Http::new(&INFURA_GOERLI_ENDPOINT)?;
//     let eth_client = EthClient::new(transport,
// &GOERLI_TEST_ADDRESS.to_string());

//     let block_number = eth_client.current_block_number().await;

//     let gas_price = eth_client.gas_price().await;

//     // let block_data =
// eth_client.web3.eth().block(BlockId::Number(BlockNumber::Latest)).await.
// unwrap().unwrap();     // print_type_of(&block_data);
//     // println!("{:#?}", block_data);
//     // let bd = eth_client.get_latest_block().await?;
//     // print_type_of(&bd);
//     // println!("{:#?}", bd);
//     // eth_client.print_txdata_for_block(&bd).await;

//     let bd = eth_client.block_data_from_numeric_string(&"800000").await?;
//     //let bd = eth_client.block_by_numeric_string(&"80000").await?;
//     print_type_of(&bd);

//     eth_client.smart_contract_transactions(&bd).await;

//     // bd = eth_client.print_txdata_for_block(bd);
//     //let block_id = BlockId::Number(BlockNumber::Latest);

//     //let latest_block = eth_client.get_latest_block().await;
//     // let latest_block = eth_client.web3
//     // .eth()
//     // .block(BlockId::Number(BlockNumber::Latest))
//     // .await
//     // .unwrap()
//     // .unwrap();
//     // let latest_block: web3::types::block::Block<primitive_types::H256> =
// eth_client.get_latest_block().await.unwrap();     //eth_client.
// get_latest_block().await;     // println!("latest block {:#?}",
// latest_block);)     // let block =
// eth_client.get_block_by_hash(block.hash.unwrap()).await;

//     // let transaction_count =
// eth_client.get_block_transaction_count_by_number(block_number).await;

//     Ok(())
// }
