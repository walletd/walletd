use candid::Principal;
use walletd_hd_key::{HDKey, HDNetworkType};
use walletd_icp::IcpWallet;
use walletd_mnemonics_core::Seed;

#[test]
fn test_create_wallet_from_principal() {
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);
    assert_eq!(*wallet.principal(), principal);
    assert_eq!(HDNetworkType::MainNet, HDNetworkType::MainNet);
}

#[test]
fn test_create_wallet_from_hd_key() {
    let seed = Seed::new(vec![0u8; 64]);
    let hd_key = HDKey::new_master(seed, HDNetworkType::TestNet).unwrap();
    let wallet_result = IcpWallet::from_hd_key(&hd_key, 0);
    assert!(wallet_result.is_ok());
}
