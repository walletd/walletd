static CHINESE_SIMPLIFIED: &str = include_str!("langs/chinese_simplified.txt");
static DUTCH: &str = include_str!("langs/dutch.txt");
static ENGLISH: &str = include_str!("langs/english.txt");
static ESPERANTO: &str = include_str!("langs/esperanto.txt");
static FRENCH: &str = include_str!("langs/french.txt");
static GERMAN: &str = include_str!("langs/german.txt");
static ITALIAN: &str = include_str!("langs/italian.txt");
static JAPANESE: &str = include_str!("langs/japanese.txt");
static LOJBAN: &str = include_str!("langs/lojban.txt");
static PORTUGUESE: &str = include_str!("langs/portuguese.txt");
static RUSSIAN: &str = include_str!("langs/russian.txt");
static SPANISH: &str = include_str!("langs/spanish.txt");

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crc::{crc32, Ha
use walletd_mnemonics_core::Languag
use crate::Error;

#[derive(Debug)]
pub struct WordList {
    language: Language,
    inner: Vec<&'static str>,
    prefix_length: usize,
    trimmed_word_map: HashMap<String, usize>,
}

impl WordList {
    /// Creates a new wordlist for a specified language
    pub fn new(language: Language) -> WordList {
        match language {
            Language::English => {
                let words = ENGLISH.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 3,
                    trimmed_word_map: Self::create_trimmed_word_list(ENGLISH, 3),
                }
            }
            Language::ChineseSimplified => {
                let words = CHINESE_SIMPLIFIED.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 1,
                    trimmed_word_map: Self::create_trimmed_word_list(CHINESE_SIMPLIFIED, 1),
                }
            }
            Language::Dutch => {
                let words = DUTCH.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 4,
                    trimmed_word_map: Self::create_trimmed_word_list(DUTCH, 4),
                }
            }
            Language::Esperanto => {
                let words = ESPERANTO.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 4,
                    trimmed_word_map: Self::create_trimmed_word_list(ESPERANTO, 4),
                }
            }
            Language::French => {
                let words = FRENCH.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 4,
                    trimmed_word_map: Self::create_trimmed_word_list(FRENCH, 4),
                }
            }
            Language::German => {
                let words = GERMAN.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 4,
                    trimmed_word_map: Self::create_trimmed_word_list(GERMAN, 4),
                }
            }
            Language::Italian => {
                let words = ITALIAN.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 4,
                    trimmed_word_map: Self::create_trimmed_word_list(ITALIAN, 4),
                }
            }
            Language::Japanese => {
                let words = JAPANESE.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 3,
                    trimmed_word_map: Self::create_trimmed_word_list(JAPANESE, 3),
                }
            }
            Language::Lojban => {
                let words = LOJBAN.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 4,
                    trimmed_word_map: Self::create_trimmed_word_list(LOJBAN, 4),
                }
            }
            Language::Portuguese => {
                let words = PORTUGUESE.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 4,
                    trimmed_word_map: Self::create_trimmed_word_list(PORTUGUESE, 4),
                }
            }
            Language::Russian => {
                let words = RUSSIAN.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 4,
                    trimmed_word_map: Self::create_trimmed_word_list(RUSSIAN, 4),
                }
            }
            Language::Spanish => {
                let words = SPANISH.split_whitespace().collect();
                WordList {
                    language,
                    inner: words,
                    prefix_length: 4,
                    trimmed_word_map: Self::create_trimmed_word_list(SPANISH, 4),
                }
            }
        }
    }

    /// Gets the index of a word in a language's wordlist, returns error if word
    /// is not found in wordlist for a language
    #[allow(dead_code)]
    pub fn get_index(&self, word: &str) -> Result<usize, Error> {
        match self.inner.iter().position(|element| element == &word) {
            Some(index) => Ok(index),
            None => Err(Error::InvalidWord(
                word.to_string(),
                self.language.to_string(),
            )),
        }
    }

    /// If all words in the phrase are present in a language's wordlist, the
    /// language of the phrase is detected
    pub fn detect_language(phrase: Vec<&str>) -> Result<Language, Error> {
        let all_languages = enum_iterator::all::<Language>().collect::<Vec<_>>();
        for language in all_languages {
            let wordlist = WordList::new(language);
            let mut matched_language = true;
            for word in &phrase {
                match wordlist.trimmed_word_index(word) {
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

    /// Create a version of the wordlist with each word trimmed
    pub fn create_trimmed_word_list(
        wordlist: &str,
        unique_prefix_len: usize,
    ) -> HashMap<String, usize> {
        let wordlist2: Vec<&str> = wordlist.split_whitespace().collect();
        let mut trimmed_word_map = HashMap::new();
        for (index, word) in wordlist2.iter().enumerate() {
            trimmed_word_map.insert(Self::to_trimmed(word, unique_prefix_len), index);
        }
        trimmed_word_map
    }

    /// Trim one word
    pub fn to_trimmed(word: &str, unique_prefix_len: usize) -> String {
        match word.chars().count() > unique_prefix_len {
            true => word.chars().take(unique_prefix_len).collect(),
            false => word.into(),
        }
    }

    /// Get the index of the trimmed word
    pub fn trimmed_word_index(&self, word: &str) -> Result<usize, Error> {
        let trimmed_word = Self::to_trimmed(word, self.prefix_length());
        let index = self.trimmed_word_map.get(&trimmed_word);
        match index {
            None => Err(Error::InvalidWord(
                format!("trimmed_word {}", trimmed_word),
                self.language.to_string(),
            )),
            Some(i) => Ok(*i),
        }
    }

    /// Calculate the checksum word
    pub fn checksum_word(&self, phrase: &Vec<&str>) -> String {
        let phrase_trimmed = phrase
            .iter()
            .map(|word| WordList::to_trimmed(word, self.prefix_length))
            .collect::<Vec<String>>();

        let mut digest = crc32::Digest::new(crc32::IEEE);
        digest.write(phrase_trimmed.concat().as_bytes());
        phrase[(digest.sum32() % phrase.len() as u32) as usize].to_string()
    }

    /// Get inner data
    pub fn inner(&self) -> Vec<&'static str> {
        self.inner.clone()
    }

    /// Get the prefix_length
    pub fn prefix_length(&self) -> usize {
        self.prefix_length
    }

    /// Get the trimmed_word_map
    #[allow(dead_code)]
    pub fn trimmed_word_map(&self) -> HashMap<String, usize> {
        self.trimmed_word_map.clone()
    }

    /// Get the wordlist's language
    #[allow(dead_code)]
    pub fn language(&self) -> Language {
        self.language
    }
}

/// The choice of language for a mnemonic phrase not only determines the words
/// used, but also has an impact on the binary value of each word when the
/// ['Mnemonic'][Mnemonic] is converted into a ['Seed'][Seed].
///
/// English is the only officially supported language, the rest are provided for
/// convenience.
///
/// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
/// [Seed]: ./seed/struct.Seed.html
#[derive(Debug, Clone, Copy, Eq, PartialEq, enum_iterator::Sequence)]
pub enum Language {
    English,
    ChineseSimplified,
    Dutch,
    Esperanto,
    French,
    German,
    Italian,
    Japanese,
    Lojban,
    Portuguese,
    Russian,
    Spanish,
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

impl FromStr for Language {
    type Err = Error;

    /// Converts a string to a Language.
    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "English" => Ok(Language::English),
            "Chinese Simplified" => Ok(Language::ChineseSimplified),
            "Dutch" => Ok(Language::Dutch),
            "Esperanto" => Ok(Language::Esperanto),
            "French" => Ok(Language::French),
            "German" => Ok(Language::German),
            "Italian" => Ok(Language::Italian),
            "Japanese" => Ok(Language::Japanese),
            "Lojban" => Ok(Language::Lojban),
            "Portuguese" => Ok(Language::Portuguese),
            "Russian" => Ok(Language::Russian),
            "Spanish" => Ok(Language::Spanish),
            _ => Err(Error::InvalidStrReprLang(input.into())),
        }
    }
}

impl fmt::Display for Language {
    /// Converts a Language to a string.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::English => fmt.write_str("English")?,
            Language::ChineseSimplified => fmt.write_str("Chinese Simplified")?,
            Language::Dutch => fmt.write_str("Dutch")?,
            Language::Esperanto => fmt.write_str("Esperanto")?,
            Language::French => fmt.write_str("French")?,
            Language::German => fmt.write_str("German")?,
            Language::Italian => fmt.write_str("Italian")?,
            Language::Japanese => fmt.write_str("Japanese")?,
            Language::Lojban => fmt.write_str("Lojban")?,
            Language::Portuguese => fmt.write_str("Portuguese")?,
            Language::Russian => fmt.write_str("Russian")?,
            Language::Spanish => fmt.write_str("Spanish")?,
        };
        Ok(())
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
        assert_eq!(Language::Dutch, Language::from_str("Dutch").unwrap());
        assert_eq!(
            Language::Esperanto,
            Language::from_str("Esperanto").unwrap()
        );
        assert_eq!(Language::French, Language::from_str("French").unwrap());
        assert_eq!(Language::German, Language::from_str("German").unwrap());
        assert_eq!(Language::Italian, Language::from_str("Italian").unwrap());
        assert_eq!(Language::Japanese, Language::from_str("Japanese").unwrap());
        assert_eq!(Language::Lojban, Language::from_str("Lojban").unwrap());
        assert_eq!(
            Language::Portuguese,
            Language::from_str("Portuguese").unwrap()
        );
        assert_eq!(Language::Russian, Language::from_str("Russian").unwrap());
        assert_eq!(Language::Spanish, Language::from_str("Spanish").unwrap());
    }

    #[test]
    fn test_print_language() {
        assert_eq!(format!("{}", Language::English), "English");
        assert_eq!(
            format!("{}", Language::ChineseSimplified),
            "Chinese Simplified"
        );
        assert_eq!(format!("{}", Language::Dutch), "Dutch");
        assert_eq!(format!("{}", Language::Esperanto), "Esperanto");
        assert_eq!(format!("{}", Language::French), "French");
        assert_eq!(format!("{}", Language::German), "German");
        assert_eq!(format!("{}", Language::Italian), "Italian");
        assert_eq!(format!("{}", Language::Japanese), "Japanese");
        assert_eq!(format!("{}", Language::Lojban), "Lojban");
        assert_eq!(format!("{}", Language::Portuguese), "Portuguese");
        assert_eq!(format!("{}", Language::Russian), "Russian");
        assert_eq!(format!("{}", Language::Spanish), "Spanish");
    }

    #[test]
    fn test_detect_language() {
        let phrase = vec![
            "buzzer",
            "eject",
            "zeal",
            "algebra",
            "adept",
            "arrow",
            "shipped",
            "mobile",
            "reorder",
            "light",
            "plus",
            "rover",
            "fawns",
            "fight",
            "aphid",
            "powder",
            "tufts",
            "niche",
            "plotting",
            "acumen",
            "equip",
            "civilian",
            "camp",
            "dialect algebra",
        ];
        assert_eq!(
            WordList::detect_language(phrase).unwrap(),
            Language::English
        );
    }

    #[test]
    fn test_chinese_simplified_wordlist() {
        let wordlist = WordList::new(Language::ChineseSimplified);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("的").unwrap(), 0);
        assert_eq!(wordlist.get_index("貌").unwrap(), 1625);
        assert!(wordlist.get_index("A").is_err()); // cant find a character
                                                   // thats not in the list
    }

    #[test]
    fn test_dutch_wordlist() {
        let wordlist = WordList::new(Language::Dutch);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("aalglad").unwrap(), 0);
        assert_eq!(wordlist.get_index("zworen").unwrap(), 1625);
        assert!(wordlist.get_index("neplatný").is_err());
    }

    #[test]
    fn test_english_wordlist() {
        let wordlist = WordList::new(Language::English);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("abbey").unwrap(), 0);
        assert_eq!(wordlist.get_index("zoom").unwrap(), 1625);
        assert!(wordlist.get_index("invalid").is_err());
    }

    #[test]
    fn test_esperanto_wordlist() {
        let wordlist = WordList::new(Language::Esperanto);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("abako").unwrap(), 0);
        assert_eq!(wordlist.get_index("zumilo").unwrap(), 1625);
        assert!(wordlist.get_index("neplatný").is_err());
    }

    #[test]
    fn test_french_wordlist() {
        let wordlist = WordList::new(Language::French);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("abandon").unwrap(), 0);
        assert_eq!(wordlist.get_index("zoom").unwrap(), 1625);
        assert!(wordlist.get_index("invalide").is_err());
    }

    #[test]
    fn test_german_wordlist() {
        let wordlist = WordList::new(Language::German);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("Abakus").unwrap(), 0);
        assert_eq!(wordlist.get_index("Zyklop").unwrap(), 1625);
        assert!(wordlist.get_index("invalide").is_err());
    }

    #[test]
    fn test_italian_wordlist() {
        let wordlist = WordList::new(Language::Italian);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("abbinare").unwrap(), 0);
        assert_eq!(wordlist.get_index("zucchero").unwrap(), 1625);
        assert!(wordlist.get_index("valido").is_err());
    }

    #[test]
    fn test_japanese_wordlist() {
        let wordlist = WordList::new(Language::Japanese);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("あいこくしん").unwrap(), 0);
        assert_eq!(wordlist.get_index("ひしょ").unwrap(), 1625);
        assert!(wordlist.get_index("無効").is_err());
    }

    #[test]
    fn test_lojban_wordlist() {
        let wordlist = WordList::new(Language::Lojban);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("backi").unwrap(), 0);
        assert_eq!(wordlist.get_index("snaxa'a").unwrap(), 1625);
        assert!(wordlist.get_index("유효하지 않은").is_err());
    }

    #[test]
    fn test_portuguese_wordlist() {
        let wordlist = WordList::new(Language::Portuguese);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("abaular").unwrap(), 0);
        assert_eq!(wordlist.get_index("zumbi").unwrap(), 1625);
        assert!(wordlist.get_index("inválido").is_err());
    }

    #[test]
    fn test_russian_wordlist() {
        let wordlist = WordList::new(Language::Russian);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("абажур").unwrap(), 0);
        assert_eq!(wordlist.get_index("ящик").unwrap(), 1625);
        assert!(wordlist.get_index("inválido").is_err());
    }

    #[test]
    fn test_spanish_wordlist() {
        let wordlist = WordList::new(Language::Spanish);
        assert_eq!(wordlist.inner.len(), 1626);
        assert_eq!(wordlist.get_index("ábaco").unwrap(), 0);
        assert_eq!(wordlist.get_index("rito").unwrap(), 1625);
        assert!(wordlist.get_index("inválido").is_err());
    }
}
