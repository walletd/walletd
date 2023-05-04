use walletd_ethereum::prelude::*;
use walletd_hd_key::prelude::*;

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
    let address_derivation_path = derived_hd_key.derivation_path;
    println!("address derivation path: {}", address_derivation_path);
    assert_eq!(
        address_derivation_path.to_string(),
        "m/44'/60'/0'/0/0".to_string()
    );

    let ethclient_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
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
