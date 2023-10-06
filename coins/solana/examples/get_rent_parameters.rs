use walletd_solana::solana_client::SolanaClient;

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let solana_client = SolanaClient::new(rpc_url).await.unwrap();
    let latest_blockhash = solana_client
        .rpc_client()
        .get_latest_blockhash()
        .await
        .unwrap();
    println!("Latest blockhash: {:?}", latest_blockhash);

    let data_len = 300;
    let balance = solana_client
        .rpc_client()
        .get_minimum_balance_for_rent_exemption(data_len)
        .await
        .unwrap();
    println!(
        "Assuming a 300 byte contract, the minimum fee to be rent-exempt is: {}",
        balance
    );
    
}
