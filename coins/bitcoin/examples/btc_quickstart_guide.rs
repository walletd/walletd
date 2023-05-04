use walletd_bitcoin::prelude::*;
use walletd_hd_key::prelude::*;

#[tokio::main]
async fn main() -> Result<(), walletd_bitcoin::Error> {
    let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
    let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;
    let mut btc_wallet = BitcoinWallet::builder()
        .master_hd_key(master_hd_key)
        .build()?;
    let btc_client = Blockstream::new("https://blockstream.info/testnet/api")?;
    let fee_estimates = btc_client.fee_estimates().await?;
    println!("fee estimates: {:?}", fee_estimates);
    btc_wallet.set_blockchain_client(btc_client);
    btc_wallet.sync().await?;
    for addr in btc_wallet.associated_info() {
        println!(
            "address: {}, derivation path {}",
            addr.address.public_address(),
            addr.hd_key().derivation_path().to_string()
        );
    }
    println!("next receive address: {}", btc_wallet.receive_address()?);
    println!(
        "next change address: {}",
        btc_wallet.next_change_address()?.public_address()
    );

    let balance = btc_wallet.balance().await?;
    println!(
        "bitcoin wallet balance: {} BTC, ({} satoshi",
        balance.btc(),
        balance.satoshi()
    );

    Ok(())
}
