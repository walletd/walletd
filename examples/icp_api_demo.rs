use walletd_icp_api::{IcpWalletApi, WalletDIcpApi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut walletd = WalletDIcpApi::new()?;
    let wallet1 = walletd.create_wallet()?.to_string();
    let wallet2 = walletd.generate_address().await?;
    println!("Wallets: {} and {}", wallet1, wallet2);

    walletd
        .wallets
        .get_mut(&candid::Principal::from_text(&wallet1)?)
        .unwrap()
        .balance = 100_000_000;
    walletd.transfer(&wallet1, &wallet2, 50_000_000).await?;
    println!("Transferred 0.5 ICP");

    let canister_id = candid::Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai")?;
    let balance: u64 = walletd
        .call_canister(canister_id, "account_balance", wallet1.clone())
        .await
        .unwrap_or(0);
    println!("Canister balance query: {}", balance);

    walletd
        .swap_icp_to_btc(
            candid::Principal::from_text(&wallet1)?,
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            25_000_000,
        )
        .await?;
    println!("Swapped 0.25 ICP to BTC");

    println!("Wallet 1 balance: {}", walletd.balance(&wallet1).await?);
    println!("Wallet 2 balance: {}", walletd.balance(&wallet2).await?);

    Ok(())
}
