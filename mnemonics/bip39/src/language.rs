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

use walletd_mnemonics_core::Language;

use crate::Error;

#[derive(Debug)]
/// Represents a wordlist for a language for the Bip39 Mnemonic
pub struct WordList {
    language: Bip39Language,
    inner: Vec<&'static str>,
}

impl WordList {
    /// Creates a new [WordList] for a specifed language
    pub fn new(language: Bip39Language) -> WordList {
        match language {
            Bip39Language::English => WordList {
                language,
                inner: ENGLISH.split_whitespace().collect(),
            },
            Bip39Language::ChineseSimplified => WordList {
                language,
                inner: CHINESE_SIMPLIFIED.split_whitespace().collect(),
            },
            Bip39Language::ChineseTraditional => WordList {
                language,
                inner: CHINESE_TRADITIONAL.split_whitespace().collect(),
            },
            Bip39Language::Czech => WordList {
                language,
                inner: CZECH.split_whitespace().collect(),
            },
            Bip39Language::French => WordList {
                language,
                inner: FRENCH.split_whitespace().collect(),
            },
            Bip39Language::Italian => WordList {
                language,
                inner: ITALIAN.split_whitespace().collect(),
            },
            Bip39Language::Japanese => WordList {
                language,
                inner: JAPANESE.split_whitespace().collect(),
            },
            Bip39Language::Korean => WordList {
                language,
                inner: KOREAN.split_whitespace().collect(),
            },
            Bip39Language::Spanish => WordList {
                language,
                inner: SPANISH.split_whitespace().collect(),
            },
            Bip39Language::Portuguese => WordList {
                language,
                inner: PORTUGUESE.split_whitespace().collect(),
            },
        }
    }

    /// Gets the index of a word in a language's wordlist, returns error if word
    /// is not found in wordlist for a language
    pub fn get_index(&self, word: &str) -> Result<usize, Error> {
        match self.inner.iter().position(|element| element == &word) {
            Some(index) => Ok(index),
            None => Err(Error::InvalidWord(word.to_string())),
        }
    }

    /// If all words in the phrase are present in a language's wordlist, the
    /// language of the phrase is detected
    pub fn detect_language(phrase: Vec<&str>) -> Result<Bip39Language, Error> {
        let all_languages = enum_iterator::all::<Bip39Language>().collect::<Vec<_>>();
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
        Err(Error::InvalidPhraseLanguage(phrase.join(" ")))
    }

    /// Returns the language of the [WordList]
    pub fn language(&self) -> Bip39Language {
        self.language
    }

    /// Returns the inner wordlist
    pub fn inner(&self) -> Vec<&'static str> {
        self.inner.clone()
    }
}

/// The language of a Bip39 mnemonic phrase. English is the default language.
///
/// The choice of language for a mnemonic phrase not only determines the words
/// used, but also has an impact on the binary value of each word when the
/// [`Bip39Mnemonic`](crate::Bip39Mnemonic) is converted into a [`Seed`](crate::Seed).
///
/// English is the only officially supported language, the rest are provided for
/// convenience.
///
/// The wordlists for each language are taken from the BIP39 repo: <https://github.com/bitcoin/bips/tree/master/bip-0039>
#[derive(Debug, Clone, Copy, PartialEq, Eq, enum_iterator::Sequence)]
pub enum Bip39Language {
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

impl FromStr for Bip39Language {
    type Err = Error;

    /// Converts a string to a Language.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "English" => Ok(Self::English),
            "Chinese Simplified" => Ok(Self::ChineseSimplified),
            "Chinese Traditional" => Ok(Self::ChineseTraditional),
            "Czech" => Ok(Self::Czech),
            "French" => Ok(Self::French),
            "Italian" => Ok(Self::Italian),
            "Japanese" => Ok(Self::Japanese),
            "Korean" => Ok(Self::Korean),
            "Portuguese" => Ok(Self::Portuguese),
            "Spanish" => Ok(Self::Spanish),
            _ => Err(Error::InvalidStrReprLang(input.into())),
        }
    }
}

impl Default for Bip39Language {
    /// Returns the default language, English.
    fn default() -> Self {
        Self::English
    }
}

impl Language for Bip39Language {
    type Language = Self;

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
        assert_eq!(Bip39Language::new(), Bip39Language::English);
    }

    #[test]
    fn test_from_str_language() {
        assert_eq!(
            Bip39Language::English,
            Bip39Language::from_str("English").unwrap()
        );
        assert_eq!(
            Bip39Language::ChineseSimplified,
            Bip39Language::from_str("Chinese Simplified").unwrap()
        );
        assert_eq!(
            Bip39Language::ChineseTraditional,
            Bip39Language::from_str("Chinese Traditional").unwrap()
        );
        assert_eq!(
            Bip39Language::Czech,
            Bip39Language::from_str("Czech").unwrap()
        );
        assert_eq!(
            Bip39Language::French,
            Bip39Language::from_str("French").unwrap()
        );
        assert_eq!(
            Bip39Language::Italian,
            Bip39Language::from_str("Italian").unwrap()
        );
        assert_eq!(
            Bip39Language::Japanese,
            Bip39Language::from_str("Japanese").unwrap()
        );
        assert_eq!(
            Bip39Language::Korean,
            Bip39Language::from_str("Korean").unwrap()
        );
        assert_eq!(
            Bip39Language::Portuguese,
            Bip39Language::from_str("Portuguese").unwrap()
        );
        assert_eq!(
            Bip39Language::Spanish,
            Bip39Language::from_str("Spanish").unwrap()
        );
    }

    #[test]
    fn test_detect_language() {
        let phrase = vec![
            "outer", "ride", "neither", "foil", "glue", "number", "place", "usage", "ball", "shed",
            "dry", "point",
        ];
        assert_eq!(
            WordList::detect_language(phrase).unwrap(),
            Bip39Language::English
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
        let wordlist = WordList::new(Bip39Language::ChineseSimplified);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("的").unwrap(), 0);
        assert_eq!(wordlist.get_index("歇").unwrap(), 2047);
        assert!(wordlist.get_index("A").is_err()); // cant find a character thats not in the list
    }

    #[test]
    fn test_chinese_traditional_wordlist() {
        let wordlist = WordList::new(Bip39Language::ChineseTraditional);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("的").unwrap(), 0);
        assert_eq!(wordlist.get_index("歇").unwrap(), 2047);
        assert!(wordlist.get_index("A").is_err()); // cant find a character thats not in the list
    }

    #[test]
    fn test_czech_wordlist() {
        let wordlist = WordList::new(Bip39Language::Czech);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abdikace").unwrap(), 0);
        assert_eq!(wordlist.get_index("zvyk").unwrap(), 2047);
        assert!(wordlist.get_index("neplatný").is_err());
    }

    #[test]
    fn test_english_wordlist() {
        let wordlist = WordList::new(Bip39Language::English);
        assert_eq!(wordlist.language(), Bip39Language::English);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abandon").unwrap(), 0);
        assert_eq!(wordlist.get_index("zoo").unwrap(), 2047);
        assert!(wordlist.get_index("invalid").is_err());
    }

    #[test]
    fn test_french_wordlist() {
        let wordlist = WordList::new(Bip39Language::French);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abaisser").unwrap(), 0);
        assert_eq!(wordlist.get_index("zoologie").unwrap(), 2047);
        assert!(wordlist.get_index("invalide").is_err());
    }

    #[test]
    fn test_italian_wordlist() {
        let wordlist = WordList::new(Bip39Language::Italian);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abaco").unwrap(), 0);
        assert_eq!(wordlist.get_index("zuppa").unwrap(), 2047);
        assert!(wordlist.get_index("valido").is_err());
    }

    #[test]
    fn test_japanese_wordlist() {
        let wordlist = WordList::new(Bip39Language::Japanese);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("あいこくしん").unwrap(), 0);
        assert_eq!(wordlist.get_index("われる").unwrap(), 2047);
        assert!(wordlist.get_index("無効").is_err());
    }

    #[test]
    fn test_korean_wordlist() {
        let wordlist = WordList::new(Bip39Language::Korean);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("가격").unwrap(), 0);
        assert_eq!(wordlist.get_index("힘껏").unwrap(), 2047);
        assert!(wordlist.get_index("유효하지 않은").is_err());
    }

    #[test]
    fn test_portuguese_wordlist() {
        let wordlist = WordList::new(Bip39Language::Portuguese);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("abacate").unwrap(), 0);
        assert_eq!(wordlist.get_index("zumbido").unwrap(), 2047);
        assert!(wordlist.get_index("inválido").is_err());
    }

    #[test]
    fn test_spanish_wordlist() {
        let wordlist = WordList::new(Bip39Language::Spanish);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("ábaco").unwrap(), 0);
        assert_eq!(wordlist.get_index("zurdo").unwrap(), 2047);
        assert!(wordlist.get_index("inválido").is_err());
    }
}
