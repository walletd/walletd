pub mod seed;
pub use self::seed::Seed;

/// The MnemonicHandler trait is used to provide a common interface for the
/// different Mnemonic libraries
pub trait MnemonicHandler {
    /// The Mnemonic struct associated with the handler
    type MnemonicStyle;
    /// The Language struct associated with the handler
    type LanguageHandler;
    /// The MnemonicType struct associated with the handler
    type MnemonicTypeSpec;

    /// Create a new mnemonic phrase
    fn new(
        language: Self::LanguageHandler,
        mnemonic_type: Self::MnemonicTypeSpec,
        passphrase: Option<&str>,
    ) -> Self::MnemonicStyle;
    /// Create a mnemonic phrase from a given phrase
    fn from_phrase(
        language: Self::LanguageHandler,
        mnemonic_phrase: &str,
        passphrase: Option<&str>,
    ) -> Result<Self::MnemonicStyle, anyhow::Error>;
    /// Get the ['Seed'][Seed] from the mnemonic phrase
    /// [Seed]: ./seed/struct.Seed.html
    fn to_seed(&self) -> Seed;
    // Imports a mnemonic phrase, detecting the language to inform the Mnemonic
    // struct
    fn detect_language(
        mnemonic_phrase: &str,
        passphrase: Option<&str>,
    ) -> Result<Self::MnemonicStyle, anyhow::Error>;

    // Gets the language
    fn language(&self) -> Self::LanguageHandler;

    // Gets the phrase
    fn phrase(&self) -> String;

    // Gets the mnemonic type
    fn mnemonic_type(&self) -> Self::MnemonicTypeSpec;
}

/// The LanguageHandler trait is used to provide a common interface for the
/// different Language implementations in different Mnemonic libraries
pub trait LanguageHandler {
    type Language;
    fn new() -> Self::Language;
}
