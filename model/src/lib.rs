pub mod seed;
pub use self::seed::Seed;

mod error;
pub use error::Error;

/// The MnemonicHandler trait is used to provide a common interface for the
/// different Mnemonic libraries
pub trait MnemonicHandler {
    /// The associated Language struct
    type LanguageHandler;
    /// The associated MnemonicType struct
    type MnemonicTypeSpec;
    /// The associated Builder struct for the Mnemonic
    type MnemonicStyleBuilder;
    type MnemonicStyle;
    type ErrorType;

    /// Generates a new mnemonic given the language, length of mnemonic, and
    /// optional passphrase
    fn new(
        language: Self::LanguageHandler,
        mnemonic_type: Self::MnemonicTypeSpec,
        passphrase: Option<&str>,
    ) -> Self::MnemonicStyle;

    /// Recovers a mnemonic given the language, mnemonic phrase, and optional
    /// passphrase
    fn from_phrase(
        language: Self::LanguageHandler,
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self::MnemonicStyle, Self::ErrorType>;

    /// Recovers a mnemonic given the mnemonic phrase and optional passphrase,
    /// attempts to auto detect the language of the mnemonic phrase
    fn detect_language(
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self::MnemonicStyle, Self::ErrorType>;

    /// This method returns the builder for the mnemonic phrase
    fn builder() -> Self::MnemonicStyleBuilder;

    /// Returns the ['Seed'][Seed] associated with the mnemonic phrase
    /// [Seed]: ./seed/struct.Seed.html
    fn to_seed(&self) -> Seed;

    // Returns the language for the mnemonic
    fn language(&self) -> Self::LanguageHandler;

    // Returns the mnemonic phrase
    fn phrase(&self) -> String;

    // Returns the mnemonic type
    fn mnemonic_type(&self) -> Self::MnemonicTypeSpec;
}

/// This trait implements a builder pattern for creating a mnemonic
pub trait MnemonicStyleBuilder {
    /// The associated Mnemonic struct
    type MnemonicStyle;
    /// The associated Language struct
    type LanguageHandler;
    /// The associated MnemonicType struct
    type MnemonicTypeSpec;

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
    fn set_seed(&mut self, seed: &Seed) -> Self;

    /// Specifies the mnemonic phrase from which the mnemonic struct is
    /// recovered
    fn set_phrase(&mut self, mnemonic_phrase: &str) -> Self;

    /// Specifies the language for the mnemonic phrase, can be used when
    /// recovering a mnemonic phrase or generating a new mnemonic phrase
    fn set_language(&mut self, language: Self::LanguageHandler) -> Self;

    /// Specifies a passphrase to use to offset/encrypt the seed recovered from
    /// the mnemonic phrase
    fn set_passphrase(&mut self, passphrase: &str) -> Self;

    /// Specifies the mnemonic type to use when recovering or generating a
    /// mnemonic phrase
    fn set_mnemonic_type(&mut self, mnemonic_type: Self::MnemonicTypeSpec) -> Self;

    /// Sets the specified language to None and returns the builder
    /// This method can be used to let the mnemomic be created from the phrase
    /// and automatically detect the language of the mnemonic phrase
    fn detect_language(&mut self) -> Self;

    /// Restore a previously used mnemonic given specifications
    fn restore(&self) -> Result<Self::MnemonicStyle, Self::ErrorType>;

    /// Generate a new mnemonic which follows given specifications
    fn generate(&self) -> Result<Self::MnemonicStyle, Self::ErrorType>;
}

/// The LanguageHandler trait is used to provide a common interface for the
/// different Language implementations in different Mnemonic libraries
pub trait LanguageHandler {
    type Language;
    fn new() -> Self::Language;
}
