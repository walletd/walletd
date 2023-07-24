#[tokio::main]
async fn main() {
    walletd_rates::get_rate2().await;
}
