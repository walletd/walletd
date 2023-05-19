use walletd_bitcoin::mempool_space::MempoolSpace;
use walletd_bitcoin::prelude::*;
use walletd_hd_key::prelude::*;

#[tokio::main]
async fn main() -> Result<(), walletd_bitcoin::Error> {
    let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
    let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;
    let btc_wallet = BitcoinWallet::builder()
        .master_hd_key(master_hd_key)
        .build()?;
    // first address = tb1q344p7ttvrpyhxj9xglkwxx6dxytxxjzexth0du
    let address = btc_wallet.next_address()?;
    println!("address: {:?}", address);
    let btc_client = MempoolSpace::new("https://mempool.space/signet/api")?;
    // let height = btc_client.block_height().await?;
    // println!("block height: {:?}", height);
    let fee_estimates = btc_client.fee_estimates().await?;
    println!("fee estimates: {:?}", fee_estimates);
    // let transactions = btc_client.transactions("tb1q344p7ttvrpyhxj9xglkwxx6dxytxxjzexth0du").await?;
    // println!("transactions: {:?}", transactions);
    // let utxos = btc_client.utxo("tb1q344p7ttvrpyhxj9xglkwxx6dxytxxjzexth0du").await?;
    // println!("utxos: {:?}", utxos);
    // let raw = btc_client.raw_transaction_hex("636fa7bb3fc6dc105fb570e4b4c07e2c9b6fde54f1c1a99b2568969a1921cf0d").await?;
    // println!("raw: {:?}", raw);
    // let transaction = btc_client.transaction("636fa7bb3fc6dc105fb570e4b4c07e2c9b6fde54f1c1a99b2568969a1921cf0d").await?;
    // println!("transaction: {:?}", transaction);
    Ok(())
}
