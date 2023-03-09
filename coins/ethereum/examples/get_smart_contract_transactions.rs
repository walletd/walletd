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

// #[tokio::main]
// async fn main() {

//     let transport = web3::transports::Http::new(INFURA_GOERLI_ENDPOINT)?;
//     let eth_client = EthClient::new(transport,
// &INFURA_GOERLI_ENDPOINT.to_string());     let bn = "8455626";
//     let block_data = EthClient::block_data_from_numeric_string(&eth_client,
// &bn);

//     smart_contract_transactions(&self, block: &web3::types::Block<H256>) {
//         for transaction_hash in &block.transactions {
//             let tx = match self
//                 .web3
//                 .eth()
//                 .transaction(TransactionId::Hash(*transaction_hash))
//                 .await
//             {
//                 Ok(Some(tx)) => Ok(tx),
//                 Err(error) => Err(Error::TxResponseError),
//                 _ => Err(Error::TxResponseError),
//             };
//             // println!("transaction data {:#?}", tx);
//             let smart_contract_addr = match tx.unwrap().to {
//                 Some(addr) => match &self.web3.eth().code(addr, None).await {
//                     Ok(code) => {
//                         if code == &web3::types::Bytes::from([]) {
//                             // "Empty code, skipping
//                             continue;
//                         } else {
//                             // "Non empty code, this address has bytecode we
// have retrieved                             // Attempt to initialise an
// instance of an ERC20 contract at this                             // address
//                             let smart_contract =
// self.initialise_contract(addr).unwrap();                             let
// token_name: String =                                 
// self.get_token_name(&smart_contract).await.unwrap();

//                             // Attempt to get and print the total supply of
// an ERC20-compliant                             // contract
//                             let total_supply: Uint =
//                                 
// self.total_supply(&smart_contract).await.unwrap();

//                             println!("token name {:#?}", token_name);
//                             println!("token supply {:#?}", total_supply);
//                         }
//                     }
//                     _ => {

//                         continue;
//                     }
//                 },
//                 _ => {
//                     // println!("To address is not a valid address,
// skipping.");                     continue;
//                 }
//             };
//         }
//         // println!("{:#?}", smart_contract_addr);
//     }

// }
