use std::fmt;

use anyhow::anyhow;

pub const BITS_IN_BYTES: usize = 8;

/// Determines the number of words that will make up the [`Mnemonic`][Mnemonic]
/// phrase
///
/// Also directly affects the amount of entropy that will be used to create a
/// [`Mnemonic`][Mnemonic], and therefore the cryptographic strength of the
/// wallet keys/addresses that can be derived from it using the [`Seed`][Seed].
///
/// For example, a 13 word mnemonic phrase is essentially a friendly
/// representation of a 128-bit key, while a 25 word mnemonic phrase is
/// essentially a 256-bit key.
///
/// If you know you want a specific phrase length, you can use the enum variant
/// directly, for example `MnemonicType::Words13`.
///
/// You can also get a `MnemonicType` that corresponds to one of the standard
/// MONERO key sizes by passing arbitrary `usize` values:
///
/// ```
/// use walletd_monero_mnemonic::MnemonicType;
///
/// let mnemonic_type = MnemonicType::from_key_size(128).unwrap();
/// ```
///
/// [MnemonicType]: ../mnemonic_type/struct.MnemonicType.html
/// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
/// [Seed]: ../seed/struct.Seed.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MnemonicType {
    // value is the length of the entropy in bits
    Words13 = 128,
    Words25 = 256,
}

impl fmt::Display for MnemonicType {
    /// Display the mnemonic type as a string
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MnemonicType::Words13 => fmt.write_str("13 words")?,
            MnemonicType::Words25 => fmt.write_str("25 words")?,
        };
        Ok(())
    }
}

impl Default for MnemonicType {
    /// The default mnemonic type is 25 words
    fn default() -> MnemonicType {
        MnemonicType::Words25
    }
}

impl MnemonicType {
    /// Creates the MnemonicType based on the word count
    ///
    /// Specifying a word count not provided for by the MONERO standard will
    /// return an `Error`
    ///
    /// # Example
    /// ```
    /// use walletd_monero_mnemonic::MnemonicType;
    ///
    /// let mnemonic_type = MnemonicType::from_word_count(13).unwrap();
    /// ```
    pub fn from_word_count(size: usize) -> Result<MnemonicType, anyhow::Error> {
        let mnemonic_type = match size {
            13 => MnemonicType::Words13,
            25 => MnemonicType::Words25,
            _ => Err(anyhow!("invalid number of words in phrase"))?,
        };

        Ok(mnemonic_type)
    }

    /// Creates the MnemonicType based on the length of the key size in bits
    ///
    /// Specifying a key size not provided for by the MONERO standard will
    /// return an `Error`
    ///
    /// # Example
    /// ```
    /// use walletd_monero_mnemonic::MnemonicType;
    ///
    /// let mnemonic_type = MnemonicType::from_key_size(128).unwrap();
    /// ```
    pub fn from_key_size(size: usize) -> Result<MnemonicType, anyhow::Error> {
        let mnemonic_type = match size {
            128 => MnemonicType::Words13,
            256 => MnemonicType::Words25,
            _ => Err(anyhow!("invalid number of words in phrase"))?,
        };

        Ok(mnemonic_type)
    }

    /// Get a `MnemonicType` for an existing mnemonic phrase
    ///
    /// This can be used when you need information about a mnemonic phrase based
    /// on the number of words, for example you can get the entropy value
    /// using [`MnemonicType::entropy_bits`][MnemonicType::entropy_bits()].
    ///
    /// Specifying a phrase that does not match one of the standard MONERO
    /// phrase lengths will return an `Error`
    ///
    /// # Example
    /// ```
    /// use walletd_monero_mnemonic::MnemonicType;
    ///
    /// let mnemonic = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
    ///
    /// let mnemonic_type = MnemonicType::from_phrase(mnemonic).unwrap();
    ///
    /// let entropy_bits = mnemonic_type.entropy_bits();
    /// ```
    ///
    /// [MnemonicType::entropy_bits()]: ./enum.MnemonicType.html#method.entropy_bits
    pub fn from_phrase(phrase: &str) -> Result<MnemonicType, anyhow::Error> {
        let word_count = phrase.split(' ').count();

        Self::from_word_count(word_count)
    }

    /// Return the number of entropy bits
    ///
    ///
    /// # Example
    /// ```
    /// use walletd_monero_mnemonic::MnemonicType;
    ///
    /// let mnemonic = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
    ///
    /// let mnemonic_type = MnemonicType::from_phrase(mnemonic).unwrap();
    ///
    /// let entropy_bits = mnemonic_type.entropy_bits();
    /// ```
    pub fn entropy_bits(&self) -> usize {
        *self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let mnemonic = MnemonicType::default();
        assert_eq!(mnemonic, MnemonicType::Words25);
    }

    #[test]
    fn test_entropy_offset() {
        assert_eq!(8, BITS_IN_BYTES);
    }

    #[test]
    fn test_print() {
        assert_eq!(format!("{}", MnemonicType::Words13), "13 words");
        assert_eq!(format!("{}", MnemonicType::Words25), "25 words");
    }

    #[test]
    fn test_from_word_count() {
        let mnemonic = MnemonicType::from_word_count(13).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words13);
        let mnemonic = MnemonicType::from_word_count(25).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words25);
    }

    #[test]
    fn test_from_phrase() {
        let mnemonic = MnemonicType::from_phrase(
            "one two three four five six seven eight nine ten eleven twelve thirteen",
        )
        .unwrap();
        assert_eq!(mnemonic, MnemonicType::Words13);
        let mnemonic = MnemonicType::from_phrase("one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty twentyone twentytwo twentythree twentyfour twentyfive").unwrap();
        assert_eq!(mnemonic, MnemonicType::Words25);
    }

    #[test]
    fn test_from_key_size() {
        let mnemonic = MnemonicType::from_key_size(128).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words13);
        let mnemonic = MnemonicType::from_key_size(256).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words25);
    }

    #[test]
    fn test_entropy_bits() {
        let mnemonic = MnemonicType::Words13;
        assert_eq!(mnemonic.entropy_bits(), 128);
        let mnemonic = MnemonicType::Words25;
        assert_eq!(mnemonic.entropy_bits(), 256);
    }
}
