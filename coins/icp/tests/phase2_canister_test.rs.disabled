#[cfg(test)]
mod phase2_tests {

    use candid::Principal;
    use walletd_icp::canister::canisters::Account;

    #[test]
    fn test_account_creation() {
        let owner = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let account = Account {
            owner,
            subaccount: None,
        };

        assert_eq!(account.owner, owner);
        assert!(account.subaccount.is_none());
    }

    #[test]
    fn test_canister_client_types() {
        // Verify types compile and can be instantiated
        let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        // Would need mock agent for full test
    }
}
