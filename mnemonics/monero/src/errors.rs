use thiserror::Error;

use crate::MnemonicType;

/// Custom error enum for the crate
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid word in mnemonic phrase for the given language
    #[error("Invalid word: word: {0} not found in wordlist for language: {1}")]
    InvalidWord(String, String),
    /// Could not find language match for the given phrase
    #[error("Could not find a language match for the given phrase: {0}")]
    InvalidPhraseLanguage(String),
    /// Could not find language match for the given str representing language
    /// name
    #[error("Could not match str {0} to a language")]
    InvalidStrReprLang(String),
    /// Invalid number of words in phrase
    #[error(
        "invalid number of words in phrase, found {0} words, expected {} or {}",
        MnemonicType::Words13,
        MnemonicType::Words25
    )]
    InvalidNumberOfWords(usize),
    /// Invalid number of bits in entropy for mnemonic
    #[error("invalid number of bits, found {0} bits, expected {} bits or {} bits", MnemonicType::Words13.entropy_bits(), MnemonicType::Words25.entropy_bits())]
    InvalidNumberOfBits(usize),

    /// Missing some necessary information to create a mnemonic struct
    #[error("Missing information: {0}")]
    MissingInformation(String),
    /// Mismatch in what was specified for the mnemonic struct versus what was
    /// implictly derived from other specifications
    #[error(
        "Mismatch in specification of the {} for mnemonic struct, specified: {}, implict: {}",
        attribute,
        spec,
        implict
    )]
    /// Mismatch in what was specified versus was implicitly derived from other
    /// specifications
    MismatchInSpecificationVersusImplict {
        attribute: String,
        spec: String,
        implict: String,
    },
    /// Error related to bytes, with message explaining further details
    #[error("Error in bytes: {0}")]
    ErrorInBytes(String),
    /// Error related to words, with message explaining further details
    #[error("Error in words: {0}")]
    ErrorInWords(String),
}
