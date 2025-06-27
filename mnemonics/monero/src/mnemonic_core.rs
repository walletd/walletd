use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mnemonic {
    phrase: String,
}

#[derive(Debug, thiserror::Error)]
pub enum MnemonicError {
    #[error("Invalid mnemonic: {0}")]
    Invalid(String),
}

impl Mnemonic {
    pub fn generate(word_count: usize) -> Result<Self, MnemonicError> {
        // Placeholder implementation
        Ok(Mnemonic {
            phrase: "placeholder mnemonic".to_string(),
        })
    }

    pub fn phrase(&self) -> &str {
        &self.phrase
    }
}