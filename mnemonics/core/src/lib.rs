//! This library provides a common interface for handling mnemonics
pub use hex;

pub mod seed;
pub use self::seed::Seed;
pub mod prelude;

#[doc(hidden)]
pub use std::str::FromStr;

/// Provide a common interface for
/// different mnemonic protocols.
pub trait Mnemonic {
    /// The associated Language struct
    type Language;
    /// The associated MnemonicType struct
    type MnemonicType;
    /// The associated Builder struct for the Mnemonic
    type MnemonicBuilder;
    type Mnemonic;
    type ErrorType;

    /// Generates a new [mnemonic][Self::Mnemonic] given the [language][Self::Language], [mnemonic type][Self::MnemonicType], and
    /// an optional passphrase.
    fn new(
        language: Self::Language,
        mnemonic_type: Self::MnemonicType,
        passphrase: Option<&str>,
    ) -> Self::Mnemonic;

    /// Recovers a [mnemonic][Self::Mnemonic] given the given the [language][Self::Language], [mnemonic type][Self::MnemonicType], and
    /// an optional passphrase.
    fn from_phrase(
        language: Self::Language,
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self::Mnemonic, Self::ErrorType>;

    /// Recovers a [mnemonic][Self::Mnemonic] given the mnemonic phrase and optional passphrase,
    /// attempts to auto detect the language of the mnemonic phrase.
    /// Returns an [error][Self::ErrorType] if the language cannot be detected or the provided mnemonic phrase was invalid.
    fn detect_language(
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self::Mnemonic, Self::ErrorType>;

    /// Returns the [builder][Self::MnemonicBuilder] for the [mnemonic][Self::Mnemonic].
    fn builder() -> Self::MnemonicBuilder;

    /// Returns the [seed][Seed] associated with the mnemonic phrase.
    fn to_seed(&self) -> Seed;

    // Returns the [language][Self::Language] for the mnemonic.
    fn language(&self) -> Self::Language;

    // Returns the mnemonic phrase.
    fn phrase(&self) -> String;

    // Returns the [mnemonic type][Self::MnemonicType].
    fn mnemonic_type(&self) -> Self::MnemonicType;
}

/// Provides a builder pattern for creating a [Mnemonic].
pub trait MnemonicBuilder {
    /// The associated Mnemonic struct
    type Mnemonic;
    /// The associated Language struct
    type Language;
    /// The associated MnemonicType struct
    type MnemonicType;

    /// The type of error that can be returned by the builder
    type ErrorType;

    /// Creates a new builder struct with default values.
    ///
    /// The default values depend on the implementation of the builder.
    fn new() -> Self;

    /// Specifies the seed from which the mnemonic struct is recovered.
    ///
    /// If a passphrase is specified with the [passphrase][MnemonicBuilder::passphrase] method, the seed recovered using
    /// the [restore][MnemonicBuilder::restore] or [build][MnemonicBuilder::build] method will be the encrypted version of the seed which takes
    /// into account the passphrase value.
    fn mnemonic_seed(&mut self, seed: &Seed) -> &mut Self;

    /// Specifies the mnemonic phrase from which the mnemonic struct is
    /// recovered.
    fn mnemonic_phrase(&mut self, mnemonic_phrase: &str) -> &mut Self;

    /// Specifies the language for the mnemonic phrase, can be used when
    /// recovering a mnemonic phrase or generating a new mnemonic phrase.
    fn language(&mut self, language: Self::Language) -> &mut Self;

    /// Specifies a passphrase to use to offset/encrypt the seed recovered from
    /// the mnemonic phrase.
    fn passphrase(&mut self, passphrase: &str) -> &mut Self;

    /// Specifies the [mnemonic type][Self::mnemonic_type] to use when recovering or generating a
    /// mnemonic phrase.
    fn mnemonic_type(&mut self, mnemonic_type: Self::MnemonicType) -> &mut Self;

    /// Sets the [specified language][Self::Language] to None.
    ///
    /// When used with [mnemonic_phrase][MnemonicBuilder::mnemonic_phrase] and [build][MnemonicBuilder::build] or [restore][MnemonicBuilder::restore], automatically detects the language of the mnemonic phrase and returns an error if the mnemonic phrase is invalid for every [mnemonic][Self::Mnemonic] [language][Self::Language] wordlist.
    fn detect_language(&mut self) -> &mut Self;

    /// Builds a mnemonic struct given the specifications provided to the
    /// builder.
    ///
    /// [build()][MnemonicBuilder::build()] can be used in place of [generate()][MnemonicBuilder::generate()] or [restore()][MnemonicBuilder::restore()] and will emulate the appropriate behavior based on the specifications provided to the builder.   
    fn build(&self) -> Result<Self::Mnemonic, Self::ErrorType>;

    /// Restore a previously used mnemonic based on the given specifications.
    ///
    /// Will return an [error][Self::ErrorType] in cases that not enough information was provided to restore the mnemonic or if any provided specifications conflict with each other.
    fn restore(&self) -> Result<Self::Mnemonic, Self::ErrorType>;

    /// Generate a new mnemonic which follows the provided specifications.
    fn generate(&self) -> Result<Self::Mnemonic, Self::ErrorType>;
}

/// The Language trait is used to provide a common interface for the
/// different Language implementations in different walletD mnemonic libraries.
pub trait Language {
    type Language;

    /// Returns the default [Language] for the implementation.
    fn new() -> Self::Language;
}
