/// The MnemonicHandler trait is used to provide a common interface for the different Mnemonic libraries
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
    ) -> Result<Self::MnemonicStyle, String>;
    /// Get the ['Seed'][Seed] from the mnemonic phrase
    /// [Seed]: ./seed/struct.Seed.html
    fn to_seed(&self) -> Seed;
}

pub mod seed;

pub use self::seed::Seed;
