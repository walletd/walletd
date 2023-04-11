use std::fmt::{self, Display};
use std::str::FromStr;

use crate::Error;

/// The DeriveType enum represents the different derivation path schemes
/// supported by the library.
///
/// BIP32 is the default derivation scheme which uses a purpose value of 0'
/// BIP44: https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
/// BIP49: https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki
/// BIP84: https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki
#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum HDPurpose {
    #[default]
    BIP32,
    BIP44,
    BIP49,
    BIP84,
}

impl HDPurpose {
    /// Returns the purpose representation associated with each
    /// derivation type as a DerivPathComponent
    pub fn purpose(&self) -> HDPathIndex {
        match self {
            HDPurpose::BIP32 => HDPathIndex::IndexHardened(0),
            HDPurpose::BIP44 => HDPathIndex::IndexHardened(44),
            HDPurpose::BIP49 => HDPathIndex::IndexHardened(49),
            HDPurpose::BIP84 => HDPathIndex::IndexHardened(84),
        }
    }

    /// Returns a string specifying the derivation path, given the shorform
    /// index values for the coin_id, account, change and address index This
    /// function uses a hardened index for the coin id and acount and a
    /// non-hardened index for the change and address index
    // TODO(AS): Change or augment this to use the builder pattern
    pub fn full_deriv_path(&self, coin_id: u32, account: u32, change: u32, index: u32) -> String {
        format!(
            "m/{}/{}/{}/{}/{}",
            self.purpose(),
            HDPathIndex::IndexHardened(coin_id),
            HDPathIndex::IndexHardened(account),
            HDPathIndex::IndexNotHardened(change),
            HDPathIndex::IndexNotHardened(index)
        )
    }
}

impl FromStr for HDPurpose {
    type Err = Error;

    /// TODO(AS): allow parsing the character h the same as '
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "0'" => Ok(HDPurpose::BIP32),
            "44'" => Ok(HDPurpose::BIP44),
            "49'" => Ok(HDPurpose::BIP49),
            "84'" => Ok(HDPurpose::BIP84),
            _ => Err(Error::FromStr(format!(
                "Unknown purpose, unknown deriv type {}",
                s
            ))),
        }
    }
}

/// The DerivePathComponent distinguishes between the different derivation path
/// components.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HDPathIndex {
    Master,
    IndexHardened(u32),
    IndexNotHardened(u32),
}

impl HDPathIndex {
    /// Convert to the full number used to represent a hardend index from the
    /// number used in the derivation path string accompanied by ' to indicate
    /// hardened
    pub fn hardened_full_index(num: u32) -> u32 {
        num + (1 << 31)
    }

    /// Convert from the full number used represent a hardened index to the
    /// number when accompanied by ' indicates a hardened index
    pub fn hardened_shortform_index(full_index: u32) -> u32 {
        full_index - (1 << 31)
    }

    /// Returns the short form value of index, for master type always returns 0,
    /// for hardened index returns the short form value without the hardened
    /// indicator, the value here for hardened index is not the same as the full
    /// index number which is used in the calculation but rather the short form
    /// value used in the derivation string when accompanied by the ' indicator
    pub fn to_shortform_index(&self) -> u32 {
        match self {
            HDPathIndex::Master => 0,
            HDPathIndex::IndexHardened(num) => *num,
            HDPathIndex::IndexNotHardened(num) => *num,
        }
    }
}

impl Display for HDPathIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HDPathIndex::Master => {
                write!(f, "m")?;
            }
            HDPathIndex::IndexHardened(num) => {
                write!(f, "{}'", num)?;
            }
            HDPathIndex::IndexNotHardened(num) => {
                write!(f, "{}", num)?;
            }
        }
        Ok(())
    }
}

impl FromStr for HDPathIndex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "m" {
            return Ok(HDPathIndex::Master);
        }
        let chars = s.chars();
        let mut is_hardened = false;
        let mut num = String::new();
        for c in chars {
            if c == '\'' {
                is_hardened = true;
            } else {
                num.push(c);
            }
        }
        let num: u32 = num
            .parse::<u32>()
            .map_err(|e| Error::FromStr(e.to_string()))?;
        if is_hardened {
            Ok(HDPathIndex::IndexHardened(num))
        } else {
            Ok(HDPathIndex::IndexNotHardened(num))
        }
    }
}

#[cfg(test)]
mod tests {
    use slip44::{Coin, Symbol};

    use super::*;
    use crate::{ExtendedPrivateKey, ExtendedPublicKey, HDKey, HDNetworkType};

    #[test]
    fn test_network_type() {
        assert_eq!(HDNetworkType::MainNet.to_string(), "mainnet");
        assert_eq!(HDNetworkType::TestNet.to_string(), "testnet");
    }

    #[test]
    fn test_derive_type() {
        let dt = HDPurpose::BIP32;
        assert_eq!(format!("{}", dt.purpose()), "0'");
    }

    #[test]
    fn test_derive_first_account() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            HDNetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            keys.derive(format!(
                "m/{}/{}'/0'",
                dt.purpose(),
                Coin::from(Symbol::BTC).id()
            ))
            .unwrap(),
            HDKey {
                master_seed: [
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]
                .to_vec(),
                derivation_path: "m/0'/0'/0'".to_string(),
                chain_code: [
                    232, 52, 107, 14, 44, 22, 8, 59, 174, 66, 87, 0, 203, 147, 163, 167, 84, 231,
                    203, 92, 107, 241, 154, 155, 115, 40, 57, 109, 88, 159, 240, 240
                ],
                depth: 3,
                parent_fingerprint: [107, 29, 72, 246],
                extended_private_key: Some(
                    ExtendedPrivateKey::from_slice(&[
                        192, 250, 8, 248, 220, 160, 148, 114, 210, 240, 91, 48, 42, 71, 243, 28,
                        64, 173, 186, 85, 26, 141, 214, 240, 128, 27, 225, 155, 145, 56, 237, 101
                    ])
                    .unwrap()
                ),
                extended_public_key: Some(
                    ExtendedPublicKey::from_slice(&[
                        2, 134, 68, 19, 216, 122, 40, 153, 49, 141, 8, 93, 145, 229, 90, 54, 99,
                        218, 63, 46, 66, 210, 6, 3, 180, 128, 2, 30, 250, 181, 84, 87, 185
                    ])
                    .unwrap()
                ),
                child_index: 2147483648,
                network: HDNetworkType::MainNet,
                derivation_type: HDPurpose::BIP32
            }
        );
    }

    // TODO(AS): some of these tests might be duplicates of the same functions,
    // update this after refactoring this module
    #[test]
    fn test_derive_first_address() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            HDNetworkType::MainNet,
        )
        .unwrap();

        assert_eq!(
            keys.derive(dt.full_deriv_path(Coin::Bitcoin.id(), 0, 0, 0))
                .unwrap(),
            HDKey {
                master_seed: [
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]
                .to_vec(),
                derivation_path: "m/0'/0'/0'/0/0".to_string(),
                chain_code: [
                    77, 157, 183, 97, 179, 135, 148, 182, 249, 135, 66, 7, 35, 20, 70, 206, 27, 66,
                    0, 133, 246, 255, 179, 36, 121, 22, 245, 17, 169, 178, 56, 73
                ],
                depth: 5,
                parent_fingerprint: [252, 17, 0, 152],
                extended_private_key: Some(
                    ExtendedPrivateKey::from_slice(&[
                        37, 137, 71, 12, 145, 160, 177, 51, 192, 93, 77, 95, 253, 188, 73, 141, 60,
                        223, 118, 144, 156, 92, 95, 18, 7, 104, 131, 208, 25, 158, 233, 219
                    ])
                    .unwrap()
                ),
                extended_public_key: Some(
                    ExtendedPublicKey::from_slice(&[
                        2, 232, 62, 185, 87, 185, 189, 35, 206, 203, 149, 71, 11, 176, 241, 36,
                        100, 0, 201, 165, 200, 202, 72, 77, 132, 229, 128, 178, 82, 207, 191, 60,
                        8
                    ])
                    .unwrap()
                ),
                child_index: 0,
                network: HDNetworkType::MainNet,
                derivation_type: HDPurpose::BIP32
            }
        );
    }

    #[test]
    fn test_derive_specify_account_address_indices() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            HDNetworkType::MainNet,
        )
        .unwrap();
        let derived = keys
            .derive(dt.full_deriv_path(Coin::Bitcoin.id(), 0, 0, 0))
            .unwrap();
        assert_eq!(
            derived,
            HDKey {
                master_seed: [
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]
                .to_vec(),
                derivation_path: "m/0'/0'/0'/0/0".to_string(),
                chain_code: [
                    77, 157, 183, 97, 179, 135, 148, 182, 249, 135, 66, 7, 35, 20, 70, 206, 27, 66,
                    0, 133, 246, 255, 179, 36, 121, 22, 245, 17, 169, 178, 56, 73
                ],
                depth: 5,
                parent_fingerprint: [252, 17, 0, 152],
                extended_private_key: Some(
                    ExtendedPrivateKey::from_slice(&[
                        37, 137, 71, 12, 145, 160, 177, 51, 192, 93, 77, 95, 253, 188, 73, 141, 60,
                        223, 118, 144, 156, 92, 95, 18, 7, 104, 131, 208, 25, 158, 233, 219
                    ])
                    .unwrap()
                ),
                extended_public_key: Some(
                    ExtendedPublicKey::from_slice(&[
                        2, 232, 62, 185, 87, 185, 189, 35, 206, 203, 149, 71, 11, 176, 241, 36,
                        100, 0, 201, 165, 200, 202, 72, 77, 132, 229, 128, 178, 82, 207, 191, 60,
                        8
                    ])
                    .unwrap()
                ),
                child_index: 0,
                network: HDNetworkType::MainNet,
                derivation_type: HDPurpose::BIP32
            }
        );
    }

    #[test]
    fn test_derive_specify_change_account_address_indices() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            HDNetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            keys.derive(dt.full_deriv_path(Coin::Bitcoin.id(), 0, 0, 0))
                .unwrap(),
            HDKey {
                master_seed: [
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]
                .to_vec(),
                derivation_path: "m/0'/0'/0'/0/0".to_string(),
                chain_code: [
                    77, 157, 183, 97, 179, 135, 148, 182, 249, 135, 66, 7, 35, 20, 70, 206, 27, 66,
                    0, 133, 246, 255, 179, 36, 121, 22, 245, 17, 169, 178, 56, 73
                ],
                depth: 5,
                parent_fingerprint: [252, 17, 0, 152],
                extended_private_key: Some(
                    ExtendedPrivateKey::from_slice(&[
                        37, 137, 71, 12, 145, 160, 177, 51, 192, 93, 77, 95, 253, 188, 73, 141, 60,
                        223, 118, 144, 156, 92, 95, 18, 7, 104, 131, 208, 25, 158, 233, 219
                    ])
                    .unwrap()
                ),
                extended_public_key: Some(
                    ExtendedPublicKey::from_slice(&[
                        2, 232, 62, 185, 87, 185, 189, 35, 206, 203, 149, 71, 11, 176, 241, 36,
                        100, 0, 201, 165, 200, 202, 72, 77, 132, 229, 128, 178, 82, 207, 191, 60,
                        8
                    ])
                    .unwrap()
                ),
                child_index: 0,
                network: HDNetworkType::MainNet,
                derivation_type: HDPurpose::BIP32
            }
        );
    }

    #[test]
    fn test_derive_change_internal_chain_specify_account_address_indices() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            HDNetworkType::MainNet,
        )
        .unwrap();
        let derived = keys
            .derive(dt.full_deriv_path(Coin::Bitcoin.id(), 0, 1, 0))
            .unwrap();
        assert_eq!(derived.master_seed, keys.master_seed);
        assert_eq!(derived.derivation_path, "m/0'/0'/0'/1/0".to_string());
        assert_eq!(&derived.extended_private_key_serialized().unwrap(), "xprvA47jwGZNLdTnKuMGfLdeYMV7dgAF9gCjYUNYGeAjJuXrRbj1MULdePjyC5nH7Pp2GTRqnXqkumeJC29fRVVSJmbrWDUENyRG22n1tJdn5b7");
        assert_eq!(&derived.extended_public_key_serialized().unwrap(), "xpub6H76Ln6GB125YPRjmNAeuVRrBhzjZ8vauhJ952aLsF4qJQ49u1etCC4T3KkvysShJwgdPL3B5fEsiZJCeymY1Z2wfUNXN77ksN9oqLP9PU3");
        assert_eq!(derived.derivation_type, HDPurpose::BIP32);
        assert_eq!(
            &derived.to_wif().unwrap(),
            "L36tbAQoqCpU4rHQuyhYmRHscbcrSc31HXefsrMUaXco8Wqfpaqf"
        );
        assert_eq!(
            format!("{:x}", derived.extended_public_key().unwrap()),
            "0224c0180e484ca64cea39fc471a02bf286196e12d10f08dfa18bdc995f0707cad"
        );
    }
}
