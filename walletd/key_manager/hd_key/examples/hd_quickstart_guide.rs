use slip44::{Coin, Symbol};
use walletd_hd_key::prelude::*;

fn main() -> Result<(), walletd_hd_key::Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let master_seed = Seed::from_str(seed_hex)?;
    // Setting a network type on the HDKey is required, you should select HDNetworkType::TestNet during development and testing purposes and to avoid using real funds and HDNetworkType::MainNet for production level code with caution.
    // Be sure to be consistent with HDNetworkType when connecting to the blockchain, make sure to use a compatible blockchain for the specified network type category

    let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;

    // Wallet Import Format (WIF) is a standard way to encode private keys
    println!("wif of master hd key {}", master_hd_key.to_wif().unwrap());
    // The extended public key and extended private key can be serialized using the serialized string format
    println!(
        "master hd key extended public key: {}",
        master_hd_key.extended_public_key_serialized()?
    );
    println!(
        "master hd key extended private key: {}",
        master_hd_key.extended_private_key_serialized()?
    );
    assert_eq!(master_hd_key.depth(), 0);

    let default_deriv_path = HDPath::builder().build().to_string();
    // without specifying the purpose the default derivation path is "m
    assert_eq!(default_deriv_path, "m");
    println!("default derivation path: {}", default_deriv_path);

    let account_deriv_path = HDPath::builder()
        .purpose_index(HDPurpose::BIP44.to_shortform_num())
        .coin_type_index(Coin::from(Symbol::ETH).id())
        .account_index(0)
        .no_change_index()
        .no_address_index()
        .build()
        .to_string();

    println!("account derivation path: {}", account_deriv_path);

    assert_eq!(&account_deriv_path, "m/44'/60'/0'");
    let eth_first_account_key = master_hd_key.derive(&account_deriv_path)?;
    assert_eq!(
        eth_first_account_key.master_seed(),
        master_hd_key.master_seed()
    );
    println!(
        "eth_first_account_key depth {}",
        eth_first_account_key.depth()
    );
    assert_eq!(eth_first_account_key.depth(), 3);
    println!(
        "wif of eth_first_account_key {}",
        eth_first_account_key.to_wif()?
    );

    // Can derive a child key from a master key or a parent key, the derivation path must be a valid derivation path starting from the master node
    let derive_from_master = master_hd_key.derive("m/44'/60'/0'/0/0")?;
    let derive_from_parent = eth_first_account_key.derive("m/44'/60'/0'/0/0")?;
    assert_eq!(derive_from_master, derive_from_parent);
    println!(
        "derive_from_master == derive_from_parent: {}",
        derive_from_master == derive_from_parent
    );

    // Can flexibly specify the derivation path using the HDPathBuilder, specifying the derivation path using a string is even more flexible
    let custom_key_path = HDPath::builder()
        .purpose_index(HDPurpose::BIP84.to_shortform_num())
        .coin_type_index(Coin::Testnet.id())
        .account_index(0)
        .change_index(1)
        .address_index(0)
        .hardened_address()
        .build()
        .to_string();

    assert_eq!(custom_key_path, "m/84'/1'/0'/1/0'");
    // Can use ' or h to specify hardened derivation
    let custom_key = master_hd_key.derive(&custom_key_path)?;
    let key_compare = master_hd_key.derive("m/84h/1h/0h/1/0h")?;

    assert_eq!(custom_key, key_compare);

    // Shortcut to create a derived key directly from master seed
    let derived_key = HDKey::new(
        Seed::from_str(seed_hex)?,
        HDNetworkType::TestNet,
        &custom_key_path,
    )?;
    assert_eq!(derived_key, custom_key);
    println!("derived_key: {:?}", derived_key);
    println!("derived_key depth: {}", derived_key.depth());
    println!("derived_key wif: {}", derived_key.to_wif()?);
    // Can display the extended public key and extended private key using the serialized string format
    println!(
        "derived_key public key: {}",
        derived_key.extended_public_key_serialized()?
    );
    println!(
        "derived_key private key: {}",
        derived_key.extended_private_key_serialized()?
    );

    Ok(())
}
