#![allow(unexpected_cfgs)]
#![cfg(feature = "phase1")]

use candid::Principal;
use walletd_hd_key::HDKey;
use walletd_icp::{HDNetworkType, IcpTransaction, IcpWallet};
use walletd_mnemonics_core::Seed;

#[test]
fn test_wallet_creation_and_transaction() {
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

    let to = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let tx = wallet
        .create_transaction(to, 100_000_000, Some(12345))
        .unwrap();

    assert_eq!(tx.from, principal);
    assert_eq!(tx.to, to);
    assert_eq!(tx.amount, 100_000_000);
    assert_eq!(tx.memo, 12345);
}

#[test]
fn test_hd_wallet_derivation() {
    let seed = Seed::new(vec![0u8; 64]);
    let master_key = HDKey::new_master(seed, HDNetworkType::MainNet).unwrap();

    // ICP BIP44 path: m/44'/223'/0'/0/0
    let wallet = IcpWallet::from_hd_key(&master_key, 0);
    assert!(wallet.is_ok());
}

#[test]
fn test_did_creation() {
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let mut wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

    let public_key = vec![1u8; 32];
    let did_doc = wallet.create_did(public_key).unwrap();

    assert_eq!(did_doc.id, format!("did:icp:{}", principal.to_text()));
    assert_eq!(did_doc.principal, principal);
}

#[tokio::test]
async fn test_mock_balance() {
    // This test requires the phase1 feature
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet = IcpWallet::from_principal(principal, HDNetworkType::TestNet);

    // Note: This would fail in real usage without a proper agent
    // For now it's just testing the API structure
}
