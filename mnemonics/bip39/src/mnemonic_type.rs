pub const ENTROPY_OFFSET: usize = 8;

#[derive(Debug, Copy, Clone)]
pub enum MnemonicType {
    //  ... = (entropy_bits << ...)   | checksum_bits
    Words12 = (128 << ENTROPY_OFFSET) | 4,
    Words15 = (160 << ENTROPY_OFFSET) | 5,
    Words18 = (192 << ENTROPY_OFFSET) | 6,
    Words21 = (224 << ENTROPY_OFFSET) | 7,
    Words24 = (256 << ENTROPY_OFFSET) | 8, 
}

impl Default for MnemonicType {
    fn default() -> MnemonicType {
        MnemonicType::Words12
    }
}

impl MnemonicType {
    /// Creates the MnemonicType based on the word count
    pub fn for_word_count(size: usize) -> Result<MnemonicType, String> {
        let mnemonic_type = match size {
            12 => MnemonicType::Words12,
            15 => MnemonicType::Words15,
            18 => MnemonicType::Words18,
            21 => MnemonicType::Words21,
            24 => MnemonicType::Words24,
            _ => Err("invalid number of words in phrase")?,
        };

        Ok(mnemonic_type)
    }

    /// Creates the MnemonicType based on the length of the entropy in bits
    pub fn for_key_size(size: usize) -> Result<MnemonicType, String> {
        let mnemonic_type = match size {
            128 => MnemonicType::Words12,
            160 => MnemonicType::Words15,
            192 => MnemonicType::Words18,
            224 => MnemonicType::Words21,
            256 => MnemonicType::Words24,
            _ => Err("invalid number of words in phrase")?,
        };

        Ok(mnemonic_type)
    }

    /// Creates the MnemonicType based on the number of words in the mnemonic phrase
    pub fn for_phrase(phrase: &str) -> Result<MnemonicType, String> {
        let word_count = phrase.split(" ").count();

        Self::for_word_count(word_count)
    }

    /// Returns the number of entropy bits associated with the MnemonicType
    pub fn entropy_bits(&self) -> usize {
        (*self as usize) >> ENTROPY_OFFSET
    }
    
}
