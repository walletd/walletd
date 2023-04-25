use crate::ParseMnemonicError;

pub const ENTROPY_OFFSET: usize = 8;

/// Determines the number of words that will make up the [`Mnemonic`][Mnemonic]
/// phrase
///
/// Also directly affects the amount of entropy that will be used to create a
/// [`Mnemonic`][Mnemonic], and therefore the cryptographic strength of the HD
/// wallet keys/addresses that can be derived from it using the [`Seed`][Seed].
///
/// For example, a 12 word mnemonic phrase is essentially a friendly
/// representation of a 128-bit key, while a 24 word mnemonic phrase is
/// essentially a 256-bit key.
///
/// If you know you want a specific phrase length, you can use the enum variant
/// directly, for example `MnemonicType::Words12`.
///
/// You can also get a `MnemonicType` that corresponds to one of the standard
/// BIP39 key sizes by passing arbitrary `usize` values:
///
/// ```
/// use walletd_bip39::MnemonicType;
///
/// let mnemonic_type = MnemonicType::from_key_size(128).unwrap();
/// ```
///
/// [MnemonicType]: ../mnemonic_type/struct.MnemonicType.html
/// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
/// [Seed]: ../seed/struct.Seed.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MnemonicType {
    //  ... = (entropy_bits << ...)   | checksum_bits
    /// 12 word mnemonic phrase
    Words12 = (128 << ENTROPY_OFFSET) | 4,
    /// 15 word mnemonic phrase
    Words15 = (160 << ENTROPY_OFFSET) | 5,
    /// 18 word mnemonic phrase
    Words18 = (192 << ENTROPY_OFFSET) | 6,
    /// 21 word mnemonic phrase
    Words21 = (224 << ENTROPY_OFFSET) | 7,
    /// 24 word mnemonic phrase
    Words24 = (256 << ENTROPY_OFFSET) | 8,
}

impl Default for MnemonicType {
    /// The default mnemonic type is 12 words
    fn default() -> MnemonicType {
        MnemonicType::Words12
    }
}

impl MnemonicType {
    /// Creates the MnemonicType based on the word count
    ///
    /// Specifying a word count not provided for by the BIP39 standard will
    /// return an `Error`
    ///
    /// # Example
    /// ```
    /// use walletd_bip39::MnemonicType;
    ///
    /// let mnemonic_type = MnemonicType::from_word_count(12).unwrap();
    /// ```
    pub fn from_word_count(size: usize) -> Result<MnemonicType, ParseMnemonicError> {
        let mnemonic_type = match size {
            12 => MnemonicType::Words12,
            15 => MnemonicType::Words15,
            18 => MnemonicType::Words18,
            21 => MnemonicType::Words21,
            24 => MnemonicType::Words24,
            _ => return Err(ParseMnemonicError::InvalidNumberOfWords(size)),
        };

        Ok(mnemonic_type)
    }

    /// Creates the MnemonicType based on the length of the key size in bits
    ///
    /// Specifying a key size not provided for by the BIP39 standard will return
    /// an `Error`
    ///
    /// # Example
    /// ```
    /// use walletd_bip39::MnemonicType;
    ///
    /// let mnemonic_type = MnemonicType::from_key_size(128).unwrap();
    /// ```
    pub fn from_key_size(size: usize) -> Result<MnemonicType, ParseMnemonicError> {
        let mnemonic_type = match size {
            128 => MnemonicType::Words12,
            160 => MnemonicType::Words15,
            192 => MnemonicType::Words18,
            224 => MnemonicType::Words21,
            256 => MnemonicType::Words24,
            _ => return Err(ParseMnemonicError::InvalidNumberOfBits(size)),
        };

        Ok(mnemonic_type)
    }

    /// Get a `MnemonicType` for an existing mnemonic phrase
    ///
    /// This can be used when you need information about a mnemonic phrase based
    /// on the number of words, for example you can get the entropy value
    /// using [`MnemonicType::entropy_bits`][MnemonicType::entropy_bits()].
    ///
    /// Specifying a phrase that does not match one of the standard BIP39 phrase
    /// lengths will return an `ParseMnemonicError`
    ///
    /// # Example
    /// ```
    /// use walletd_bip39::MnemonicType;
    ///
    /// let mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::from_phrase(mnemonic).unwrap();
    ///
    /// let entropy_bits = mnemonic_type.entropy_bits();
    /// ```
    ///
    /// [MnemonicType::entropy_bits()]: ./enum.MnemonicType.html#method.entropy_bits
    pub fn from_phrase(phrase: &str) -> Result<MnemonicType, ParseMnemonicError> {
        let word_count = phrase.split(' ').count();

        Self::from_word_count(word_count)
    }

    /// Return the number of entropy bits
    ///
    ///
    /// # Example
    /// ```
    /// use walletd_bip39::MnemonicType;
    ///
    /// let mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::from_phrase(mnemonic).unwrap();
    ///
    /// let entropy_bits = mnemonic_type.entropy_bits();
    /// ```
    pub fn entropy_bits(&self) -> usize {
        (*self as usize) >> ENTROPY_OFFSET
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let mnemonic = MnemonicType::default();
        assert_eq!(mnemonic, MnemonicType::Words12);
    }

    #[test]
    fn test_entropy_offset() {
        assert_eq!(8, ENTROPY_OFFSET);
    }

    #[test]
    fn test_from_word_count() {
        let mnemonic = MnemonicType::from_word_count(12).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words12);
        let mnemonic = MnemonicType::from_word_count(15).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words15);
        let mnemonic = MnemonicType::from_word_count(18).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words18);
        let mnemonic = MnemonicType::from_word_count(21).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words21);
        let mnemonic = MnemonicType::from_word_count(24).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words24);
    }

    #[test]
    fn test_from_phrase() {
        let mnemonic = MnemonicType::from_phrase(
            "one two three four five six seven eight nine ten eleven twelve",
        )
        .unwrap();
        assert_eq!(mnemonic, MnemonicType::Words12);
        let mnemonic = MnemonicType::from_phrase("one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen").unwrap();
        assert_eq!(mnemonic, MnemonicType::Words15);
        let mnemonic = MnemonicType::from_phrase("one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen").unwrap();
        assert_eq!(mnemonic, MnemonicType::Words18);
        let mnemonic = MnemonicType::from_phrase("one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty twentyone").unwrap();
        assert_eq!(mnemonic, MnemonicType::Words21);
        let mnemonic = MnemonicType::from_phrase("one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty twentyone twentytwo twentythree twentyfour").unwrap();
        assert_eq!(mnemonic, MnemonicType::Words24);
    }

    #[test]
    fn test_from_key_size() {
        let mnemonic = MnemonicType::from_key_size(128).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words12);
        let mnemonic = MnemonicType::from_key_size(160).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words15);
        let mnemonic = MnemonicType::from_key_size(192).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words18);
        let mnemonic = MnemonicType::from_key_size(224).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words21);
        let mnemonic = MnemonicType::from_key_size(256).unwrap();
        assert_eq!(mnemonic, MnemonicType::Words24);
    }

    #[test]
    fn test_entropy_bits() {
        let mnemonic = MnemonicType::Words12;
        assert_eq!(mnemonic.entropy_bits(), 128);
        let mnemonic = MnemonicType::Words15;
        assert_eq!(mnemonic.entropy_bits(), 160);
        let mnemonic = MnemonicType::Words18;
        assert_eq!(mnemonic.entropy_bits(), 192);
        let mnemonic = MnemonicType::Words21;
        assert_eq!(mnemonic.entropy_bits(), 224);
        let mnemonic = MnemonicType::Words24;
        assert_eq!(mnemonic.entropy_bits(), 256);
    }
}
