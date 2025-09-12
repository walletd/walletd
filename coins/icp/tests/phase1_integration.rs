use candid::Principal;
use walletd_icp::{HDNetworkType, IcpWallet};

#[tokio::test]
async fn test_full_transaction_flow() {
    // Create wallet
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

    // Create recipient
    let to = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

    // Create transaction
    let tx = wallet
        .create_transaction(
            to,
            100_000_000, // 1 ICP
            Some(12345),
        )
        .unwrap();

    // Verify transaction
    assert_eq!(tx.from, principal);
    assert_eq!(tx.to, to);
    assert_eq!(tx.amount, 100_000_000);
    assert_eq!(tx.memo, Some(12345));
}

#[test]
fn test_wallet_creation_with_known_principal() {
    // Test with a known principal instead of deriving from HD key
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

    // Verify wallet was created successfully
    assert!(!wallet.address().is_empty());
    assert_eq!(wallet.principal(), principal);
}

#[test]
fn test_wallet_persistence() {
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet1 = IcpWallet::from_principal(principal, HDNetworkType::MainNet);
    let wallet2 = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

    // Both wallets should generate same address for same principal
    assert_eq!(wallet1.address(), wallet2.address());
}
