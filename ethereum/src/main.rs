// Mainnet Infura: https://mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// Ropsten Infura: https://ropsten.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// Goerli Infura: https://goerli.infura.io/v3/933b67502c4340a7bf3e873f0de62073
// Localhost Ganache: http://localhost:8545 by default

mod ethclient;

// use std::io::Error;
use std::str::FromStr;

use hex_literal::hex;
use thiserror::Error;
use walletd_bip39::{Language, Mnemonic, MnemonicHandler};
use walletd_coin_model::crypto_wallet::CryptoWallet;
use walletd_ethereum::*;
use walletd_hd_key::HDKey;
// use walletd_coin_model::CryptoWallet;
use walletd_hd_key::NetworkType;
use web3::contract::{Contract, Options};
use web3::helpers as w3h;
use web3::transports::Http;
use web3::types::{
    Address, Block, BlockId, BlockNumber, Transaction, TransactionId, H160, H256, U256, U64,
};

use crate::ethclient::ethclient::EthClient;
// use walletd_coin_model::walletd_hd_keys::{MnemonicKeyPairType, NetworkType};
// use walletd_cli::{onboard, transact};

// pub fn recover_existing_keypair(
//     mnemonic_keypair_type: MnemonicKeyPairType,
//     mnemonic_phrase: &String,
//     passphrase: Option<&str>,
//     network_type: NetworkType,
// ) -> Result<KeyPair, anyhow::Error> {
//     match mnemonic_keypair_type {
//         MnemonicKeyPairType::HdBip39 => {
//             let mnemonic = Bip39Mnemonic::detect_language(mnemonic_phrase,
// passphrase)?;             println!("Recovered BIP39 Mnemonic: \n{}",
// mnemonic);             Ok(KeyPair::new(
//                 mnemonic.to_seed(),
//                 mnemonic.phrase(),
//                 mnemonic_keypair_type,
//                 passphrase,
//                 network_type,
//             ))
//         }
//     }
// }

#[derive(Error, Debug)]
pub enum Error {
    #[error("An error was encountered when attempting to initialise from a mnemonic phrase")]
    FromMnemonicError,
    #[error("An error was encountered while trying to retrieve a tx from a tx hash")]
    GetTxError,
}

const GOERLI_TEST_ADDRESS: &str = "0x681dA56258fF429026449F1435aE87e1B6e9F85b";
#[tokio::main]
async fn main() -> web3::Result<()> {
    // main_wip()?;

    // initialise wallet from mnemonic
    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let passphrase: Option<&str> = Some("mypassphrase");
    let restored_mnemonic =
        Mnemonic::from_phrase(Language::English, mnemonic_phrase, passphrase).unwrap();
    let seed = restored_mnemonic.to_seed();

    let wallet = match EthereumWallet::from_mnemonic(
        &seed,
        NetworkType::TestNet,
        EthereumFormat::Checksummed,
    ) {
        Ok(wallet) => Ok(wallet),
        Err(e) => Err(e),
    };

    println!("seed as bytes: {:?}", seed.as_bytes());
    // EthClient is a wallet-agnostic interface for retrieving network data

    let transport = web3::transports::Http::new(&INFURA_GOERLI_ENDPOINT)?;

    // works
    let eth_client = EthClient::new(transport, &GOERLI_TEST_ADDRESS.to_string());
    let block_number = eth_client.current_block_number().await;
    println!("block_number: {:?}", block_number);
    // endworks

    let gas_price = eth_client.gas_price().await;
    println!("gas_price: {:?}", gas_price);

    let address: H160 = GOERLI_TEST_ADDRESS.parse().unwrap();
    let balance = eth_client.balance_of_account(address).await;
    println!("BoA: {:?}", balance);
    // let language: &str = "english";
    // let mnemonic_phrase: &String = &"candy maple cake sugar pudding cream honey
    // rich smooth crumble sweet treat".to_string(); let passphrase: &String =
    // &"".to_string(); let mnemonic = Mnemonic::from_phrase(language,
    // mnemonic_phrase, passphrase)?; println!("Imported Mnemonic Seed Info:
    // \n{}", mnemonic); let mut keypair = HDKeyPair::new(
    //     restored_mnemonic.to_seed(),
    //     restored_mnemonic.phrase(),
    //     MnemonicKeyPairType::HdBip39,
    //     passphrase,
    //     NetworkType::TestNet,
    // );
    // let wallet = EthereumWallet::from_mnemonic(&seed,
    //     NetworkType::TestNet,
    //     EthereumFormat::Checksummed);
    println!("wallet: {:?}", &wallet);

    let tx_hash: H256 =
         hex!("55e88ab80bc5d8842a3f5159cae2d74dd471b07ae922140be3e28e130e9e3162").into();
    let tx_data = eth_client.transaction_data_from_hash(tx_hash).await;

    // println!("tx_data: {:#?}", tx_data);

    // This example now assumes that the wallet has been funded with some testnet
    // ETH let sa = 10000.0;
    // let send_amount: <walletd_ethereum::EthereumAmount as
    // Trait>::new_from_eth::new_from_eth<sa>; println!("send_amount: {:?}",
    // &send_amount);

    // println!("wallet: {:?}", &wallet);

    // let balance = &wallet.unwrap().balance(&GOERLI_TEST_ADDRESS.to_string());

    // if let Err(e) = &wallet {
    //     println!("Error: {:?}", e);
    // }
    // println!("wallet: {:?}", &wallet);
    // keypair::new
    // let mnemonic_keypair_type = MnemonicKeyPairType::HdBip39;
    // let eth_wallet = EthereumWallet::from_hd_key()

    // let mnemonic_phrase: &String = &"candy maple cake sugar pudding cream honey
    // rich smooth crumble sweet treat".to_string(); /// To get the raw byte
    // value use [`Seed::as_bytes()`][Seed::as_bytes()]. These can be used to derive
    /// HD wallet addresses using another crate (deriving HD wallet addresses is
    /// outside the scope of this crate and the BIP39 standard).
    ///
    /// [Mnemonic]: ./mnemonic/struct.Mnemonic.html    
    // let passphrase: &String = &"".to_string();
    // let networktype = NetworkType::TestNet;
    // //let transport = web3::transports::Http::new("http://localhost:8545")?;
    // let keypair = KeyPair::new(
    //     mnemonic.to_seed(),
    //     mnemonic.phrase(),
    //     mnemonic_keypair_type,
    //     passphrase,
    //     network_type,
    // );

    // let keypair = EthereumWallet::from_mnemonic("candy maple cake sugar pudding cream honey rich
    // smooth crumble sweet treat", NetworkType::Goerli, EthereumFormat::Checksummed);

    // let transport = web3::transports::Http::new(&INFURA_GOERLI_ENDPOINT)?;
    // let web3 = web3::Web3::new(&transport);

    // let transport = web3::transports::Http::new(&INFURA_GOERLI_ENDPOINT)?;
    // let eth_client = EthClient::new(transport, &GOERLI_TEST_ADDRESS.to_string());

    // let block_number = eth_client.current_block_number().await;

    // let gas_price = eth_client.gas_price().await;

    // let block_data =
    // eth_client.web3.eth().block(BlockId::Number(BlockNumber::Latest)).await.unwrap().unwrap();
    // print_type_of(&block_data);
    // println!("{:#?}", block_data);
    let bd = eth_client.latest_block().await?;
    //print_type_of(&bd);
    println!("{:#?}", bd);
    // eth_client.print_txdata_for_block(&bd).await;

    // let bd = eth_client.block_data_from_numeric_string(&"800000").await?;
    // let bd = eth_client.block_by_numeric_string(&"80000").await?;
    // let str_slice: String =
    // "0x55e88ab80bc5d8842a3f5159cae2d74dd471b07ae922140be3e28e130e9e3162".to_string();
    // let bs = str_slice.as_bytes(&str_slice[0..32]);
    // let tx_hash: H256 =
    // hex!("55e88ab80bc5d8842a3f5159cae2d74dd471b07ae922140be3e28e130e9e3162").into();
    // let tx_data = eth_client.transaction_data_from_hash(tx_hash).await?;

    // println!("hash: {}", tx_data.hash);
    // println!("nonce: {}", tx_data.nonce);

    // let transport = web3::transports::Http::new(&INFURA_GOERLI_ENDPOINT)?;
    // let web3 = web3::Web3::new(transport);

    // eth_client.smart_contract_transactions(&bd).await;
    // let mut accounts = web3.eth().accounts().await?;
    // secret key cdc07acf62ff0ed32939935c836a0a46e81365e0d17559d9b5d6f0352c297ffc
    // public key 0x681dA56258fF429026449F1435aE87e1B6e9F85b
    // accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());
    // println!("Accounts: {:?}", accounts);
    // let wei_conv: U256 = U256::exp10(18);
    // for account in accounts {
    //     let balance = web3.eth().balance(account, None).await?;
    //     println!(
    //         "Eth balance of {:?}: {}",
    //         account,
    //         balance.checked_div(wei_conv).unwrap()
    //     );
    // }

    // uniswap v2 address 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
    // uniswap v2 requires WETH

    // let router02_addr = Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap();
    // let router02_contract = Contract::from_json(
    //     web3.eth(),
    //     router02_addr,
    //     include_bytes!("./uniswap_v2_router02_abi.json"),
    // )
    // .unwrap();

    // let weth_addr: Address = router02_contract
    //     .query("WETH", (), None, Options::default(), None)
    //     .await
    //     .unwrap();
    // println!("WETH address: {:?}", &weth_addr);

    // Goerli DAI address 0xdc31ee1784292379fbb2964b3b9c4124d8f89c60
    // let block = eth_client.get_block_by_hash(block.hash.unwrap()).await;

    // let transaction_count = eth_client.get_block_transaction_count_by_number(block_number).await;
    Ok(())
}
