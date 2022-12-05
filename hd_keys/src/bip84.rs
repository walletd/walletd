//! Implements BIP84 as described in https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki
//! This BIP defines the derivation scheme for HD wallets using the P2WPKH (BIP 173) serialization format for segregated witness transactions.
//! When serializing extended keys, this scheme uses alternate version bytes. 
//! Extended public keys use 0x04b24746 to produce a "zpub" prefix, and private keys use 0x04b2430c to produce a "zprv" prefix. 
//! Testnet uses 0x045f1cf6 "vpub" and 0x045f18bc "vprv."
//! To derive a public key from the root account, this BIP uses the same account-structure as defined in BIP 44, but uses a purpose value of 84'
//! m / purpose' / coin_type' / account' / change / address_index

use crate::bip32::BIP32;
use walletd_coins::CryptoCoin;

pub fn derive_first_account(master_node: &BIP32, coin: &CryptoCoin) -> Result<BIP32, String> {
    let derived_account_path = format!("{}{}{}", "m/84'/", coin.coin_type(), "'/0'");
    BIP32::derived_from_master_with_specified_path(&master_node, derived_account_path)
}
pub fn derive_first_address(master_node: &BIP32, coin: &CryptoCoin) -> Result<BIP32, String> {
    let derived_first_account = derive_first_account(master_node, coin)?;
    println!("First Derived Account HD Key Info: \n{}", derived_first_account);
    let bip84_deriv_path = format!("{}{}{}", "m/84'/", coin.coin_type(), "'/0'/0/0");
    BIP32::derived_from_master_with_specified_path(
        &master_node,
        bip84_deriv_path)
}