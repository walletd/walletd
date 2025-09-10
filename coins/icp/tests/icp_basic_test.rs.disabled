use candid::Principal;
use walletd_icp::{HDNetworkType, IcpLedger, IcpWallet};

#[test]
fn test_icp_wallet_creation() {
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);
    assert_eq!(wallet.principal(), principal);
}

#[test]
fn test_icp_address_generation() {
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let account = IcpLedger::principal_to_account(principal);
    assert_eq!(account.0.len(), 32);
}

#[test]
fn test_transaction_creation() {
    let from = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let to = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let wallet = IcpWallet::from_principal(from, HDNetworkType::MainNet);

    let tx = wallet
        .create_transaction(to, 100_000_000, Some(12345))
        .unwrap();
    assert_eq!(tx.amount, 100_000_000);
}
