pub mod hd_key;
pub mod slip44;
use anyhow::anyhow;
pub use hd_key::HDKey;
pub use slip44::SlipCoin;
use std::fmt;

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub enum DeriveType {
    #[default]
    BIP32,
    BIP44,
    BIP49,
    BIP84,
}
impl DeriveType {
    /// Returns the purppose string representation associated with each derivation type
    pub fn purpose(&self) -> &str {
        match self {
            DeriveType::BIP32 => "0'",
            DeriveType::BIP44 => "44'",
            DeriveType::BIP49 => "49'",
            DeriveType::BIP84 => "84'",
        }
    }
    /// Derives the default first account with the specified derivation path scheme
    pub fn derive_first_account(
        &self,
        master_node: &HDKey,
        coin: &SlipCoin,
    ) -> Result<HDKey, anyhow::Error> {
        let derived_account_path = format!("m/{}/{}'/0'", &self.purpose(), coin);
        HDKey::derived_from_master_with_specified_path(&master_node, derived_account_path)
    }

    // Derives the default first address with the specified derivation path scheme
    pub fn derive_first_address(
        &self,
        master_node: &HDKey,
        coin: &SlipCoin,
    ) -> Result<HDKey, anyhow::Error> {
        let derived_first_account = &self.derive_first_account(master_node, coin)?;
        println!(
            "First Derived Account HD Key Info: \n{}",
            derived_first_account
        );
        let deriv_path = format!("m/{}/{}'/0'/0/0", &self.purpose(), coin);
        HDKey::derived_from_master_with_specified_path(&master_node, deriv_path)
    }

    pub fn derive_specify_account_address_indices(
        &self,
        master_node: &HDKey,
        coin: &SlipCoin,
        account_index: usize,
        address_index: usize,
    ) -> Result<HDKey, anyhow::Error> {
        let derived_path = format!(
            "m/{}/{}'/{}'/0/{}",
            &self.purpose(),
            coin,
            account_index,
            address_index
        );
        HDKey::derived_from_master_with_specified_path(&master_node, derived_path)
    }

    pub fn derive_specify_change_account_address_indices(
        &self,
        master_node: &HDKey,
        coin: &SlipCoin,
        change_index: usize,
        account_index: usize,
        address_index: usize,
    ) -> Result<HDKey, anyhow::Error> {
        let derived_path = format!(
            "m/{}/{}'/{}'/{}/{}",
            &self.purpose(),
            coin,
            account_index,
            change_index,
            address_index
        );
        HDKey::derived_from_master_with_specified_path(&master_node, derived_path)
    }

    pub fn derive_change_internal_chain_specify_account_address_indices(
        &self,
        master_node: &HDKey,
        coin: &SlipCoin,
        account_index: usize,
        address_index: usize,
    ) -> Result<HDKey, anyhow::Error> {
        let derived_path = format!(
            "m/{}/{}/{}'/1/{}",
            &self.purpose(),
            coin,
            account_index,
            address_index
        );
        HDKey::derived_from_master_with_specified_path(&master_node, derived_path)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DerivPathComponent {
    Master,
    IndexHardened(u32),
    IndexNotHardened(u32),
}

#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum NetworkType {
    #[default]
    MainNet,
    TestNet,
}

impl fmt::Display for NetworkType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkType::MainNet => fmt.write_str("mainnet")?,
            NetworkType::TestNet => fmt.write_str("testnet")?,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_type() {
        assert_eq!(NetworkType::MainNet.to_string(), "mainnet");
        assert_eq!(NetworkType::TestNet.to_string(), "testnet");
    }

    #[test]
    fn test_derive_type() {
        let dt = DeriveType::BIP32;
        assert_eq!(dt.purpose(), "0'");
    }

    #[test]
    fn test_derive_first_account() {
        let dt = DeriveType::BIP32;
        let keys = HDKey::new(&[
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ])
        .unwrap();
        assert_eq!(
            dt.derive_first_account(&keys, &SlipCoin::BTC).unwrap(),
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
                extended_private_key: Some([
                    192, 250, 8, 248, 220, 160, 148, 114, 210, 240, 91, 48, 42, 71, 243, 28, 64,
                    173, 186, 85, 26, 141, 214, 240, 128, 27, 225, 155, 145, 56, 237, 101
                ]),
                extended_public_key: Some([
                    2, 134, 68, 19, 216, 122, 40, 153, 49, 141, 8, 93, 145, 229, 90, 54, 99, 218,
                    63, 46, 66, 210, 6, 3, 180, 128, 2, 30, 250, 181, 84, 87, 185
                ]),
                child_index: 2147483648,
                network: NetworkType::MainNet,
                derivation_type: DeriveType::BIP32
            }
        );
    }

    #[test]
    fn test_derive_first_address() {
        let dt = DeriveType::BIP32;
        let keys = HDKey::new(&[
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ])
        .unwrap();
        assert_eq!(
            dt.derive_first_address(&keys, &SlipCoin::BTC).unwrap(),
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
                extended_private_key: Some([
                    37, 137, 71, 12, 145, 160, 177, 51, 192, 93, 77, 95, 253, 188, 73, 141, 60,
                    223, 118, 144, 156, 92, 95, 18, 7, 104, 131, 208, 25, 158, 233, 219
                ]),
                extended_public_key: Some([
                    2, 232, 62, 185, 87, 185, 189, 35, 206, 203, 149, 71, 11, 176, 241, 36, 100, 0,
                    201, 165, 200, 202, 72, 77, 132, 229, 128, 178, 82, 207, 191, 60, 8
                ]),
                child_index: 0,
                network: NetworkType::MainNet,
                derivation_type: DeriveType::BIP32
            }
        );
    }

    #[test]
    fn test_derive_specify_account_address_indices() {
        let dt = DeriveType::BIP32;
        let keys = HDKey::new(&[
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ])
        .unwrap();
        assert_eq!(
            dt.derive_specify_account_address_indices(&keys, &SlipCoin::BTC, 0, 0)
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
                extended_private_key: Some([
                    37, 137, 71, 12, 145, 160, 177, 51, 192, 93, 77, 95, 253, 188, 73, 141, 60,
                    223, 118, 144, 156, 92, 95, 18, 7, 104, 131, 208, 25, 158, 233, 219
                ]),
                extended_public_key: Some([
                    2, 232, 62, 185, 87, 185, 189, 35, 206, 203, 149, 71, 11, 176, 241, 36, 100, 0,
                    201, 165, 200, 202, 72, 77, 132, 229, 128, 178, 82, 207, 191, 60, 8
                ]),
                child_index: 0,
                network: NetworkType::MainNet,
                derivation_type: DeriveType::BIP32
            }
        );
    }

    #[test]
    fn test_derive_specify_change_account_address_indices() {
        let dt = DeriveType::BIP32;
        let keys = HDKey::new(&[
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ])
        .unwrap();
        assert_eq!(
            dt.derive_specify_change_account_address_indices(&keys, &SlipCoin::BTC, 0, 0, 0)
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
                extended_private_key: Some([
                    37, 137, 71, 12, 145, 160, 177, 51, 192, 93, 77, 95, 253, 188, 73, 141, 60,
                    223, 118, 144, 156, 92, 95, 18, 7, 104, 131, 208, 25, 158, 233, 219
                ]),
                extended_public_key: Some([
                    2, 232, 62, 185, 87, 185, 189, 35, 206, 203, 149, 71, 11, 176, 241, 36, 100, 0,
                    201, 165, 200, 202, 72, 77, 132, 229, 128, 178, 82, 207, 191, 60, 8
                ]),
                child_index: 0,
                network: NetworkType::MainNet,
                derivation_type: DeriveType::BIP32
            }
        );
    }

    #[test]
    fn test_derive_change_internal_chain_specify_account_address_indices() {
        let dt = DeriveType::BIP32;
        let keys = HDKey::new(&[
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ])
        .unwrap();
        assert_eq!(
            dt.derive_change_internal_chain_specify_account_address_indices(
                &keys,
                &SlipCoin::BTC,
                0,
                0
            )
            .unwrap(),
            HDKey {
                master_seed: [
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]
                .to_vec(),
                derivation_path: "m/0'/0/0'/1/0".to_string(),
                chain_code: [
                    104, 27, 25, 16, 119, 159, 11, 95, 70, 109, 30, 115, 223, 242, 167, 113, 227,
                    177, 108, 150, 43, 156, 211, 165, 163, 236, 223, 47, 193, 150, 209, 208
                ],
                depth: 5,
                parent_fingerprint: [192, 16, 102, 140],
                extended_private_key: Some([
                    105, 19, 181, 242, 111, 207, 235, 171, 181, 93, 101, 83, 17, 229, 102, 204, 39,
                    162, 171, 156, 220, 186, 53, 24, 163, 199, 238, 239, 16, 52, 56, 120
                ]),
                extended_public_key: Some([
                    2, 244, 190, 229, 119, 71, 89, 191, 63, 241, 234, 200, 223, 245, 23, 120, 209,
                    176, 222, 183, 173, 68, 237, 102, 204, 32, 248, 50, 40, 173, 116, 81, 207
                ]),
                child_index: 0,
                network: NetworkType::MainNet,
                derivation_type: DeriveType::BIP32
            }
        );
    }
}
