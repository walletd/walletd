#[cfg(test)]
mod tests {
    use walletd_icp::prelude::*;

    #[tokio::test]
    async fn test_basic_wallet_creation() {
        // Test basic wallet creation
        let hd_wallet = HDWallet::new(None).unwrap();
        assert!(!hd_wallet.mnemonic_phrase().is_empty());

        // Test principal creation
        let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);
        assert_eq!(wallet.principal(), principal);
    }
}
