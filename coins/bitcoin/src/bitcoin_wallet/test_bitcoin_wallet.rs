use crate::{BitcoinWallet, Error, AddressType};

#[test]
fn test_default() -> Result<(), Error> {
    let expected_default = BitcoinWallet {
        associated: Vec::new(),
        blockchain_client: None,
        address_format: AddressType::P2wpkh,
        master_hd_key: None,
        gap_limit: 20,
        account_discovery: true,
        hd_path_builder: None,
    };
    let wallet = BitcoinWallet::default();
    assert_eq!(wallet.address_format, expected_default.address_format);
    assert_eq!(wallet.associated, expected_default.associated);
    assert_eq!(wallet.blockchain_client.is_none(), expected_default.blockchain_client.is_none());
    assert_eq!(wallet.master_hd_key.is_none(), expected_default.master_hd_key.is_none());
    assert_eq!(wallet.gap_limit, expected_default.gap_limit);
    assert_eq!(wallet.account_discovery, expected_default.account_discovery);
    assert_eq!(wallet.hd_path_builder.is_none(), expected_default.hd_path_builder.is_none());
    Ok(())
}