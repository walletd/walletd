//! Integration test suite for Prasaga Avio SDK
use walletd_prasaga_avio::types::PrasagaAvioAddress;

use walletd_prasaga_avio::{
    Operation, PrasagaAvioClient, PrasagaAvioKeypair, TransactionBuilder, TransactionSigner,
};

#[tokio::test]
async fn test_full_transaction_flow() {
    // Setup
    let _client = PrasagaAvioClient::mocknet().await.unwrap();
    let keypair = PrasagaAvioKeypair::from_seed(b"test seed", "m/44'/9000'/0'/0/0").unwrap();
    let address = PrasagaAvioAddress::from_public_key(&keypair.public_key_bytes()).unwrap();

    // Build transaction
    let tx_builder = TransactionBuilder::new()
        .add_operation(Operation::Transfer {
            to: "saga1234567890".to_string(),
            amount: 1000,
        })
        .with_gas_limit(100_000);

    // Sign transaction
    let signed_tx = TransactionSigner::sign_transaction(tx_builder, &keypair, &address, 1).unwrap();

    // Verify signature
    assert_eq!(signed_tx.signature.len(), 64);
    assert!(!signed_tx.hash.is_empty());
}

#[tokio::test]
async fn test_network_switching() {
    // Test all three networks
    let mocknet = PrasagaAvioClient::mocknet().await.unwrap();
    assert_eq!(mocknet.chain_id(), 31337);

    let testnet = PrasagaAvioClient::testnet().await.unwrap();
    assert_eq!(testnet.chain_id(), 9000);

    let mainnet = PrasagaAvioClient::mainnet().await.unwrap();
    assert_eq!(mainnet.chain_id(), 1);
}

#[test]
fn test_address_derivation() {
    let test_cases = vec![
        (b"seed1".to_vec(), "m/44'/9000'/0'/0/0"),
        (b"seed2".to_vec(), "m/44'/9000'/0'/0/1"),
        (b"seed3".to_vec(), "m/44'/9000'/1'/0/0"),
    ];

    for (seed, path) in test_cases {
        let keypair = PrasagaAvioKeypair::from_seed(&seed, path).unwrap();
        let address = PrasagaAvioAddress::from_public_key(&keypair.public_key_bytes()).unwrap();

        // Verify address format
        let address_str = address.to_string();
        assert!(address_str.starts_with("saga"));
        assert!(address_str.len() > 10);
    }
}
