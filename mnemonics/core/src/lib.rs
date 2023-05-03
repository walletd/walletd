//! This library provides a common interface for handling mnemonics
pub use hex;

pub mod seed;
pub use self::seed::Seed;
pub mod prelude;

/// The Mnemonic trait is used to provide a common interface for the
/// different walletD mnemonic libraries
pub trait Mnemonic {
    /// The associated Language struct
    type Language;
    /// The associated MnemonicType struct
    type MnemonicType;
    /// The associated Builder struct for the Mnemonic
    type MnemonicBuilder;
    type Mnemonic;
    type ErrorType;

    /// Generates a new mnemonic given the language, length of mnemonic, and
    /// optional passphrase
    fn new(
        language: Self::Language,
        mnemonic_type: Self::MnemonicType,
        passphrase: Option<&str>,
    ) -> Self::Mnemonic;

    /// Recovers a mnemonic given the language, mnemonic phrase, and optional
    /// passphrase
    fn from_phrase(
        language: Self::Language,
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self::Mnemonic, Self::ErrorType>;

    /// Recovers a mnemonic given the mnemonic phrase and optional passphrase,
    /// attempts to auto detect the language of the mnemonic phrase
    fn detect_language(
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self::Mnemonic, Self::ErrorType>;

    /// This method returns the builder for the mnemonic phrase
    fn builder() -> Self::MnemonicBuilder;

    /// Returns the ['Seed'][Seed] associated with the mnemonic phrase
    /// [Seed]: ./seed/struct.Seed.html
    fn to_seed(&self) -> Seed;

    // Returns the language for the mnemonic
    fn language(&self) -> Self::Language;

    // Returns the mnemonic phrase
    fn phrase(&self) -> String;

    // Returns the mnemonic type
    fn mnemonic_type(&self) -> Self::MnemonicType;
}

/// This trait implements a builder pattern for creating a mnemonic
pub trait MnemonicBuilder {
    /// The associated Mnemonic struct
    type Mnemonic;
    /// The associated Language struct
    type Language;
    /// The associated MnemonicType struct
    type MnemonicType;

    /// The type of error that can be returned by the builder
    type ErrorType;

    /// Creates new builder struct with default values
    fn new() -> Self;

    /// Specifies the seed from which the mnemonic struct is recovered, it's
    /// assumed that the seed provided directly corresponds to the mnemonic
    /// phrase and is not an encrypted version of the seed. If a passphrase
    /// is specified with the set_passphrase method, the seed recovered using
    /// the recover method will be the encrypted version of the seed which takes
    /// into account the passphrase value.
    fn mnemonic_seed(&mut self, seed: &Seed) -> &mut Self;

    /// Specifies the mnemonic phrase from which the mnemonic struct is
    /// recovered
    fn mnemonic_phrase(&mut self, mnemonic_phrase: &str) -> &mut Self;

    /// Specifies the language for the mnemonic phrase, can be used when
    /// recovering a mnemonic phrase or generating a new mnemonic phrase
    fn language(&mut self, language: Self::Language) -> &mut Self;

    /// Specifies a passphrase to use to offset/encrypt the seed recovered from
    /// the mnemonic phrase
    fn passphrase(&mut self, passphrase: &str) -> &mut Self;

    /// Specifies the mnemonic type to use when recovering or generating a
    /// mnemonic phrase
    fn mnemonic_type(&mut self, mnemonic_type: Self::MnemonicType) -> &mut Self;

    /// Sets the specified language to None and returns the builder
    /// This method can be used to let the mnemomic be created from the phrase
    /// and automatically detect the language of the mnemonic phrase
    fn detect_language(&mut self) -> &mut Self;

    /// Builds a mnemonic struct given the specifications provided to the
    /// builder
    fn build(&self) -> Result<Self::Mnemonic, Self::ErrorType>;

    /// Restore a previously used mnemonic given specifications
    fn restore(&self) -> Result<Self::Mnemonic, Self::ErrorType>;

    /// Generate a new mnemonic which follows given specifications
    fn generate(&self) -> Result<Self::Mnemonic, Self::ErrorType>;
}

/// The Language trait is used to provide a common interface for the
/// different Language implementations in different walletD mnemonic libraries
pub trait Language {
    type Language;
    fn new() -> Self::Language;
}
