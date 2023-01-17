static CHINESE_SIMPLIFIED: &'static str = include_str!("langs/chinese_simplified.txt");
static CHINESE_TRADITIONAL: &'static str = include_str!("langs/chinese_traditional.txt");
static CZECH: &'static str = include_str!("langs/czech.txt");
static ENGLISH: &'static str = include_str!("langs/english.txt");
static FRENCH: &'static str = include_str!("langs/french.txt");
static ITALIAN: &'static str = include_str!("langs/italian.txt");
static JAPANESE: &'static str = include_str!("langs/japanese.txt");
static KOREAN: &'static str = include_str!("langs/korean.txt");
static SPANISH: &'static str = include_str!("langs/spanish.txt");
static PORTUGUESE: &'static str = include_str!("langs/portuguese.txt");

use anyhow::anyhow;
use std::fmt;
use std::str::FromStr;

use walletd_mnemonic_model::LanguageHandler;

#[derive(Debug)]
pub struct WordList {
    inner: Vec<&'static str>,
}

impl WordList {
    pub fn new(language: Language) -> WordList {
        match language {
            Language::English => WordList {
                inner: ENGLISH.split_whitespace().collect(),
            },
            Language::ChineseSimplified => WordList {
                inner: CHINESE_SIMPLIFIED.split_whitespace().collect(),
            },
            Language::ChineseTraditional => WordList {
                inner: CHINESE_TRADITIONAL.split_whitespace().collect(),
            },
            Language::Czech => WordList {
                inner: CZECH.split_whitespace().collect(),
            },
            Language::French => WordList {
                inner: FRENCH.split_whitespace().collect(),
            },
            Language::Italian => WordList {
                inner: ITALIAN.split_whitespace().collect(),
            },
            Language::Japanese => WordList {
                inner: JAPANESE.split_whitespace().collect(),
            },
            Language::Korean => WordList {
                inner: KOREAN.split_whitespace().collect(),
            },
            Language::Spanish => WordList {
                inner: SPANISH.split_whitespace().collect(),
            },
            Language::Portuguese => WordList {
                inner: PORTUGUESE.split_whitespace().collect(),
            },
        }
    }

    pub fn get_index(&self, word: &str) -> Result<usize, anyhow::Error> {
        match self.inner.iter().position(|element| element == &word) {
            Some(index) => Ok(index),
            None => Err(anyhow!("Invalid word")),
        }
    }

    /// If all words in the phrase are present in a language's wordlist, the language of the phrase is detected
    pub fn detect_language(phrase: Vec<&str>) -> Result<Language, anyhow::Error> {
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
        Err(anyhow!(
            "Could not find a language match for the given phrase"
        ))
    }

    pub fn inner(&self) -> Vec<&'static str> {
        self.inner.clone()
    }
}

/// The choice of language for a mnemonic phrase not only determines the words used,
/// but also has an impact on the binary value of each word when the ['Mnemonic'][Mnemonic] is converted into a ['Seed'][Seed].
///
/// English is the only officially supported language, the rest are provided for convenience.
///
/// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
/// [Seed]: ./seed/struct.Seed.html
#[derive(Debug, Clone, Copy, PartialEq, enum_iterator::Sequence)]
pub enum Language {
    English,
    ChineseSimplified,
    ChineseTraditional,
    Czech,
    French,
    Italian,
    Japanese,
    Korean,
    Portuguese,
    Spanish,
}

impl FromStr for Language {
    type Err = anyhow::Error;
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
            _ => Err(anyhow!("Could not match str {} to a language", input))?,
        }
    }
}

impl fmt::Display for Language {
    /// Converts a Language to a string.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::English => fmt.write_str("English")?,
            Language::ChineseSimplified => fmt.write_str("Chinese Simplified")?,
            Language::ChineseTraditional => fmt.write_str("Chinese Traditional")?,
            Language::Czech => fmt.write_str("Czech")?,
            Language::French => fmt.write_str("French")?,
            Language::Italian => fmt.write_str("Italian")?,
            Language::Japanese => fmt.write_str("Japanese")?,
            Language::Korean => fmt.write_str("Korean")?,
            Language::Portuguese => fmt.write_str("Portuguese")?,
            Language::Spanish => fmt.write_str("Spanish")?,
        };
        Ok(())
    }
}

impl Default for Language {
    /// Returns the default language, English.
    fn default() -> Language {
        Language::English
    }
}

impl LanguageHandler for Language {
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
    fn test_print_language() {
        assert_eq!(format!("{}", Language::English), "English");
        assert_eq!(
            format!("{}", Language::ChineseSimplified),
            "Chinese Simplified"
        );
        assert_eq!(
            format!("{}", Language::ChineseTraditional),
            "Chinese Traditional"
        );
        assert_eq!(format!("{}", Language::Czech), "Czech");
        assert_eq!(format!("{}", Language::French), "French");
        assert_eq!(format!("{}", Language::Italian), "Italian");
        assert_eq!(format!("{}", Language::Japanese), "Japanese");
        assert_eq!(format!("{}", Language::Korean), "Korean");
        assert_eq!(format!("{}", Language::Portuguese), "Portuguese");
        assert_eq!(format!("{}", Language::Spanish), "Spanish");
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
    fn test_chinese_simplified_wordlist() {
        let wordlist = WordList::new(Language::ChineseSimplified);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("的").unwrap(), 0);
        assert_eq!(wordlist.get_index("歇").unwrap(), 2047);
        // assert!(wordlist.get_index("效").is_err()); // cant find a character thats not in the list
    }

    #[test]
    fn test_chinese_traditional_wordlist() {
        let wordlist = WordList::new(Language::ChineseTraditional);
        assert_eq!(wordlist.inner.len(), 2048);
        assert_eq!(wordlist.get_index("的").unwrap(), 0);
        assert_eq!(wordlist.get_index("歇").unwrap(), 2047);
        // assert!(wordlist.get_index("效").is_err()); // cant find a character thats not in the list
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
