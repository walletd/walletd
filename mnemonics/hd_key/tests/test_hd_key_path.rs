use slip44::{Coin, Symbol};
use std::str::FromStr;
use walletd_hd_key::{
    Error, ExtendedPrivateKey, ExtendedPublicKey, HDKey, HDNetworkType, HDPath, HDPurpose, Seed,
};

#[test]
fn test_network_type() {
    assert_eq!(HDNetworkType::MainNet.to_string(), "mainnet");
    assert_eq!(HDNetworkType::TestNet.to_string(), "testnet");
}

#[test]
fn test_new_func_bip32_first_account() -> Result<(), Error> {
    let dt = HDPurpose::BIP32;
    let keys = HDKey::new(
        Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]),
        HDNetworkType::MainNet,
        format!("m/{}/{}'/0'", dt, Coin::from(Symbol::BTC).id()),
    )?;

    assert_eq!(
        keys,
        HDKey {
            master_seed: Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238
            ]),
            derivation_path: HDPath::from_str("m/0'/0'/0'")?,
            chain_code: [
                232, 52, 107, 14, 44, 22, 8, 59, 174, 66, 87, 0, 203, 147, 163, 167, 84, 231, 203,
                92, 107, 241, 154, 155, 115, 40, 57, 109, 88, 159, 240, 240
            ],
            depth: 3,
            parent_fingerprint: [107, 29, 72, 246],
            extended_private_key: Some(ExtendedPrivateKey::from_slice(&[
                192, 250, 8, 248, 220, 160, 148, 114, 210, 240, 91, 48, 42, 71, 243, 28, 64, 173,
                186, 85, 26, 141, 214, 240, 128, 27, 225, 155, 145, 56, 237, 101
            ])?),
            extended_public_key: Some(ExtendedPublicKey::from_slice(&[
                2, 134, 68, 19, 216, 122, 40, 153, 49, 141, 8, 93, 145, 229, 90, 54, 99, 218, 63,
                46, 66, 210, 6, 3, 180, 128, 2, 30, 250, 181, 84, 87, 185
            ])?),
            child_index: 2147483648,
            network: HDNetworkType::MainNet,
            derivation_purpose: HDPurpose::BIP32,
        }
    );
    Ok(())
}

#[test]
fn test_bip32_first_account_derive() -> Result<(), Error> {
    let dt = HDPurpose::BIP32;
    let keys = HDKey::new_master(
        Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]),
        HDNetworkType::MainNet,
    )?;
    assert_eq!(
        keys.derive(format!("m/{}/{}'/0'", dt, Coin::from(Symbol::BTC).id()))?,
        HDKey {
            master_seed: Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238
            ]),
            derivation_path: HDPath::from_str("m/0'/0'/0'")?,
            chain_code: [
                232, 52, 107, 14, 44, 22, 8, 59, 174, 66, 87, 0, 203, 147, 163, 167, 84, 231, 203,
                92, 107, 241, 154, 155, 115, 40, 57, 109, 88, 159, 240, 240
            ],
            depth: 3,
            parent_fingerprint: [107, 29, 72, 246],
            extended_private_key: Some(ExtendedPrivateKey::from_slice(&[
                192, 250, 8, 248, 220, 160, 148, 114, 210, 240, 91, 48, 42, 71, 243, 28, 64, 173,
                186, 85, 26, 141, 214, 240, 128, 27, 225, 155, 145, 56, 237, 101
            ])?),
            extended_public_key: Some(ExtendedPublicKey::from_slice(&[
                2, 134, 68, 19, 216, 122, 40, 153, 49, 141, 8, 93, 145, 229, 90, 54, 99, 218, 63,
                46, 66, 210, 6, 3, 180, 128, 2, 30, 250, 181, 84, 87, 185
            ])?),
            child_index: 2147483648,
            network: HDNetworkType::MainNet,
            derivation_purpose: HDPurpose::BIP32,
        }
    );

    Ok(())
}

#[test]
fn test_new_func_first_bip32_address() -> Result<(), Error> {
    let dt = HDPurpose::BIP32;
    let keys = HDKey::new(
        Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]),
        HDNetworkType::MainNet,
        dt.default_path_specify(Coin::Bitcoin.id(), 0, 0, 0),
    )?;

    assert_eq!(
        keys,
        HDKey {
            master_seed: Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238
            ]),
            derivation_path: HDPath::from_str("m/0'/0'/0'/0/0")?,
            chain_code: [
                77, 157, 183, 97, 179, 135, 148, 182, 249, 135, 66, 7, 35, 20, 70, 206, 27, 66, 0,
                133, 246, 255, 179, 36, 121, 22, 245, 17, 169, 178, 56, 73
            ],
            depth: 5,
            parent_fingerprint: [252, 17, 0, 152],
            extended_private_key: Some(ExtendedPrivateKey::from_slice(&[
                37, 137, 71, 12, 145, 160, 177, 51, 192, 93, 77, 95, 253, 188, 73, 141, 60, 223,
                118, 144, 156, 92, 95, 18, 7, 104, 131, 208, 25, 158, 233, 219
            ])?),
            extended_public_key: Some(ExtendedPublicKey::from_slice(&[
                2, 232, 62, 185, 87, 185, 189, 35, 206, 203, 149, 71, 11, 176, 241, 36, 100, 0,
                201, 165, 200, 202, 72, 77, 132, 229, 128, 178, 82, 207, 191, 60, 8
            ])?),
            child_index: 0,
            network: HDNetworkType::MainNet,
            derivation_purpose: HDPurpose::BIP32,
        }
    );

    Ok(())
}

#[test]
fn test_derive_first_bip32_address() -> Result<(), Error> {
    let dt = HDPurpose::BIP32;
    let keys = HDKey::new_master(
        Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]),
        HDNetworkType::MainNet,
    )?;

    assert_eq!(
        keys.derive(dt.default_path_specify(Coin::Bitcoin.id(), 0, 0, 0))?,
        HDKey {
            master_seed: Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238
            ]),
            derivation_path: HDPath::from_str("m/0'/0'/0'/0/0")?,
            chain_code: [
                77, 157, 183, 97, 179, 135, 148, 182, 249, 135, 66, 7, 35, 20, 70, 206, 27, 66, 0,
                133, 246, 255, 179, 36, 121, 22, 245, 17, 169, 178, 56, 73
            ],
            depth: 5,
            parent_fingerprint: [252, 17, 0, 152],
            extended_private_key: Some(ExtendedPrivateKey::from_slice(&[
                37, 137, 71, 12, 145, 160, 177, 51, 192, 93, 77, 95, 253, 188, 73, 141, 60, 223,
                118, 144, 156, 92, 95, 18, 7, 104, 131, 208, 25, 158, 233, 219
            ])?),
            extended_public_key: Some(ExtendedPublicKey::from_slice(&[
                2, 232, 62, 185, 87, 185, 189, 35, 206, 203, 149, 71, 11, 176, 241, 36, 100, 0,
                201, 165, 200, 202, 72, 77, 132, 229, 128, 178, 82, 207, 191, 60, 8
            ])?),
            child_index: 0,
            network: HDNetworkType::MainNet,
            derivation_purpose: HDPurpose::BIP32,
        }
    );

    Ok(())
}

#[test]
fn test_derive_change_internal_chain() -> Result<(), Error> {
    let dt = HDPurpose::BIP32;
    let derived = HDKey::new(
        Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]),
        HDNetworkType::MainNet,
        dt.default_path_specify(Coin::Bitcoin.id(), 0, 1, 0),
    )?;
    assert_eq!(
        derived.derivation_path.to_string(),
        "m/0'/0'/0'/1/0".to_string()
    );
    assert_eq!(&derived.extended_private_key_serialized()?, "xprvA47jwGZNLdTnKuMGfLdeYMV7dgAF9gCjYUNYGeAjJuXrRbj1MULdePjyC5nH7Pp2GTRqnXqkumeJC29fRVVSJmbrWDUENyRG22n1tJdn5b7");
    assert_eq!(&derived.extended_public_key_serialized()?, "xpub6H76Ln6GB125YPRjmNAeuVRrBhzjZ8vauhJ952aLsF4qJQ49u1etCC4T3KkvysShJwgdPL3B5fEsiZJCeymY1Z2wfUNXN77ksN9oqLP9PU3");
    assert_eq!(
        &derived.to_wif()?,
        "L36tbAQoqCpU4rHQuyhYmRHscbcrSc31HXefsrMUaXco8Wqfpaqf"
    );
    assert_eq!(
        format!("{:x}", derived.extended_public_key()?),
        "0224c0180e484ca64cea39fc471a02bf286196e12d10f08dfa18bdc995f0707cad"
    );
    Ok(())
}

#[test]
fn test_eth_bip44() -> Result<(), Error> {
    let dpath = HDPath::builder()
        .purpose(HDPurpose::BIP44.to_shortform_num())
        .coin_type_index(Coin::from(Symbol::ETH).id())
        .build()
        .to_string();

    let derived_key = HDKey::new(
        Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]),
        HDNetworkType::MainNet,
        dpath,
    )?;

    assert_eq!(
        derived_key,
        HDKey {
            master_seed: Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238
            ]),
            derivation_path: HDPath::from_str("m/44h/60h/0h/0/0")?,
            chain_code: [
                109, 150, 159, 21, 145, 38, 169, 238, 94, 27, 158, 36, 221, 164, 167, 226, 84, 253,
                81, 90, 210, 254, 84, 178, 233, 164, 217, 131, 149, 75, 168, 105
            ],
            depth: 5,
            parent_fingerprint: [219, 127, 235, 119],
            extended_private_key: Some(ExtendedPrivateKey::from_slice(&[
                165, 220, 218, 239, 160, 128, 19, 9, 44, 163, 125, 63, 96, 212, 111, 39, 81, 13,
                248, 119, 122, 58, 125, 214, 161, 185, 243, 115, 53, 44, 170, 117
            ])?),
            extended_public_key: Some(ExtendedPublicKey::from_slice(&[
                3, 237, 181, 7, 68, 80, 173, 147, 6, 71, 14, 89, 107, 91, 14, 126, 178, 36, 245,
                197, 197, 57, 113, 112, 101, 150, 46, 195, 101, 233, 63, 6, 97
            ])?),
            child_index: 0,
            network: HDNetworkType::MainNet,
            derivation_purpose: HDPurpose::BIP44,
        }
    );
    Ok(())
}

#[test]
fn test_bip49_first_account() -> Result<(), Error> {
    let mut path_builder = HDPath::builder();
    path_builder
        .purpose(HDPurpose::BIP49.to_shortform_num())
        .coin_type_index(Coin::from(Symbol::BTC).id())
        .no_change_index()
        .no_address_index();

    let master_key = HDKey::new_master(
        Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]),
        HDNetworkType::MainNet,
    )?;

    assert_eq!(master_key.derivation_path.to_string(), "m".to_string());

    let first_account = master_key.derive(path_builder.build().to_string())?;
    assert_eq!(
        first_account.derivation_path.to_string(),
        "m/49'/0'/0'".to_string()
    );
    assert_eq!(&first_account.extended_private_key_serialized()?, "yprvAKG7rKYNjGJwgbJsjEADohvBMDupqc4J3hAopkwqyfKk73voyQzBNuVFLJFUPVrpm3ei2H1cQCBP1oiAaasyNc9UoPzasnScRfeZxFDT4Tf");
    assert_eq!(&first_account.extended_public_key_serialized()?, "ypub6YFUFq5GZdsEu5PLqFhEAqruuFkKF4n9Qv6Qd9MTXzriyrFxWxJRvhojBZgZ7Wj5Ak9ow69A8QXhiZoRu41FvXdK7rv8cXJo5PoLwqNywF5");
    Ok(())
}

#[test]

fn test_bip49_address_one() -> Result<(), Error> {
    let mut path_builder = HDPath::builder();
    path_builder
        .purpose(HDPurpose::BIP49.to_shortform_num())
        .coin_type_index(Coin::from(Symbol::BTC).id())
        .address_index(1);

    let derived = HDKey::new(
        Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]),
        HDNetworkType::MainNet,
        path_builder.build().to_string(),
    )?;

    assert_eq!(
        derived.derivation_path.to_string(),
        "m/49'/0'/0'/0/1".to_string()
    );
    assert_eq!(
        &derived.to_wif()?,
        "KzzMux1HnhZCAiCLScSpUDtXtsHgjts4RJLadDDi2zgxU2qq3g53"
    );
    assert_eq!(
        format!("{:x}", derived.extended_public_key()?),
        "02b9a730f83f85b77c7cf2f444d6cf76b144e11370bb96c6cbc624072f2d8e94cc"
    );
    Ok(())
}
