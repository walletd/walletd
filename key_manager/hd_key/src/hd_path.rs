use crate::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HDPurpose {
    BIP32,
    BIP44,
    BIP49,
    BIP84,
}

impl Default for HDPurpose {
    fn default() -> Self {
        HDPurpose::BIP32
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
            HDPathIndex::IndexNotHardened(num) => write!(f, "{}", num),
            HDPathIndex::IndexHardened(num) => write!(f, "{}'", num),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HDPath(Vec<HDPathIndex>);

impl HDPath {
    pub fn from_str(s: &str) -> Result<Self, Error> {
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
            if part.ends_with('\'') {
                let num: u32 = part.trim_end_matches('\'').parse().map_err(|_| {
                    Error::Invalid(format!("Invalid hardened index: {}", part))
                })?;
                indices.push(HDPathIndex::IndexHardened(num));
            } else {
                let num: u32 = part.parse().map_err(|_| {
                    Error::Invalid(format!("Invalid index: {}", part))
                })?;
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
        self.0.get(index).copied().ok_or_else(|| {
            Error::IndexOutOfRange {
                index: index as u32,
                max: self.0.len() as u32,
            }
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
            write!(f, "{}", index)?;
        }
        Ok(())
    }
}

impl From<Vec<HDPathIndex>> for HDPath {
    fn from(indices: Vec<HDPathIndex>) -> Self {
        HDPath(indices)
    }
}