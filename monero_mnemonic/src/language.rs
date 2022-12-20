static CHINESE_SIMPLIFIED: &'static str = include_str!("langs/chinese_simplified.txt");
static DUTCH: &'static str = include_str!("langs/dutch.txt");
static ENGLISH: &'static str = include_str!("langs/english.txt");
static ESPERANTO: &'static str = include_str!("langs/esperanto.txt");
static FRENCH: &'static str = include_str!("langs/french.txt");
static GERMAN: &'static str = include_str!("langs/german.txt");
static ITALIAN: &'static str = include_str!("langs/italian.txt");
static JAPANESE: &'static str = include_str!("langs/japanese.txt");
static LOJBAN: &'static str = include_str!("langs/lojban.txt");
static PORTUGUESE: &'static str = include_str!("langs/portuguese.txt");
static RUSSIAN: &'static str = include_str!("langs/russian.txt");
static SPANISH: &'static str = include_str!("langs/spanish.txt");

use crc::{crc32, Hasher32};
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub struct WordList {
    pub inner: Vec<&'static str>,
    pub prefix_length: usize,
    pub trimmed: Vec<&'static str>,
}

impl WordList {

    /// Creates a new wordlist for a specified language
    pub fn new(language: Language) -> WordList {
        match language {
            Language::English => {
                let words = ENGLISH.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 3,
                    trimmed: Self::create_trimmed_word_map(&ENGLISH, 3)
                }
            }
            Language::ChineseSimplified => {
                let words = CHINESE_SIMPLIFIED.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 1,
                    trimmed: Self::create_trimmed_word_map(&CHINESE_SIMPLIFIED, 1)
                }
            }
            Language::Dutch => {
                let words = DUTCH.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 4,
                    trimmed: Self::create_trimmed_word_map(&DUTCH, 4)
                }
            }
            Language::Esperanto => {
                let words = ESPERANTO.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 4,
                    trimmed: Self::create_trimmed_word_map(&ESPERANTO, 4)
                }
            }
            Language::French => {
                let words = FRENCH.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 4,
                    trimmed: Self::create_trimmed_word_map(&FRENCH, 4)
                }
            }
            Language::German => {
                let words = GERMAN.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 4,
                    trimmed: Self::create_trimmed_word_map(&GERMAN, 4)
                }
            }
            Language::Italian => {
                let words = ITALIAN.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 4,
                    trimmed: Self::create_trimmed_word_map(&ITALIAN, 4)
                }
            }
            Language::Japanese => {
                let words = JAPANESE.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 3,
                    trimmed: Self::create_trimmed_word_map(&JAPANESE, 3)
                }
            }
            Language::Lojban => {
                let words = LOJBAN.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 4,
                    trimmed: Self::create_trimmed_word_map(&LOJBAN, 4)
                }
            }
            Language::Portuguese => {
                let words = PORTUGUESE.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 4,
                    trimmed: Self::create_trimmed_word_map(&PORTUGUESE, 4)
                }
            }
            Language::Russian => {
                let words = RUSSIAN.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 4,
                    trimmed: Self::create_trimmed_word_map(&RUSSIAN, 4)
                }
            }
            Language::Spanish => {
                let words = SPANISH.split_whitespace().collect();
                WordList {
                    inner: words,
                    prefix_length: 4,
                    trimmed: Self::create_trimmed_word_map(&SPANISH, 4)
                }
            }
        }
    }

    /// Get the index of the word in the wordlist
    pub fn get_index(&self, word: &str) -> Result<usize, String> {
        match self.inner.iter().position(|element| element == &word) {
            Some(index) => Ok(index),
            None => Err("Invalid word".to_string()),
        }
    }

    /// Create a version of the wordlist with each word trimmed 
    pub fn create_trimmed_word_map(wordlist: &str, unique_prefix_len: usize) -> Vec<&str> {
        let wordlist2: Vec<&str> = wordlist.split_whitespace().collect();
        wordlist2
            .iter()
            .map(|word| &word[0..unique_prefix_len])
            .collect()
    }

    /// Trim one word
    pub fn to_trimmed(word: &str, unique_prefix_len: usize) -> String {
        match word.chars().count() > unique_prefix_len {
            true => word.chars().take(unique_prefix_len).collect(),
            false => word.into(),
        }
    }

    /// Get the index of the trimmed word
    pub fn get_trimmed_word_index(
        word: &str,
        trimmed_word_map: &HashMap<String, usize>,
        unique_prefix_len: usize,
    ) -> Result<usize, String> {
        let trimmed_word = Self::to_trimmed(word, unique_prefix_len);
        let index = trimmed_word_map.get(&trimmed_word);
        match index {
            None => {
                return Err(format!(
                    "Could not find trimmed word in word list. Attempted to find index of {}",
                    trimmed_word
                ))
            }
            Some(i) => return Ok(*i),
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
        phrase[(digest.sum32() % phrase.len() as u32) as usize]
            .clone()
            .to_string()
    }
}

/// The language determines which words will be used in a mnemonic phrase.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn default() -> Language {
        Language::English
    }
}

impl Language {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FromStr for Language {

    type Err = ();
  
    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "english"  => Ok(Language::English),
            "chinese_simplified"  => Ok(Language::ChineseSimplified),
            "dutch" => Ok(Language::Dutch),
            "esperanto" => Ok(Language::Esperanto),
            "french" => Ok(Language::French),
            "german" => Ok(Language::German),
            "italian" => Ok(Language::Italian),
            "japanese" => Ok(Language::Japanese),
            "lojban" => Ok(Language::Lojban),
            "portuguese" => Ok(Language::Portuguese),
            "russian" => Ok(Language::Russian),
            "spanish" => Ok(Language::Spanish),
            _      => Err(()),
        }
    }
}

impl fmt::Display for Language {
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
  fn test_wordlist() {
    let wordlist = WordList::new(Language::English);
    assert_eq!(wordlist.inner.len(), 1626);
    assert_eq!(wordlist.get_index("abbey").unwrap(), 0);
    assert_eq!(wordlist.get_index("zoom").unwrap(), 1625);
    assert!(wordlist.get_index("invalid").is_err());
  }
}