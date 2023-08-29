use walletd_solana::solanaclient::SolanaClient;

const URL: &str = "http://127.0.0.1:8899";
#[tokio::main]
async fn main() {
    let solana_client = SolanaClient::new(URL).unwrap();
    println!("Ok, invoked");
    let latest_blockhash = solana_client.rpc_client().get_latest_blockhash().await.unwrap();
    println!("{:?}", latest_blockhash);

    let data_len = 300;
    let balance = solana_client.rpc_client().get_minimum_balance_for_rent_exemption(data_len).await.unwrap();
    println!("Minimum fee to be rent-exempt: {}", balance);
}