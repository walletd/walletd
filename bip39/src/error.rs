use thiserror::Error;

use crate::MnemonicType;

/// Custom error type for this crate
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseMnemonicError {
    /// Invalid word in mnemonic phrase
    #[error("Invalid word: word: {0} not found in wordlist for language: {1}")]
    InvalidWord(String, String),
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
    #[error(
        "Mismatch in specification of the {} for mnemonic struct, specified: {}, implict: {}",
        attribute,
        spec,
        implict
    )]
    /// Mismatch in what was specified for the mnemonic struct versus what was
    /// implictly derived from other specifications
    MismatchInSpecificationVersusImplict {
        /// The attribute that was specified, something like language or
        /// mnemonic type
        attribute: String,
        /// The specification that was given
        spec: String,
        /// The implict specification that was derived from other
        /// specifications, and error is due to conflit between the implicit and
        /// explicit specification
        implict: String,
    },
    /// Invalid checksum for mnemonic phrase
    #[error("Invalid mnemonic phrase, the checksum word does not match")]
    InvalidMnemonicPhraseChecksum,
    /// Invalid entropy for mnemonic, with message explaining further details
    #[error("Invalid entropy: {0}")]
    InvalidEntropy(String),
    /// Invalid number of words in mnemonic phrase
    #[error(
        "invalid number of words in phrase, found {0} words, expected {}, {}, {}, {}, or {}",
        MnemonicType::Words12,
        MnemonicType::Words15,
        MnemonicType::Words18,
        MnemonicType::Words21,
        MnemonicType::Words24
    )]
    InvalidNumberOfWords(usize),
    /// Invalid number of bits in entropy for mnemonic
    #[error("invalid number of bits, found {0} bits, expected {} bits, {} bits, {} bits, {} bits, or {} bits", MnemonicType::Words12.entropy_bits(), MnemonicType::Words15.entropy_bits(), MnemonicType::Words18.entropy_bits(), MnemonicType::Words21.entropy_bits(), MnemonicType::Words24.entropy_bits())]
    InvalidNumberOfBits(usize),
}
