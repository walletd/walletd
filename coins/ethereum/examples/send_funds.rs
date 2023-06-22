// EthereumAmount.new_from_eth(u64)
// use std::str::FromStr;

// use walletd_bip39::{Bip39Mnemonic, Mnemonic, MnemonicBuilder as oldMnemonicBuilder};
// // use web3::types::U256;

// use walletd_coin_core::{BlockchainConnector, CryptoWallet, CryptoWalletBuilder};
// use walletd_ethereum::{EthClient, EthereumAmount, EthereumWallet};
// use walletd_hd_key::HDNetworkType;

// use ethers::signers::{LocalWallet, Signer};
// use ethers::middleware::{gas_oracle::GasNow, MiddlewareBuilder};

// use ethers::{
//     core::{types::TransactionRequest, utils::Anvil},
//     providers::{Http, Middleware, Provider},
// };
// //use eyre::Result;
// use std::convert::TryFrom;

// use ethers::{
//     core::rand,
// };

use ethers::{
    core::{types::TransactionRequest},
    middleware::SignerMiddleware,
    providers::{Middleware, Provider},
    signers::{Signer},
    signers::{coins_bip39::English, MnemonicBuilder},
};

const GOERLI_TEST_ADDRESS: &str = "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
#[tokio::main]
async fn main() {
   
    let phrase = "mandate rude write gather vivid inform leg swift usual early bamboo element"; 
    let index = 0u32; // The wallet number's index. Indexes in Ethereum start at 0.

    // Initialise a provider from a URL
    // TODO: this should be handled by our EthClient instead of directly calling ethers
    let provider = Provider::try_from(PROVIDER_URL).unwrap();

    // Initialise a wallet using the Mnemonic Builder (We need to switch this to our facade at some stage)
    let wallet_nonlocal = MnemonicBuilder::<English>::default()
        .phrase(phrase)
        .index(index)
        .unwrap()
        .build()
        .unwrap();

    println!("provider: {:?}", &provider);

    // 5 = goerli chain id 
    let client = SignerMiddleware::new(provider, wallet_nonlocal.with_chain_id(5u64));

    let tx = TransactionRequest::new()
        .to("0x681dA56258fF429026449F1435aE87e1B6e9F85b")
        .gas(21000)
        .value(10000);
     
    let pending_tx = client.send_transaction(tx, None).await.unwrap();

    let receipt = pending_tx.await.unwrap().ok_or_else(|| println!("tx dropped from mempool")).unwrap();
    let tx = client.get_transaction(receipt.transaction_hash).await.unwrap();
    
    println!("tx: {:?}", &tx);
}
