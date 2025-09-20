use walletd_prasaga_avio::*;

#[tokio::test]
async fn test_client_creation() {
    let endpoints = vec!["https://testnet.prasaga.com".to_string()];
    let client = PrasagaAvioClient::new(endpoints).await;
    assert!(client.is_ok());
}
