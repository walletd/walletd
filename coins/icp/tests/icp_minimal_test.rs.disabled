use candid::Principal;
use walletd_icp::*;

#[test]
fn test_wallet_basics() {
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

    let address = wallet.address();
    assert!(!address.is_empty());

    // Dereference wallet.principal() for comparison
    assert_eq!(*wallet.principal(), principal);
}

#[test]
fn test_did_creation() {
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();

    // Handle Result type
    match IcpDID::create(principal) {
        Ok(did) => {
            // Now this will work with Display trait
            let did_string = did.to_string();
            assert!(did_string.starts_with("did:icp:"));
            assert!(did_string.contains("rrkah-fqaaa-aaaaa-aaaaq-cai"));
        }
        Err(e) => panic!("DID creation failed: {:?}", e),
    }
}

#[test]
fn test_transaction_creation() {
    let from = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let to = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let wallet = IcpWallet::from_principal(from, HDNetworkType::MainNet);

    match wallet.create_transaction(to, 100_000_000, Some(12345)) {
        Ok(tx) => {
            assert_eq!(tx.amount, 100_000_000);
            assert_eq!(tx.from, from);
            assert_eq!(tx.to, to);
        }
        Err(e) => panic!("Transaction creation failed: {:?}", e),
    }
}
