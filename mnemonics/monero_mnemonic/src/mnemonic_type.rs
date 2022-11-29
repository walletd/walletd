pub const BITS_IN_BYTES: usize = 8;

#[derive(Debug, Copy, Clone)]
pub enum MnemonicType {
    // value is the length of the entropy in bits
    Words13 = 128,
    Words25 = 256,
}

impl Default for MnemonicType {
    fn default() -> MnemonicType {
        MnemonicType::Words25
    }
}

impl MnemonicType {
    pub fn for_word_count(size: usize) -> Result<MnemonicType, String> {
        let mnemonic_type = match size {
            13 => MnemonicType::Words13,
            25 => MnemonicType::Words25,
            _ => Err("invalid number of words in phrase")?,
        };

        Ok(mnemonic_type)
    }

    pub fn for_key_size(size: usize) -> Result<MnemonicType, String> {
        let mnemonic_type = match size {
            128 => MnemonicType::Words13,
            256 => MnemonicType::Words25,
            _ => Err("invalid number of words in phrase")?,
        };

        Ok(mnemonic_type)
    }

    pub fn for_phrase(phrase: &str) -> Result<MnemonicType, String> {
        let word_count = phrase.split(" ").count();

        Self::for_word_count(word_count)
    }

    pub fn entropy_bits(&self) -> usize {
        *self as usize
    }
    
}
