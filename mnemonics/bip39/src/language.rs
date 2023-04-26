static CHINESE_SIMPLIFIED: &str = include_str!("langs/chinese_simplified.txt");
static CHINESE_TRADITIONAL: &str = include_str!("langs/chinese_traditional.txt");
static CZECH: &str = include_str!("langs/czech.txt");
static ENGLISH: &str = include_str!("langs/english.txt");
static FRENCH: &str = include_str!("langs/french.txt");
static ITALIAN: &str = include_str!("langs/italian.txt");
static JAPANESE: &str = include_str!("langs/japanese.txt");
static KOREAN: &str = include_str!("langs/korean.txt");
static SPANISH: &str = include_str!("langs/spanish.txt");
static PORTUGUESE: &str = include_str!("langs/portuguese.txt");

use std::str::FromStr;

use walletd_mnemonics_core::LanguageExt;

use crate::ParseMnemonicError;

#[derive(Debug)]
/// Represents a wordlist for a language for the Bip39 Mnemonic
pub struct WordList {
    language: Language,
    inner: Vec<&'static str>,
}

impl WordList {
    /// Creates a new [WordList] for a specifed language
    pub fn new(language: Language) -> WordList {
        match language {
            Language::English => WordList {
                language,
                inner: ENGLISH.split_whitespace().collect(),
            },
            Language::ChineseSimplified => WordList {
                language,
                inner: CHINESE_SIMPLIFIED.split_whitespace().collect(),
            },
            Language::ChineseTraditional => WordList {
                language,
                inner: CHINESE_TRADITIONAL.split_whitespace().collect(),
            },
            Language::Czech => WordList {
                language,
                inner: CZECH.split_whitespace().collect(),
            },
            Language::French => WordList {
                language,
                inner: FRENCH.split_whitespace().collect(),
            },
            Language::Italian => WordList {
                language,
                inner: ITALIAN.split_whitespace().collect(),
            },
            Language::Japanese => WordList {
                language,
                inner: JAPANESE.split_whitespace().collect(),
            },
            Language::Korean => WordList {
                language,
                inner: KOREAN.split_whitespace().collect(),
            },
            Language::Spanish => WordList {
                language,
                inner: SPANISH.split_whitespace().collect(),
            },
            Language::Portuguese => WordList {
                language,
                inner: PORTUGUESE.split_whitespace().collect(),
            },
        }
    }

    /// Gets the index of a word in a language's wordlist, returns error if word
    /// is not found in wordlist for a language
    pub fn get_index(&self, word: &str) -> Result<usize, ParseMnemonicError> {
        match self.inner.iter().position(|element| element == &word) {
            Some(index) => Ok(index),
            None => Err(ParseMnemonicError::InvalidWord(word.to_string())),
        }
    }

    /// If all words in the phrase are present in a language's wordlist, the
    /// language of the phrase is detected
    pub fn detect_language(phrase: Vec<&str>) -> Result<Language, ParseMnemonicError> {
        let all_languages = enum_iterator::all::<Language>().collect::<Vec<_>>();
        for language in all_languages {
            let wordlist = WordList::new(language);
            let mut matched_language = true;
            for word in &phrase {
                match wordlist.get_index(word) {
                    Ok(_) => continue,
                    Err(_) => {
                        matched_language = false;
                        break;
                    }
                }
            }

            if matched_language {
                return Ok(language);
            }
        }
        Err(ParseMnemonicError::InvalidPhraseLanguage(phrase.join(" ")))
    }

    /// Returns the language of the [WordList]
    pub fn language(&self) -> Language {
        self.language
    }

    /// Returns the inner wordlist
    pub fn inner(&self) -> Vec<&'static str> {
        self.inner.clone()
    }
}

/// The choice of language for a mnemonic phrase not only determines the words
/// used, but also has an impact on the binary value of each word when the
/// [`Mnemonic`](crate::Mnemonic) is converted into a [`Seed`](crate::Seed).
///
/// English is the only officially supported language, the rest are provided for
/// convenience.
///
/// The wordlists for each language are taken from the BIP39 repo: <https://github.com/bitcoin/bips/tree/master/bip-0039>
#[derive(Debug, Clone, Copy, PartialEq, Eq, enum_iterator::Sequence)]
pub enum Language {
    /// English, this is the only officially supported language
    English,
    /// Chinese Simplified
    ChineseSimplified,
    /// Chinese Traditional
    ChineseTraditional,
    /// Czech
    Czech,
    /// French
    French,
    /// Italian
    Italian,
    /// Japanese
    Japanese,
    /// Korean
    Korean,
    /// Portuguese
    Portuguese,
    /// Spanish
    Spanish,
}

impl FromStr for Language {
    type Err = ParseMnemonicError;

    /// Converts a string to a Language.
    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "English" => Ok(Language::English),
            "Chinese Simplified" => Ok(Language::ChineseSimplified),
            "Chinese Traditional" => Ok(Language::ChineseTraditional),
            "Czech" => Ok(Language::Czech),
            "French" => Ok(Language::French),
            "Italian" => Ok(Language::Italian),
            "Japanese" => Ok(Language::Japanese),
            "Korean" => Ok(Language::Korean),
            "Portuguese" => Ok(Language::Portuguese),
            "Spanish" => Ok(Language::Spanish),
            _ => Err(ParseMnemonicError::InvalidStrReprLang(input.into())),
        }
    }
}

impl Default for Language {
    /// Returns the default language, English.
    fn default() -> Language {
        Language::English
    }
}

impl LanguageExt for Language {
    type Language = Language;

    /// Returns a new Language with default language set.
    fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_language() {
        assert_eq!(Language::new(), Language::English);
    }

    #[test]
    fn test_from_str_language() {
        assert_eq!(Language::English, Language::from_str("English").unwrap());
        assert_eq!(
            Language::ChineseSimplified,
            Language::from_str("Chinese Simplified").unwrap()
        );
        assert_eq!(
            Language::ChineseTraditional,
            Language::from_str("Chinese Traditional").unwrap()
        );
        assert_eq!(Language::Czech, Language::from_str("Czech").unwrap());
        assert_eq!(Language::French, Language::from_str("French").unwrap());
        assert_eq!(Language::Italian, Language::from_str("Italian").unwrap());
        assert_eq!(Language::Japanese, Language::from_str("Japanese").unwrap());
        assert_eq!(Language::Korean, Language::from_str("Korean").unwrap());
        assert_eq!(
            Language::Portuguese,
            Language::from_str("Portuguese").unwrap()
        );
        assert_eq!(Language::Spanish, Language::from_str("Spanish").unwrap());
    }

    #[test]
    fn test_detect_language() {
        let phrase = vec![
            "outer", "ride", "neither", "foil", "glue", "number", "place", "usage", "ball", "shed",
            "dry", "point",
        ];
        assert_eq!(
            WordList::detect_language(phrase).unwrap(),
            Language::English
        );
    }

    #[test]
    fn test_fail_to_detect_language() {
        let phrase = vec![
            "outer", "ride", "neither", "foil", "glue", "number", "place", "usage", "ball", "shed",
            "dry", "pointx",
        ];

        assert!(WordList::detect_language(phrase).is_err());
    }

    #[test]
    fn test_chinese_simplified_wordlist() {
        let wordlist = WordList::new(Language::ChineseSimplified);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("的").unwrap(), 0);
        assert_eq!(wordlist.get_index("歇").unwrap(), 2047);
        assert!(wordlist.get_index("A").is_err()); // cant find a character thats not in the list
    }

    #[test]
    fn test_chinese_traditional_wordlist() {
        let wordlist = WordList::new(Language::ChineseTraditional);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("的").unwrap(), 0);
        assert_eq!(wordlist.get_index("歇").unwrap(), 2047);
        assert!(wordlist.get_index("A").is_err()); // cant find a character thats not in the list
    }

    #[test]
    fn test_czech_wordlist() {
        let wordlist = WordList::new(Language::Czech);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abdikace").unwrap(), 0);
        assert_eq!(wordlist.get_index("zvyk").unwrap(), 2047);
        assert!(wordlist.get_index("neplatný").is_err());
    }

    #[test]
    fn test_english_wordlist() {
        let wordlist = WordList::new(Language::English);
        assert_eq!(wordlist.language(), Language::English);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abandon").unwrap(), 0);
        assert_eq!(wordlist.get_index("zoo").unwrap(), 2047);
        assert!(wordlist.get_index("invalid").is_err());
    }

    #[test]
    fn test_french_wordlist() {
        let wordlist = WordList::new(Language::French);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abaisser").unwrap(), 0);
        assert_eq!(wordlist.get_index("zoologie").unwrap(), 2047);
        assert!(wordlist.get_index("invalide").is_err());
    }

    #[test]
    fn test_italian_wordlist() {
        let wordlist = WordList::new(Language::Italian);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abaco").unwrap(), 0);
        assert_eq!(wordlist.get_index("zuppa").unwrap(), 2047);
        assert!(wordlist.get_index("valido").is_err());
    }

    #[test]
    fn test_japanese_wordlist() {
        let wordlist = WordList::new(Language::Japanese);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("あいこくしん").unwrap(), 0);
        assert_eq!(wordlist.get_index("われる").unwrap(), 2047);
        assert!(wordlist.get_index("無効").is_err());
    }

    #[test]
    fn test_korean_wordlist() {
        let wordlist = WordList::new(Language::Korean);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("가격").unwrap(), 0);
        assert_eq!(wordlist.get_index("힘껏").unwrap(), 2047);
        assert!(wordlist.get_index("유효하지 않은").is_err());
    }

    #[test]
    fn test_portuguese_wordlist() {
        let wordlist = WordList::new(Language::Portuguese);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abacate").unwrap(), 0);
        assert_eq!(wordlist.get_index("zumbido").unwrap(), 2047);
        assert!(wordlist.get_index("inválido").is_err());
    }

    #[test]
    fn test_spanish_wordlist() {
        let wordlist = WordList::new(Language::Spanish);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("ábaco").unwrap(), 0);
        assert_eq!(wordlist.get_index("zurdo").unwrap(), 2047);
        assert!(wordlist.get_index("inválido").is_err());
    }
}
