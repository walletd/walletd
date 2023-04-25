use std::fmt::{self, Display};
use std::str::FromStr;

use crate::Error;

/// This enum represents the different derivation path schemes
/// supported by the library.
///
/// BIP32 is the default derivation scheme which uses a purpose value of 0'
/// BIP44 uses 44': <https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki>
/// BIP49 uses 49': <https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki>
/// BIP84 uses 84': <https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki>
/// The [`HDPathBuilder`] struct can be used to set a default purpose value to
/// use with particular cryptocurrency implementation.
#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum HDPurpose {
    #[default]
    /// BIP32 is the default derivation scheme which uses a purpose value of 0'
    BIP32,
    /// BIP44 uses a purpose value of 44'
    BIP44,
    /// BIP49 uses a purpose value of 49'
    BIP49,
    /// BIP84 uses a purpose value of 84'
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
    /// This function uses a hardened index for the purpose, coin id and account
    /// and a non-hardened index for the change and address index.
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

/// This enum distinguishes between the different derivation path
/// components.
///
/// The [`HDPath`] struct contains a vector of these values.
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
    /// shortform number used in the derivation path string accompanied by '
    /// to indicate hardened
    pub fn hardened_full_num(num: u32) -> u32 {
        num + (1 << 31)
    }

    /// Convert from the full number used represent a hardened index to the
    /// shortform number which when accompanied by ' indicates a hardened
    /// index
    pub fn hardened_shortform_num(full_num: u32) -> u32 {
        full_num - (1 << 31)
    }

    /// Returns the short form value of index.
    /// For master type always returns 0,
    /// for hardened index returns the short form value without the hardened
    /// indicator, the value here for hardened index is not the same as the full
    /// index number which is used in the calculation but rather the short form
    /// value used in the derivation string when accompanied by the ' indicator
    /// For non-hardened index returns the the shortform number and full number
    /// are the same.
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

/// This struct contains vector of [`HDPathIndex`] to represent a derivation
/// path.
#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HDPath {
    path: Vec<HDPathIndex>,
}

impl HDPath {
    /// Creates a new [`HDPath`] from a string representation of the path.
    pub fn new(path: &str) -> Result<Self, Error> {
        HDPath::from_str(path)
    }

    /// Pushes a new [`HDPathIndex`] to the path
    pub fn push(&mut self, index: HDPathIndex) {
        self.path.push(index);
    }

    /// Returns the length of the path
    pub fn len(&self) -> usize {
        self.path.len()
    }

    /// Returns true if the path is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    /// Helper function to convert a derivation path string to a list of strings
    /// Returns [`Error`] if the path is empty or does not start with "m"
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

    /// Returns the underlying vector of [`HDPathIndex`]
    pub fn to_vec(&self) -> Vec<HDPathIndex> {
        self.path.clone()
    }

    /// Helper function to convert a derivation path string to a vector of
    /// [`HDPathIndex`]
    /// Returns an [`Error`] variant if the derivation path string is invalid
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

/// [`HDPathBuilder`] is a builder for the [`HDPath`], it allows specification
/// of the standard full path and also which component are hardened. The default
/// implementation uses the standard format for the full path.
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
    pub fn purpose(&mut self, purpose: u32) -> &mut Self {
        self.purpose = Some(purpose);
        self
    }

    /// Specify that the purpose index should be hardened
    pub fn hardened_purpose(&mut self) -> &mut Self {
        self.purpose_hardened = true;
        self
    }

    /// Specify that the purpose index should not be hardened
    pub fn non_hardened_purpose(&mut self) -> &mut Self {
        self.purpose_hardened = false;
        self
    }

    /// Specify the coin_type index shortform number value
    pub fn coin_type_index(&mut self, coin_type: u32) -> &mut Self {
        self.coin_type = Some(coin_type);
        self
    }

    /// Specify that the coin type index should be hardened
    pub fn hardened_coin_type(&mut self) -> &mut Self {
        self.coin_type_hardened = true;
        self
    }

    /// Specify that the coin type index should not be hardened
    pub fn non_hardened_coin_type(&mut self) -> &mut Self {
        self.coin_type_hardened = false;
        self
    }

    /// Specify the account index shortform number value
    pub fn account_index(&mut self, account: u32) -> &mut Self {
        self.account = Some(account);
        self
    }

    /// Specify that the account index should be hardened
    pub fn hardened_account(&mut self) -> &mut Self {
        self.account_hardened = true;
        self
    }

    /// Specify that the account index should not be hardened
    pub fn non_hardened_account(&mut self) -> &mut Self {
        self.account_hardened = false;
        self
    }

    /// Specify the change index shortform number value
    pub fn change_index(&mut self, change: u32) -> &mut Self {
        self.change = Some(change);
        self
    }

    /// Specify that the change index should be hardened
    pub fn hardened_change(&mut self)-> &mut Self {
        self.change_hardened = true;
        self
    }

    /// Specify that the change index should not be hardened
    pub fn non_hardened_change(&mut self) -> &mut Self {
        self.change_hardened = false;
        self
    }

    /// Specify the address_index index shortform number value
    pub fn address_index(&mut self, address_index: u32) -> &mut Self {
        self.address_index = Some(address_index);
        self
    }

    /// Specify that the address index should be hardened
    pub fn hardened_address(&mut self) -> &mut Self {
        self.address_index_hardened = true;
        self
    }

    /// Specify that the address index should not be hardened
    pub fn non_hardened_address(&mut self) -> &mut Self {
        self.address_index_hardened = false;
        self
    }

    /// Set the purpose index to None
    pub fn no_purpose_index(&mut self) -> &mut Self {
        self.purpose = None;
        self
    }

    /// Set the coin_type index to None
    pub fn no_coin_type_index(&mut self) -> &mut Self {
        self.coin_type = None;
        self
    }

    /// Set the account index to None
    pub fn no_account_index(&mut self) -> &mut Self {
        self.account = None;
        self
    }

    /// Set the change index to None
    pub fn no_change_index(&mut self) -> &mut Self {
        self.change = None;
        self
    }

    /// Set the address_index index to None
    pub fn no_address_index(&mut self) -> &mut Self {
        self.address_index = None;
        self
    }

    /// Build the [HDPath]
    /// The [HDPath] will be built from the values specified in the builder
    /// The [HDPath] always starts with the Master index (m)
    /// The [HDPath] will go in order from purpose, coin_type, account, change,
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

    use super::*;

    #[test]
    fn test_derive_type() {
        let dt = HDPurpose::BIP32;
        assert_eq!(format!("{}", dt), "0'");
    }
}
