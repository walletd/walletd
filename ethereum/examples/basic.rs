// //! TODO: This example might have been working at one point but it is very
// out //! of date currently and needs to be updated It's more of a placeholder
// at the //! moment with no actual functionality, we need to update this
// example with a //! working basic example for Ethereum

// // Mainnet Infura: https://celo-mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// // Ropsten Infura: https://ropsten.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// // Localhost Ganache: http://localhost:8545 by default

// extern crate walletd_ethereum;

// /// This example main function is currently mostly commented out
// /// TODO: update this example with an actual working example
// #[tokio::main]
// async fn main() {
//     // Run this example with ganache-cli using the following command:
//     // ganache-cli -b 3 -m "hamster coin cup brief quote trick stove draft
// hobby     // strong caught unable" let transport = web3::transports::Http::new("http://localhost:8545")?;
//     // let transport = web3::transports::Http::new("https://celo-mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073")?; // ganache-cli
//     // let web3 = web3::Web3::new(transport);

//     // println!("Calling accounts.");
//     // let mut accounts = web3.eth().accounts().await?;
//     // println!("Accounts: {:?}", accounts);
//     // accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().
//     // unwrap());

//     // println!("Calling balance.");

//     // let endpoint = "http://localhost:8545".to_string(); // ganache-cli
//     // TODO: need to update this, currently build fails becuase infura_key is
//     // not found let endpoint = "https://celo-mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073".to_string(); // endpoint aimed at infura
//     // let eth_client = EthClient {
//     //     transport: web3::transports::Http::new(&endpoint)?,
//     //     infura_key: infura_key,
//     //     account_addresses: None,
//     //     endpoint: endpoint
//     // };

//     // get_accounts(&eth_client).await;
//     // for account in accounts {
//     //     let balance = web3.eth().balance(account, None).await?;
//     //     println!("Balance of {:?}: {}", account, balance);
//     // }
//     Ok(balance)
// }

// WIP: get result and return value to match
// async fn gas_price(eth_client: &EthClient) -> web3::Result<u256> {
//     println!("start gas_price");

//     let web3 = web3::Web3::new(eth_client.transport.clone());
//     let block_number = web3.eth().block_number().await?;
//     println!("{:#?}", block_number);
//     println!("gas_price: {}", block_number);
//     Ok(block_number.as_u64())
// }

// pub struct EthClient {
//     transport: web3::transports::Http,
//     account_addresses: Option<Vec<web3::types::H160>>,
//     endpoint: String,
//     infura_key: Option<bool>
// }

// impl EthClient {
//     pub fn new(transport: Http, endpoint: &str) -> Self {
//         println!("Infura key value: {:?}", Self.infura_key);
//         Self {
//             transport: transport,
//             account_addresses: None,
//             endpoint: endpoint.to_string(), // web3 uses an &str for endpoint
//             infura_key: None
//         }
//     }
// }

// async fn get_block_number(eth_client: &EthClient) -> web3::Result<u64> {
//     println!("block number ftw");

//     let web3 = web3::Web3::new(eth_client.transport.clone());
//     let block_number = web3.eth().block_number().await?;
//     println!("{:#?}", block_number);
//     println!("Block number: {}", block_number);
//     Ok(block_number.as_u64())
// }

//     // WIP: Address type incorrect
//     // get_balance(&eth_client, web3::types::H160);

//     // Had to comment this out, should uncomment and update once we get this
//     // function to work get_block_number(&eth_client).await;

//     // gas_price(&eth_client).await;

//     // let account_list = get_accounts(&eth_client);

//     // Ok(())
// }

// // WIP
// // async fn get_balance(eth_client: &EthClient, address: web3::types::H160)
// -> // web3::Result<web3::types::U256> {     println!("balance pre-ftw");
// //     let transport = web3::transports::Http::new("http://localhost:8545")?;
// //     let web3 = web3::Web3::new(transport);
// //     let account: H160 = 0x75df5695686338883675bb27bd06fc7578aa01b7;
// //     println!("balance ftw");
// //     println!("{:#?}", account);
// //     println!("Balance for {}", account);
// //     let balance = web3.eth().balance(accounts[0], None).await?;
// //     println!("{:#?}", balance);
// //     // for account in accounts {

// //     // }
// //     Ok(balance)
// // }

// // WIP: get result and return value to match
// // async fn gas_price(eth_client: &EthClient) -> web3::Result<u256> {
// //     println!("start gas_price");

// //     let web3 = web3::Web3::new(eth_client.transport.clone());
// //     let block_number = web3.eth().block_number().await?;
// //     println!("{:#?}", block_number);
// //     println!("gas_price: {}", block_number);
// //     Ok(block_number.as_u64())
// // }

// // pub struct EthClient {
// //     transport: web3::transports::Http,
// //     account_addresses: Option<Vec<web3::types::H160>>,
// //     endpoint: String,
// //     infura_key: Option<bool>
// // }

// // impl EthClient {
// //     pub fn new(transport: Http, endpoint: &str) -> Self {
// //         println!("Infura key value: {:?}", Self.infura_key);
// //         Self {
// //             transport: transport,
// //             account_addresses: None,
// //             endpoint: endpoint.to_string(), // web3 uses an &str for
// endpoint //             infura_key: None
// //         }
// //     }
// // }

// use std::io::Error;

// use web3::transports::Http;
// use web3::types::H160;

// // async fn get_block_number(eth_client: &EthClient) -> web3::Result<u64> {
// //     println!("block number ftw");

// //     let web3 = web3::Web3::new(eth_client.transport.clone());
// //     let block_number = web3.eth().block_number().await?;
// //     println!("{:#?}", block_number);
// //     println!("Block number: {}", block_number);
// //     Ok(block_number.as_u64())
// // }

// // async fn get_accounts(eth_client: &EthClient) -> web3::Result<Vec<H160>> {
// //     println!("accounts pre-ftw");
// //     //let transport = web3::transports::Http::new("http://localhost:8545")?;
// //     let transport = web3::transports::Http::new("https://celo-mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073")?;
// //     let web3 = web3::Web3::new(transport);
// //     let accounts = web3.eth().accounts().await?;
// //     println!("accounts ftw");
// //     println!("{:#?}", accounts);
// //     println!("Account balance for {}", accounts[0]);
// //     let balance = web3.eth().balance(accounts[0], None).await?;
// //     println!("{:#?}", balance);
// //     // for account in accounts {

// //     // }
// //     Ok(accounts)

// // }

// // pub fn get_balances(&EthClient) {

// // }
