use std::fmt::{self, Display};
use std::str::FromStr;

use crate::Error;

/// The DeriveType enum represents the different derivation path schemes
/// supported by the library.
///
/// BIP32 is the default derivation scheme which uses a purpose value of 0'
/// BIP44: <https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki>
/// BIP49: <https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki>
/// BIP84: <https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki>
#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum HDPurpose {
    #[default]
    BIP32,
    BIP44,
    BIP49,
    BIP84,
}

impl HDPurpose {
    /// Returns the purpose value as a u32 shortform index value (this is the
    /// value used in the derivation path string)
    pub fn to_shortform_num(&self) -> u32 {
        let path_index: HDPathIndex = self.into();
        path_index.to_shortform_num()
    }

    /// Returns the purpose value as a full u32 num (this is the value used in
    /// the calculation)
    pub fn to_full_num(&self) -> u32 {
        let path_index: HDPathIndex = self.into();
        path_index.to_full_num()
    }

    /// Returns a string specifying the derivation path, given the shorform
    /// index values for the coin_id, account, change and address index.
    /// Thi function uses a hardened index for the purpose, coin id and account
    /// and a non-hardened index for the change and address index
    pub fn default_path_specify(
        &self,
        coin_id: u32,
        account: u32,
        change: u32,
        address_index: u32,
    ) -> String {
        format!(
            "m/{}/{}/{}/{}/{}",
            self,
            HDPathIndex::IndexHardened(coin_id),
            HDPathIndex::IndexHardened(account),
            HDPathIndex::IndexNotHardened(change),
            HDPathIndex::IndexNotHardened(address_index)
        )
    }
}

impl From<&HDPurpose> for HDPathIndex {
    fn from(purpose: &HDPurpose) -> Self {
        match purpose {
            HDPurpose::BIP32 => HDPathIndex::IndexHardened(0),
            HDPurpose::BIP44 => HDPathIndex::IndexHardened(44),
            HDPurpose::BIP49 => HDPathIndex::IndexHardened(49),
            HDPurpose::BIP84 => HDPathIndex::IndexHardened(84),
        }
    }
}

impl TryFrom<HDPathIndex> for HDPurpose {
    type Error = Error;

    fn try_from(path_index: HDPathIndex) -> Result<Self, Error> {
        match path_index {
            HDPathIndex::IndexHardened(0) => Ok(HDPurpose::BIP32),
            HDPathIndex::IndexHardened(44) => Ok(HDPurpose::BIP44),
            HDPathIndex::IndexHardened(49) => Ok(HDPurpose::BIP49),
            HDPathIndex::IndexHardened(84) => Ok(HDPurpose::BIP84),
            _ => Err(Error::Invalid(format!(
                "Cannot convert {} to HDPurpose",
                path_index
            ))),
        }
    }
}

impl FromStr for HDPurpose {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "0'" | "0h" => Ok(HDPurpose::BIP32),
            "44'" | "44h" => Ok(HDPurpose::BIP44),
            "49'" | "49h" => Ok(HDPurpose::BIP49),
            "84'" | "84h" => Ok(HDPurpose::BIP84),
            _ => Err(Error::FromStr(format!(
                "Unknown purpose, unknown deriv type {}",
                s
            ))),
        }
    }
}

impl fmt::Display for HDPurpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let path_index: HDPathIndex = self.into();
        write!(f, "{}", path_index)
    }
}

/// The HDPathIndex distinguishes between the different derivation path
/// components.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HDPathIndex {
    /// Master index is the root of the derivation tree, it is represented as m
    /// in the string path
    Master,
    /// IndexHardened is a hardened index, it is represented as a number
    /// followed by ' in the string path The number is the short form value,
    /// the full index value is the short form value + 2^31
    IndexHardened(u32),
    /// IndexNotHardened is a non-hardened index, it is represented as a number
    /// in the string path The number is the short form value, the full
    /// index value is the same short form value when the index is not hardened
    IndexNotHardened(u32),
}

impl HDPathIndex {
    /// Convert to the full number used to represent a hardend index from the
    /// number used in the derivation path string accompanied by ' to indicate
    /// hardened
    pub fn hardened_full_num(num: u32) -> u32 {
        num + (1 << 31)
    }

    /// Convert from the full number used represent a hardened index to the
    /// number when accompanied by ' indicates a hardened index
    pub fn hardened_shortform_num(full_num: u32) -> u32 {
        full_num - (1 << 31)
    }

    /// Returns the short form value of index, for master type always returns 0,
    /// for hardened index returns the short form value without the hardened
    /// indicator, the value here for hardened index is not the same as the full
    /// index number which is used in the calculation but rather the short form
    /// value used in the derivation string when accompanied by the ' indicator
    pub fn to_shortform_num(&self) -> u32 {
        match self {
            HDPathIndex::Master => 0,
            HDPathIndex::IndexHardened(num) => *num,
            HDPathIndex::IndexNotHardened(num) => *num,
        }
    }

    /// Returns the full index value, for non-hardened index this is the same as
    /// the short form value, for hardened index this is the full index
    /// value used in the calculation
    pub fn to_full_num(&self) -> u32 {
        match self {
            HDPathIndex::Master => 0,
            HDPathIndex::IndexHardened(num) => HDPathIndex::hardened_full_num(*num),
            HDPathIndex::IndexNotHardened(num) => *num,
        }
    }

    /// Creates a master HDPathIndex
    pub fn new_master() -> HDPathIndex {
        HDPathIndex::Master
    }

    /// Creates a new index (not master) from the short form index and a boolean
    /// which indicates if the index is hardened or not
    /// # Arguments
    /// * `num` - the short form index value, the value here for hardened index
    ///   is not the same as the full index number which is used in the
    ///   calculation but rather the short form value used in the derivation
    ///   string when accompanied by the ' indicator
    /// * `hardened` - boolean indicating if the index is hardened or not
    pub fn new_index(num: u32, hardened: bool) -> HDPathIndex {
        if hardened {
            HDPathIndex::IndexHardened(num)
        } else {
            HDPathIndex::IndexNotHardened(num)
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
            if c == '\'' || c == 'h' {
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

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HDPath {
    path: Vec<HDPathIndex>,
}

impl HDPath {
    /// Creates a new HDPath from a string representation of the path
    /// # Arguments
    /// * `path` - the string representation of the path
    /// # Errors
    /// * `Error::FromStr` - if the path string component describing an index is
    ///   invalid
    pub fn new(path: &str) -> Result<Self, Error> {
        HDPath::from_str(path)
    }

    /// Pushes a new HDPathIndex to the path
    pub fn push(&mut self, index: HDPathIndex) {
        self.path.push(index);
    }

    /// Returns the length of the path
    pub fn len(&self) -> usize {
        self.path.len()
    }

    /// Helper function to convert a derivation path string to a list of strings
    /// # Arguments
    /// * `deriv_path` - the derivation path string
    /// # Errors
    /// * `Error::Invalid` - if the derivation path string is invalid
    pub fn derive_path_str_to_list(deriv_path: &str) -> Result<Vec<String>, Error> {
        let deriv_path_list: Vec<String> = deriv_path.split('/').map(|s| s.to_string()).collect();
        if deriv_path_list.is_empty() || deriv_path_list[0] != *"m" {
            return Err(Error::Invalid(format!(
                "Derivation Path {} is Invalid",
                deriv_path
            )));
        }
        Ok(deriv_path_list)
    }

    /// Returns the vector of HDPathIndex
    pub fn to_vec(&self) -> Vec<HDPathIndex> {
        self.path.clone()
    }

    /// Helper function to convert a derivation path string to a vector of
    /// HDPathIndex # Arguments
    /// * `deriv_path` - the derivation path string
    /// # Errors
    /// * `Error::FromStr` - if the path string component describing an index is
    ///   invalid
    pub fn derive_path_str_to_info(deriv_path: &str) -> Result<Vec<HDPathIndex>, Error> {
        let mut deriv_path_info: Vec<HDPathIndex> = Vec::new();
        let deriv_path_list = Self::derive_path_str_to_list(deriv_path)?;
        for item in deriv_path_list {
            deriv_path_info.push(HDPathIndex::from_str(&item)?);
        }
        Ok(deriv_path_info)
    }

    /// Returns the builder for the HDPath (HDPathBuilder)
    pub fn builder() -> HDPathBuilder {
        HDPathBuilder::new()
    }

    /// Returns the HDPathIndex object at the specified position (index) in the
    /// path if it exists, otherwise returns an Error # Arguments
    /// * `index` - the index of the HDPathIndex object to return based on the
    ///   path vector inside the HDPath
    ///
    /// # Errors
    /// Returns an error `Error::IndexOutOfRange` if the index is not valid for
    /// the HDPath object
    pub fn at(&self, index: usize) -> Result<HDPathIndex, Error> {
        if index < self.path.len() {
            Ok(self.path[index])
        } else {
            Err(Error::IndexOutOfRange {
                index,
                max: self.path.len() - 1,
            })
        }
    }

    /// Returns the HDPurpose value related to the purpose attribute, if it
    /// exists in the HDPath # Errors
    /// Returns an error `Error::IndexOutOfRange` if the index 1 is not valid
    /// for the HDPath object
    pub fn purpose(&self) -> Result<HDPurpose, Error> {
        let purpose: HDPurpose = self.at(1)?.try_into()?;
        Ok(purpose)
    }

    /// Returns the HDPathIndex value related to the coin_type attribute, if it
    /// exists in the HDPath # Errors
    /// Returns an error `Error::IndexOutOfRange` if the index is not valid for
    /// the HDPath object
    pub fn coin_type(&self) -> Result<HDPathIndex, Error> {
        self.at(2)
    }

    /// Returns the HDPathIndex value related to the account attribute, if it
    /// exists in the HDPath # Errors
    /// Returns an error `Error::IndexOutOfRange` if the index is not valid for
    /// the HDPath object
    pub fn account(&self) -> Result<HDPathIndex, Error> {
        self.at(3)
    }

    /// Returns the HDPathIndex value related to the change attribute, if it
    /// exists in the HDPath # Errors
    /// Returns an error `Error::IndexOutOfRange` if the index is not valid for
    /// the HDPath object
    pub fn change(&self) -> Result<HDPathIndex, Error> {
        self.at(4)
    }

    /// Returns the HDPathIndex value related to the address attribute, if it
    /// exists in the HDPath # Errors
    /// Returns an error `Error::IndexOutOfRange` if the index is not valid for
    /// the HDPath object
    pub fn address(&self) -> Result<HDPathIndex, Error> {
        self.at(5)
    }
}

impl fmt::Display for HDPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, p) in self.path.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", p)?;
            } else {
                write!(f, "/{}", p)?;
            }
        }
        Ok(())
    }
}

impl FromStr for HDPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut path = Vec::new();
        for p in s.split('/') {
            path.push(HDPathIndex::from_str(p)?);
        }
        Ok(HDPath { path })
    }
}

impl From<Vec<HDPathIndex>> for HDPath {
    fn from(path: Vec<HDPathIndex>) -> Self {
        HDPath { path }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HDPathBuilder {
    /// purpose shortform index value, default is None, it should be specified
    /// based on the HDPurpose
    pub purpose: Option<u32>,
    /// boolean indicating if the purpose index is hardened or not, default is
    /// true (hardened)
    pub purpose_hardened: bool,
    /// coin_type shortform index value, default is None, it should be specified
    /// based on the coin
    pub coin_type: Option<u32>,
    /// boolean indicating if the coin_type index is hardened or not, default is
    /// true (hardened)
    pub coin_type_hardened: bool,
    /// account shortform index value, default is Some(0)
    pub account: Option<u32>,
    /// boolean indicating if the account index is hardened or not, default is
    /// true (hardened)
    pub account_hardened: bool,
    /// change shortform index value, default is Some(0)
    pub change: Option<u32>,
    /// boolean indicating if the change index is hardened or not, default is
    /// false (not hardened)
    pub change_hardened: bool,
    /// address_index shortform index value, default is Some(0)
    pub address_index: Option<u32>,
    /// boolean indicating if the address_index index is hardened or not,
    /// default is false (not hardened)
    pub address_index_hardened: bool,
}

impl Default for HDPathBuilder {
    fn default() -> Self {
        HDPathBuilder {
            purpose: None,
            purpose_hardened: true,
            coin_type: None,
            coin_type_hardened: true,
            account: Some(0),
            account_hardened: true,
            change: Some(0),
            change_hardened: false,
            address_index: Some(0),
            address_index_hardened: false,
        }
    }
}
impl HDPathBuilder {
    /// Creates a new HDPathBuilder with default values
    pub fn new() -> Self {
        HDPathBuilder::default()
    }

    /// Specify the purpose index shortform number value
    pub fn with_purpose(&mut self, purpose: u32) -> &mut Self {
        self.purpose = Some(purpose);
        self
    }

    /// Specify with a boolean if the purpose index is hardened (true) or not
    /// (false)
    pub fn with_purpose_hardened(&mut self, purpose_hardened: bool) -> &mut Self {
        self.purpose_hardened = purpose_hardened;
        self
    }

    /// Specify the coin_type index shortform number value
    pub fn with_coin_type(&mut self, coin_type: u32) -> &mut Self {
        self.coin_type = Some(coin_type);
        self
    }

    /// Specify with a boolean if the coin_type index is hardened (true) or not
    /// (false)
    pub fn with_coin_type_hardened(&mut self, coin_type_hardened: bool) -> &mut Self {
        self.coin_type_hardened = coin_type_hardened;
        self
    }

    /// Specify the account index shortform number value
    pub fn with_account(&mut self, account: u32) -> &mut Self {
        self.account = Some(account);
        self
    }

    /// Specify with a boolean if the account index is hardened (true) or not
    /// (false)
    pub fn with_account_hardened(&mut self, account_hardened: bool) -> &mut Self {
        self.account_hardened = account_hardened;
        self
    }

    /// Specify the change index shortform number value
    pub fn with_change(&mut self, change: u32) -> &mut Self {
        self.change = Some(change);
        self
    }

    /// Specify with a boolean if the change index is hardened (true) or not
    /// (false)
    pub fn with_change_hardened(&mut self, change_hardened: bool) -> &mut Self {
        self.change_hardened = change_hardened;
        self
    }

    /// Specify the address_index index shortform number value
    pub fn with_address_index(&mut self, address_index: u32) -> &mut Self {
        self.address_index = Some(address_index);
        self
    }

    /// Specify with a boolean if the address_index index is hardened (true) or
    /// not (false)
    pub fn with_address_index_hardened(&mut self, address_index_hardened: bool) -> &mut Self {
        self.address_index_hardened = address_index_hardened;
        self
    }

    pub fn set_purpose_none(&mut self) -> &mut Self {
        self.purpose = None;
        self
    }

    pub fn set_coin_type_none(&mut self) -> &mut Self {
        self.coin_type = None;
        self
    }

    pub fn set_account_none(&mut self) -> &mut Self {
        self.account = None;
        self
    }

    pub fn set_change_none(&mut self) -> &mut Self {
        self.change = None;
        self
    }

    pub fn set_address_index_none(&mut self) -> &mut Self {
        self.address_index = None;
        self
    }

    /// Build the HDPath
    /// The HDPath will be built from the values specified in the builder
    /// The HDPath always starts with the Master index (m)
    /// The HDPath will go in order from purpose, coin_type, account, change,
    /// address_index
    ///
    /// If the purpose is not set, the HDPath will return the HDPath with the
    /// Master index only If the coin_type is not set, the HDPath will
    /// return the HDPath with the Master and purpose index It will use the
    /// defaults for the account, change, and address_index unless if something
    /// else has been specified on the builder using the with or set functions.
    /// So in order to build a full HDPath going up to the address index, the
    /// purpose and coin_type must be set.
    pub fn build(&mut self) -> HDPath {
        let mut path = Vec::new();
        path.push(HDPathIndex::Master);
        if let Some(purpose) = self.purpose {
            path.push(HDPathIndex::new_index(purpose, self.purpose_hardened));
            if let Some(coin_type) = self.coin_type {
                path.push(HDPathIndex::new_index(coin_type, self.coin_type_hardened));
                if let Some(account) = self.account {
                    path.push(HDPathIndex::new_index(account, self.account_hardened));
                    if let Some(change) = self.change {
                        path.push(HDPathIndex::new_index(change, self.change_hardened));
                        if let Some(address_index) = self.address_index {
                            path.push(HDPathIndex::new_index(
                                address_index,
                                self.address_index_hardened,
                            ));
                        }
                    }
                }
            }
        }
        HDPath { path }
    }
}

#[cfg(test)]
mod tests {
    use slip44::{Coin, Symbol};

    use super::*;
    use crate::{ExtendedPrivateKey, ExtendedPublicKey, HDKey, HDNetworkType, Seed};

    #[test]
    fn test_network_type() {
        assert_eq!(HDNetworkType::MainNet.to_string(), "mainnet");
        assert_eq!(HDNetworkType::TestNet.to_string(), "testnet");
    }

    #[test]
    fn test_derive_type() {
        let dt = HDPurpose::BIP32;
        assert_eq!(format!("{}", dt), "0'");
    }

    #[test]
    fn test_derive_first_account() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            keys.derive(format!("m/{}/{}'/0'", dt, Coin::from(Symbol::BTC).id()))
                .unwrap(),
            HDKey {
                master_seed: Seed::new(vec![
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]),
                derivation_path: HDPath::from_str("m/0'/0'/0'").unwrap(),
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
                derivation_purpose: HDPurpose::BIP32,
            }
        );
    }

    // TODO(AS): some of these tests might be duplicates of the same functions,
    // update this after refactoring this module
    #[test]
    fn test_derive_first_address() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();

        assert_eq!(
            keys.derive(dt.default_path_specify(Coin::Bitcoin.id(), 0, 0, 0))
                .unwrap(),
            HDKey {
                master_seed: Seed::new(vec![
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]),
                derivation_path: HDPath::from_str("m/0'/0'/0'/0/0").unwrap(),
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
                derivation_purpose: HDPurpose::BIP32,
            }
        );
    }

    #[test]
    fn test_derive_specify_account_address_indices() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();
        let derived = keys
            .derive(dt.default_path_specify(Coin::Bitcoin.id(), 0, 0, 0))
            .unwrap();
        assert_eq!(
            derived,
            HDKey {
                master_seed: Seed::new(vec![
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]),
                derivation_path: HDPath::from_str("m/0'/0'/0'/0/0").unwrap(),
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
                derivation_purpose: HDPurpose::BIP32,
            }
        );
    }

    #[test]
    fn test_derive_specify_change_account_address_indices() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            keys.derive(dt.default_path_specify(Coin::Bitcoin.id(), 0, 0, 0))
                .unwrap(),
            HDKey {
                master_seed: Seed::new(vec![
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]),
                derivation_path: HDPath::from_str("m/0'/0'/0'/0/0").unwrap(),
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
                derivation_purpose: HDPurpose::BIP32,
            }
        );
    }

    #[test]
    fn test_derive_change_internal_chain_specify_account_address_indices() {
        let dt = HDPurpose::BIP32;
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();
        let derived = keys
            .derive(dt.default_path_specify(Coin::Bitcoin.id(), 0, 1, 0))
            .unwrap();
        assert_eq!(derived.master_seed, keys.master_seed);
        assert_eq!(
            derived.derivation_path.to_string(),
            "m/0'/0'/0'/1/0".to_string()
        );
        assert_eq!(&derived.extended_private_key_serialized().unwrap(), "xprvA47jwGZNLdTnKuMGfLdeYMV7dgAF9gCjYUNYGeAjJuXrRbj1MULdePjyC5nH7Pp2GTRqnXqkumeJC29fRVVSJmbrWDUENyRG22n1tJdn5b7");
        assert_eq!(&derived.extended_public_key_serialized().unwrap(), "xpub6H76Ln6GB125YPRjmNAeuVRrBhzjZ8vauhJ952aLsF4qJQ49u1etCC4T3KkvysShJwgdPL3B5fEsiZJCeymY1Z2wfUNXN77ksN9oqLP9PU3");
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
