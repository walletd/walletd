use candid::Principal;
use walletd_icp::*;

#[test]
fn test_complete_icp_workflow() {
    // 1. Create wallet
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

    // 2. Generate address
    let address = wallet.address();
    assert!(!address.is_empty());

    // 3. Create transaction
    let to = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let tx = wallet.create_transaction(to, 100_000_000, None).unwrap();
    assert_eq!(tx.amount, 100_000_000);

    println!("✅ ICP Integration Test Passed!");
}
