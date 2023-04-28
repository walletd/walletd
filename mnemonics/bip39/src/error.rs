use thiserror::Error;

use crate::Bip39MnemonicType;

/// Custom error type for this crate
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseMnemonicError {
    /// Invalid word in mnemonic phrase
    #[error("Invalid word: word: {0} not found in wordlist")]
    InvalidWord(String),
    /// Could not find language match for the given phrase
    #[error("Could not find a language match for the given phrase: {0}")]
    InvalidPhraseLanguage(String),
    /// Could not find language match for the given str representing language
    /// name
    #[error("Could not match str {0} to a language")]
    InvalidStrReprLang(String),
    /// Missing some necessary information to create a mnemonic struct
    #[error("Missing information: {0}")]
    MissingInformation(String),
    /// Mismatch in what was specified for the mnemonic struct versus what was
    /// implictly derived from other specifications
    #[error("Mismatch in specification of the {} for mnemonic struct", attribute)]
    /// Mismatch in what was specified for the mnemonic struct versus what was
    /// implictly derived from other specifications
    MismatchInSpecificationVersusImplict {
        /// The attribute that was specified, something like language or
        /// mnemonic type
        attribute: String,
    },
    /// Invalid checksum for mnemonic phrase
    #[error("Invalid mnemonic phrase, the checksum word does not match")]
    InvalidMnemonicPhraseChecksum,
    /// Invalid entropy for mnemonic, with message explaining further details
    #[error("Invalid entropy: {0}")]
    InvalidEntropy(String),
    /// Invalid number of words in mnemonic phrase
    #[error("invalid number of words in phrase, found {0} words, expected 12, 15, 18, 21, or 24")]
    InvalidNumberOfWords(usize),
    /// Invalid number of bits in entropy for mnemonic
    #[error("invalid number of bits, found {0} bits, expected {} bits, {} bits, {} bits, {} bits, or {} bits", Bip39MnemonicType::Words12.entropy_bits(), Bip39MnemonicType::Words15.entropy_bits(), Bip39MnemonicType::Words18.entropy_bits(), Bip39MnemonicType::Words21.entropy_bits(), Bip39MnemonicType::Words24.entropy_bits())]
    InvalidNumberOfBits(usize),
}
