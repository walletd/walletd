pub trait MnemonicHandler {
    type MnemonicStyle;
    type LanguageHandler;
    type MnemonicTypeSpec;
    type WordListHandler;

    fn new(language: Self::LanguageHandler, mnemonic_type: Self::MnemonicTypeSpec, passphrase: Option<&str>) -> Self::MnemonicStyle;
    fn from_phrase(language: Self::LanguageHandler, mnemonic_phrase: &str, passphrase: Option<&str>) -> Result<Self::MnemonicStyle, String>;
    fn bytes_to_words(entropy_bytes: &Vec<u8>, wordlist_info: &Self::WordListHandler) -> Result<String, String>;
    fn words_to_bytes(language: Self::LanguageHandler, mnemonic_phrase: &String) -> Result<Vec<u8>, String>;
    fn seed_hex(&self) -> Result<String, String>; 
    fn seed_bytes(&self) -> Result<&[u8], String>; 
}
