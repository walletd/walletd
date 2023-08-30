use walletd_solana::solanaclient::SolanaClient;

const URL: &str = "http://127.0.0.1:8899";
#[tokio::main]
async fn main() {
    let solana_client = SolanaClient::new(URL).unwrap();
    let latest_blockhash = solana_client.rpc_client().get_latest_blockhash().await.unwrap();
    println!("Latest blockhaish: {:?}", latest_blockhash);

    let data_len = 300;
    let balance = solana_client.rpc_client().get_minimum_balance_for_rent_exemption(data_len).await.unwrap();
    println!("Assuming a 300 byte contract, the minimum fee to be rent-exempt is: {}", balance);
    
}