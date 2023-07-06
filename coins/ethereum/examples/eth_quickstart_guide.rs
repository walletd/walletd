use ethers::prelude::*;
use walletd_ethereum::prelude::*;
use walletd_hd_key::prelude::*;

const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
#[tokio::main]
async fn main() -> Result<(), walletd_ethereum::Error> {
    let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
    let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;
    let mut ethereum_wallet = EthereumWallet::builder()
        .master_hd_key(master_hd_key)
        .build()?;

    let public_address = ethereum_wallet.public_address();

    println!("ethereum wallet public address: {}", public_address);

    assert!(ethereum_wallet.private_key().is_ok());
    assert!(ethereum_wallet.public_key().is_ok());

    let derived_hd_key = ethereum_wallet.derived_hd_key()?;
    let private_key =
        EthereumPrivateKey::from_slice(&derived_hd_key.extended_private_key()?.to_bytes())?;
    let address_derivation_path = &derived_hd_key.derivation_path.clone();

    // EthereumWallet stores the private key as a 32 byte array
    let secret_bytes = private_key.to_bytes();

    // Instantiate a provider (connecttion) pointing to the endpoint we want to use
    let provider = Provider::try_from(PROVIDER_URL).unwrap();

    // Instantiate a ethers local wallet from the wallet's secret bytes
    let wfbres = Wallet::from_bytes(&secret_bytes);

    let wfb = wfbres.unwrap();
    // 5 = goerli chain id

    // Link our wallet instance to our provider for signing our transactions
    let client = SignerMiddleware::new(provider, wfb.with_chain_id(5u64));

    // Create a transaction request to send 10000 wei to the Goerli address
    let tx = TransactionRequest::new()
        .to("0x681dA56258fF429026449F1435aE87e1B6e9F85b")
        .gas(21000)
        .value(10000)
        .chain_id(5u64);

    println!("tx: {:?}", &tx);

    let pending_tx = client.send_transaction(tx, None).await.unwrap();

    let receipt = pending_tx
        .await
        .unwrap()
        .ok_or_else(|| println!("tx dropped from mempool"))
        .unwrap();
    let tx = client
        .get_transaction(receipt.transaction_hash)
        .await
        .unwrap();

    println!("tx: {:?}", &tx);

    assert_eq!(
        address_derivation_path.to_string(),
        "m/44'/60'/0'/0/0".to_string()
    );

    let ethclient_url = PROVIDER_URL;
    let eth_client = EthClient::new(ethclient_url)?;
    let tx_hash = "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b";
    let tx = eth_client.transaction_data_from_hash(tx_hash).await?;
    let block_number = eth_client.current_block_number().await;
    let gas_price = eth_client.gas_price().await;

    println!("Block number: {:#?}", block_number);
    println!("Gas price: {:#?}", gas_price);
    println!("transaction data: {:?}", tx);
    ethereum_wallet.set_blockchain_client(eth_client);
    let balance = ethereum_wallet.balance().await?;
    println!(
        "ethereum wallet balance: {} ETH, ({} wei)",
        balance.eth(),
        balance.wei()
    );

    Ok(())
}
