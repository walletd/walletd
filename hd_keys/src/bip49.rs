//! Implements BIP49 as described in https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki
//! This BIP defines the derivation scheme for HD wallets using the P2WPKH-nested-in-P2SH (BIP 141) serialization format for segregated witness transactions.
//! When serializing extended keys, this scheme uses alternate version bytes. Extended public keys use 0x049d7cb2 to produce a "ypub" prefix, and private keys use 0x049d7878 to produce a "yprv" prefix. Testnet uses 0x044a5262 "upub" and 0x044a4e28 "uprv."
//! To derive a public key from the root account, this BIP uses the same account-structure as defined in BIP 44, but uses a purpose value of 49'
//! m / purpose' / coin_type' / account' / change / address_index


use crate::bip32::BIP32;
use walletd_coins::CryptoCoin;

pub fn derive_first_account(master_node: &BIP32, coin: &CryptoCoin) -> Result<BIP32, String> {
    let derived_account_path = format!("{}{}{}", "m/49'/", coin.coin_type(), "'/0'");
    BIP32::derived_from_master_with_specified_path(&master_node, derived_account_path)
}
pub fn derive_first_address(master_node: &BIP32, coin: &CryptoCoin) -> Result<BIP32, String> {
    let derived_first_account = derive_first_account(master_node, coin)?;
    println!("First Derived Account HD Key Info: \n{}", derived_first_account);
    let bip84_deriv_path = format!("{}{}{}", "m/49'/", coin.coin_type(), "'/0'/0/0");
    BIP32::derived_from_master_with_specified_path(
        &master_node,
        bip84_deriv_path)
}

// Later will need to add more functionality here to discover accounts and addresses that are being used and those that are not
