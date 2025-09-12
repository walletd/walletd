#[cfg(test)]
mod tests {
    use candid::Principal;
    use walletd_hd_key::HDNetworkType;
    use walletd_icp::{IcpTransaction, IcpWallet};

    #[test]
    fn test_create_wallet() {
        let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);
        assert_eq!(HDNetworkType::MainNet, HDNetworkType::MainNet);
    }

    #[tokio::test]
    async fn test_create_transaction() {
        let from = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let to = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let amount = 1000000; // 0.01 ICP

        let tx = IcpTransaction::new(from, to, amount, None, None).unwrap();
        assert_eq!(tx.amount, amount);
        assert_eq!(tx.from, from);
        assert_eq!(tx.to, to);
    }
}
