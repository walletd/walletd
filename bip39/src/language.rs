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

use std::str::FromStr;
use std::fmt;


#[derive(Debug)]
pub struct WordList {
    pub inner: Vec<&'static str>,
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

    pub fn get_index(&self, word: &str) -> Result<usize, String> {
        match self.inner.iter().position(|element| element == &word) {
            Some(index) => Ok(index),
            None => Err("Invalid word".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

  type Err = ();

  fn from_str(input: &str) -> Result<Language, Self::Err> {
      match input {
          "english"  => Ok(Language::English),
          "chinese_simplified"  => Ok(Language::ChineseSimplified),
          "chinese_traditional"  => Ok(Language::ChineseTraditional),
          "czech" => Ok(Language::Czech),
          "french" => Ok(Language::French),
          "italian" => Ok(Language::Italian),
          "japanese" => Ok(Language::Japanese),
          "korean" => Ok(Language::Korean),
          "portuguese" => Ok(Language::Portuguese),
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
    fn default() -> Language {
        Language::English
    }
}

impl Language {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_wordlist() {
    let wordlist = WordList::new(Language::English);
    assert_eq!(wordlist.inner.len(), 2048);
    assert_eq!(wordlist.get_index("abandon").unwrap(), 0);
    assert_eq!(wordlist.get_index("zoo").unwrap(), 2047);
    assert!(wordlist.get_index("invalid").is_err());
  }
}