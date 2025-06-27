use candid::Principal;
use walletd_icp::*;

#[cfg(test)]
mod canister_tests {
    use super::*;

    #[test]
    fn test_canister_client_creation_mock() {
        // Test canister ID parsing without actual agent
        let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        assert_eq!(canister_id.to_text(), "rrkah-fqaaa-aaaaa-aaaaq-cai");
    }

    #[test]
    fn test_did_operations() {
        let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let did = IcpDID::create(principal).unwrap();
        assert!(did.to_string().starts_with("did:icp:"));
    }
}
