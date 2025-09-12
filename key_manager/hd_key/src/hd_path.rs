use crate::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HDPurpose {
    #[default]
    BIP32,
    BIP44,
    BIP49,
    BIP84,
}

impl HDPurpose {
    pub fn to_shortform_num(&self) -> u32 {
        match self {
            HDPurpose::BIP32 => 0,
            HDPurpose::BIP44 => 44,
            HDPurpose::BIP49 => 49,
            HDPurpose::BIP84 => 84,
        }
    }

    pub fn default_path_specify(
        &self,
        coin_type: u32,
        account: u32,
        change: u32,
        address: u32,
    ) -> String {
        match self {
            HDPurpose::BIP32 => format!("m/{coin_type}'/{account}'/0'/{change}/{address}"),
            _ => format!(
                "m/{}'/{}'/{}'/{}'/{}'",
                self.to_shortform_num(),
                coin_type,
                account,
                change,
                address
            ),
        }
    }
}

impl fmt::Display for HDPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HDPurpose::BIP32 => write!(f, "0'"),
            HDPurpose::BIP44 => write!(f, "44'"),
            HDPurpose::BIP49 => write!(f, "49'"),
            HDPurpose::BIP84 => write!(f, "84'"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HDPathIndex {
    Master,
    IndexNotHardened(u32),
    IndexHardened(u32),
}

impl HDPathIndex {
    pub fn hardened_full_num(num: u32) -> u32 {
        num | 0x80000000
    }
}

impl fmt::Display for HDPathIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HDPathIndex::Master => write!(f, "m"),
            HDPathIndex::IndexNotHardened(num) => write!(f, "{num}"),
            HDPathIndex::IndexHardened(num) => write!(f, "{num}'"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HDPath(Vec<HDPathIndex>);

pub struct HDPathBuilder {
    indices: Vec<HDPathIndex>,
}

impl HDPathBuilder {
    pub fn new() -> Self {
        Self {
            indices: vec![HDPathIndex::Master],
        }
    }

    pub fn purpose_index(mut self, purpose: u32) -> Self {
        self.indices.push(HDPathIndex::IndexHardened(purpose));
        self
    }

    pub fn coin_type_index(mut self, coin_type: u32) -> Self {
        self.indices.push(HDPathIndex::IndexHardened(coin_type));
        self
    }

    pub fn account_index(mut self, account: u32) -> Self {
        self.indices.push(HDPathIndex::IndexHardened(account));
        self
    }

    pub fn change_index(mut self, change: u32) -> Self {
        self.indices.push(HDPathIndex::IndexNotHardened(change));
        self
    }

    pub fn address_index(mut self, address: u32) -> Self {
        self.indices.push(HDPathIndex::IndexNotHardened(address));
        self
    }

    pub fn hardened_address(mut self) -> Self {
        if let Some(last_index) = self.indices.last_mut() {
            if let HDPathIndex::IndexNotHardened(num) = last_index {
                *last_index = HDPathIndex::IndexHardened(*num);
            }
        }
        self
    }

    pub fn no_change_index(mut self) -> Self {
        if self.indices.len() == 3 {
            self.indices.push(HDPathIndex::IndexHardened(0));
        }
        self
    }

    pub fn no_address_index(mut self) -> Self {
        if self.indices.len() == 3 {
            self.indices.push(HDPathIndex::IndexHardened(0));
        }
        self
    }
    pub fn build(self) -> HDPath {
        {
            let mut indices = self.indices;
            if indices.len() == 3 {
                if let HDPathIndex::IndexHardened(44) = indices[1] {
                    indices.push(HDPathIndex::IndexHardened(0));
                    indices.push(HDPathIndex::IndexNotHardened(0));
                    indices.push(HDPathIndex::IndexNotHardened(0));
                }
            }
            HDPath(indices)
        }
    }
}

impl Default for HDPathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HDPath {
    pub fn builder() -> HDPathBuilder {
        HDPathBuilder::new()
    }

    #[allow(clippy::should_implement_trait)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, Error> {
        Self::parse_path(s)
    }

    pub fn parse_path(s: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = s.split('/').collect();
        let mut indices = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            if i == 0 && *part != "m" {
                return Err(Error::Invalid("Path must start with 'm'".to_string()));
            }
            if i == 0 {
                indices.push(HDPathIndex::Master);
                continue;
            }
            if part.ends_with('\'') || part.ends_with('h') {
                let trimmed = if part.ends_with('\'') {
                    part.trim_end_matches('\'')
                } else {
                    part.trim_end_matches('h')
                };
                let num: u32 = trimmed
                    .parse()
                    .map_err(|_| Error::Invalid(format!("Invalid hardened index: {part}")))?;
                indices.push(HDPathIndex::IndexHardened(num));
            } else {
                let num: u32 = part
                    .parse()
                    .map_err(|_| Error::Invalid(format!("Invalid index: {part}")))?;
                indices.push(HDPathIndex::IndexNotHardened(num));
            }
        }
        Ok(HDPath(indices))
    }

    pub fn to_vec(&self) -> Vec<HDPathIndex> {
        self.0.clone()
    }

    pub fn push(&mut self, index: HDPathIndex) {
        self.0.push(index);
    }

    pub fn at(&self, index: usize) -> Result<HDPathIndex, Error> {
        self.0.get(index).copied().ok_or(Error::IndexOutOfRange {
            index: index as u32,
            max: self.0.len() as u32,
        })
    }

    pub fn purpose(&self) -> Result<HDPurpose, Error> {
        if self.0.len() < 2 {
            return Err(Error::Invalid("Path too short for purpose".to_string()));
        }
        match self.0.get(1) {
            Some(HDPathIndex::IndexHardened(44)) => Ok(HDPurpose::BIP44),
            Some(HDPathIndex::IndexHardened(49)) => Ok(HDPurpose::BIP49),
            Some(HDPathIndex::IndexHardened(84)) => Ok(HDPurpose::BIP84),
            _ => Ok(HDPurpose::BIP32),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for HDPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return write!(f, "");
        }
        for (i, index) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "/")?;
            }
            write!(f, "{index}")?;
        }
        Ok(())
    }
}

impl From<Vec<HDPathIndex>> for HDPath {
    fn from(indices: Vec<HDPathIndex>) -> Self {
        HDPath(indices)
    }
}
