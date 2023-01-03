use std::fmt;

/// The secret value used to derive HD wallet addresses from a [`Mnemonic`][Mnemonic] phrase.
///
/// To get the raw byte value use [`Seed::as_bytes()`][Seed::as_bytes()]. These can be used to derive
/// HD wallet addresses using another crate (deriving HD wallet addresses is outside the scope of this
/// crate and the BIP39 standard).
///
/// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
/// [Seed]: ./seed/struct.Seed.html
/// [Seed::as_bytes()]: ./seed/struct.Seed.html#method.as_bytes
#[derive(Debug, Clone, PartialEq)]
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
