use std::fmt;

/// The secret value used to derive HD wallet addresses from a
/// [`Mnemonic`][Mnemonic] phrase.
///
/// To get the raw byte value use [`Seed::as_bytes()`][Seed::as_bytes()]. These
/// can be used to derive HD wallet addresses using another crate (deriving HD
/// wallet addresses is outside the scope of this crate and the BIP39 standard).
///
/// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
/// [Seed]: ./seed/struct.Seed.html
/// [Seed::as_bytes()]: ./seed/struct.Seed.html#method.as_bytes
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Seed {
    bytes: Vec<u8>,
}

impl fmt::Display for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.bytes.as_slice()))
    }
}
impl Seed {
    /// Create a new Seed from a byte slice
    pub fn new(bytes: Vec<u8>) -> Self {
        Seed { bytes }
    }

    /// Get the raw byte value of the Seed
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl fmt::LowerHex for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.bytes {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl fmt::UpperHex for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.bytes {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_as_upper_hex() {
        let seed = Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]);
        assert_eq!(format!("{seed:X}"), "A2FD9C0522D84D52EE4C8533DC02D4B69B4DF9B6255E1AF20C9F1D4D691689F2A38637EB1EC778972BF845C32D5AE83C7536999B5666397AC32021B21E0ACCEE");
        assert_eq!(format!("{seed:#X}"), "0xA2FD9C0522D84D52EE4C8533DC02D4B69B4DF9B6255E1AF20C9F1D4D691689F2A38637EB1EC778972BF845C32D5AE83C7536999B5666397AC32021B21E0ACCEE");
    }

    #[test]
    fn test_seed_as_lower_hex() {
        let seed = Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]);
        assert_eq!(format!("{seed:x}"), "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee");
        assert_eq!(format!("{seed:#x}"), "0xa2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee");
    }
}
