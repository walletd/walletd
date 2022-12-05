//! Specifies a derivation path based on the bip44 standard: https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
//!
//! Five Levels in BIP32 path m / purpose' / coin_type' / account' / change / address_index
//! purpose for BIP44 is 44', so all derivation paths will start with m/44'
//! Coin type value is specified based on the SLIP-0044 standard: https://github.com/satoshilabs/slips/blob/master/slip-0044.md (hardned index is used)
//! Account level splits key space into individual user identities
//! Change level uses a constant of 0 for external chain and 1 for internal chain (for change addresses)
//! Index level specifies an address and are numbered and created from index 0 in a sequentially increasing manner

use crate::bip32::BIP32;
use walletd_coins::CryptoCoin;

pub fn derive_first_account(master_node: &BIP32, coin: &CryptoCoin) -> Result<BIP32, String> {
    let derived_account_path = format!("{}{}{}", "m/44'/", coin.coin_type(), "'/0'");
    BIP32::derived_from_master_with_specified_path(&master_node, derived_account_path)
}
pub fn derive_first_address(master_node: &BIP32, coin: &CryptoCoin) -> Result<BIP32, String> {
    let derived_first_account = derive_first_account(master_node, coin)?;
    println!("First Derived Account HD Key Info: \n{}", derived_first_account);
    let bip84_deriv_path = format!("{}{}{}", "m/44'/", coin.coin_type(), "'/0'/0/0");
    BIP32::derived_from_master_with_specified_path(
        &master_node,
        bip84_deriv_path)
}

// Later will need to add more functionality here to discover accounts and addresses that are being used and those that are not