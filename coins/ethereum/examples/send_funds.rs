// EthereumAmount.new_from_eth(u64)
// use std::str::FromStr;

use walletd_bip39::{Bip39Mnemonic, Mnemonic, MnemonicBuilder as oldMnemonicBuilder};
// use web3::types::U256;

use walletd_coin_core::{BlockchainConnector, CryptoWallet, CryptoWalletBuilder};
use walletd_ethereum::{EthClient, EthereumAmount, EthereumWallet};
use walletd_hd_key::HDNetworkType;

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
    core::{types::TransactionRequest, utils::Anvil},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    signers::{coins_bip39::English, MnemonicBuilder},
};

const GOERLI_TEST_ADDRESS: &str = "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
const PROV_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
#[tokio::main]
async fn main() {
    // main_wip()?;

    // let provider = Provider::<Http>::connect(PROV_URL).await.unwrap();
    
    let phrase = "mandate rude write gather vivid inform leg swift usual early bamboo element";
    let index = 1u32; // The wallet number's index

    let provider = Provider::try_from(PROV_URL).unwrap();

    let wallet_nonlocal = MnemonicBuilder::<English>::default()
        .phrase(phrase)
        .index(index)
        .unwrap()
        .build()
        .unwrap();

    println!("provider: {:?}", &provider);

    // 5 = goerli chain id
    let client = SignerMiddleware::new(provider, wallet_nonlocal);

    let tx = TransactionRequest::new()
        .to("vitalik.eth")
        .chain_id(5u64)
        .value(10000);
    
    

    let pending_tx = client.send_transaction(tx, None).await.unwrap();

    let receipt = pending_tx.await.unwrap().ok_or_else(|| println!("tx dropped from mempool")).unwrap();
    let tx = client.get_transaction(receipt.transaction_hash).await.unwrap();
    
    println!("tx: {:?}", &tx);

    //let wallet: LocalWallet = "380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc".parse();

    //let mut client = SignerMiddleware::new(provider, wallet);


    // Access mnemonic phrase with password
    // Child key at derivation path: m/44'/60'/0'/0/{index}
    // let wallet: LocalWallet = MnemonicBuilder::<English>::default()
    //     .phrase(phrase)
    //     .index(index)?
    //     // Use this if your mnemonic is encrypted
    //     .build()?;

    // let mnemonic_phrase: &str =
    //     "mandate rude write gather vivid inform leg swift usual early bamboo element";
    // let restored_mnemonic = Bip39Mnemonic::builder()
    //     .mnemonic_phrase(mnemonic_phrase)
    //     .detect_language()
    //     .build()
    //     .unwrap();

    // let seed = restored_mnemonic.to_seed();

    // println!("seed as bytes: {:?}", seed.as_bytes());

    // let blockchain_client =
    //     EthClient::new("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161").unwrap();

    // println!("blockchain_client: {:?}", &blockchain_client);

    // let mut wallet = EthereumWallet::builder()
    //     .mnemonic_seed(seed)
    //     .network_type(HDNetworkType::TestNet)
    //     .build()
    //     .unwrap();
    // wallet.set_blockchain_client(blockchain_client);
    // // This example now assumes that the wallet has been funded with some testnet ETH
    // println!("wallet: {:#?}", &wallet);

    // println!("balance: {:?}", &wallet.balance().await.unwrap());

    // let sa = ethers::types::U256::from(10000);
    // let send_amount = EthereumAmount::from_wei(sa);

    // println!("send_amount: {:?}", &send_amount);


    // let tx_hash = wallet
    //     .transfer(&send_amount, GOERLI_TEST_ADDRESS)
    //     .await
    //     .unwrap();

    // println!("tx_hash: 0x{}", &tx_hash);
}
