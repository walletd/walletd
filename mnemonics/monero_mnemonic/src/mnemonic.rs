use crate::{Language, WordList, MnemonicHandler, MnemonicType};
use crate::mnemonic_type::BITS_IN_BYTES;
use core::str;
use curve25519_dalek::scalar::Scalar;
use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Mnemonic {
    phrase: String,
    lang: Language,
    seed: Vec<u8>,
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, " Language: {}", self.lang)?;
        writeln!(fmt, " Mnemonic Phrase: {}", &self.phrase)?;
        writeln!(fmt, " Seed hex: {}", hex::encode(self.seed.as_slice()))?;
        Ok(())
    }
}

impl MnemonicHandler for Mnemonic {
    type MnemonicStyle = Mnemonic;
    type LanguageHandler = Language;
    type MnemonicTypeSpec = MnemonicType;
    type WordListHandler = WordList;

    fn new(language: Language, mnemonic_type: MnemonicType, _passphrase: Option<&str>) -> Mnemonic {
        let wordlist = WordList::new(language);

        const DEFAULT_LENGTH: usize = 32;
        let random_bytes: [u8; DEFAULT_LENGTH] =
            Scalar::from_bytes_mod_order(thread_rng().gen()).to_bytes();

        let bytes_length = mnemonic_type.entropy_bits() / BITS_IN_BYTES;
        let entropy_bytes = random_bytes[..bytes_length].to_vec();

        let mnemonic_phrase = Self::bytes_to_words(&entropy_bytes, &wordlist).unwrap();

        Mnemonic {
            phrase: mnemonic_phrase,
            lang: language,
            seed: entropy_bytes,
        }
    }

    fn from_phrase(language: Language, mnemonic_phrase: &str, _passphrase: Option<&str>) -> Result<Mnemonic, String> {
        let phrase: Vec<&str> = mnemonic_phrase.split(" ").collect();
        let word_count = phrase.len();

        if word_count != 25 && word_count != 13 {
          return Err("The mneomonic phrase needs to be 25 or 13 words in length.".to_string());
        }
        let seed = Self::words_to_bytes(language, &mnemonic_phrase.to_string())?;

        Ok(Mnemonic {
            phrase: mnemonic_phrase.to_string(),
            lang: language,
            seed,
        })
    }

    fn bytes_to_words(entropy_bytes: &Vec<u8>, wordlist_info: &WordList) -> Result<String, String> {
        let wordlist = &wordlist_info.inner;
        if entropy_bytes.len() % 4 != 0 || entropy_bytes.len() == 0 {
            return Err(
                "Length of secret_bytes must be greater than 0 and divisible by 4".to_string(),
            );
        }

        let list_len: u32 = wordlist.len().try_into().unwrap();
        // Going to map 4 bytes to 3 words
        // First, each chunk of 4 bytes gets converted to a u32 number, using little endian representation
        let inputs = entropy_bytes
            .chunks(4)
            .map(|chunk| {
                let mut input: [u8; 4] = [0u8; 4];
                input.copy_from_slice(chunk);

                u32::from_le_bytes(input)
            })
            .collect::<Vec<u32>>();

        // Next, three words are generated from each of the 4 byte chunks, using the u32 numbers generated
        // Indices are calculated to represent each word in the mnemonic phrase in reference to the wordlist
        let mut phrase: Vec<&str> = vec![];
        for index in inputs {
            let w1 = index % list_len;
            let w2 = ((index / list_len) + w1) % list_len;
            let w3 = (((index / list_len) / list_len) + w2) % list_len;

            phrase.push(wordlist.get(w1 as usize).unwrap());
            phrase.push(wordlist.get(w2 as usize).unwrap());
            phrase.push(wordlist.get(w3 as usize).unwrap());
        }

        // The last word added to the mnemonic is a checksum word, it will repeat one of the previous words in the phrase
        let checksum_word = wordlist_info.checksum_word(&phrase);
        phrase.push(&checksum_word);
        Ok(phrase.join(" "))
    }

    fn words_to_bytes(language: Language, mnemonic_phrase: &String) -> Result<Vec<u8>, String> {
        let wordlist = WordList::new(language);
        let phrase: Vec<&str> = mnemonic_phrase.split(" ").collect();
        //let trimmed_word_map = &WordList::trimmed_word_map;
        let prefix_len = wordlist.prefix_length;
        let list_len = wordlist.inner.len();
        let word_count = phrase.len();
        if word_count != 25 && word_count != 13 {
            return Err("The mneomonic phrase needs to be 25 or 13 words in length.".to_string());
        }

        // The last word in the phrase is the checksum word
        let checksum = match phrase.last().copied() {
            Some(word) => word,
            _ => return Err("The mnemonic phrase is missing words".to_string()),
        };

        // Decode the phrase three words at a time, three words equals 4 bytes or one u32 number
        let mut buffer = vec![];
        let chunks = phrase.chunks(3);
        for chunk in chunks {
            let w1 = wordlist.get_index(chunk[0])?;
            let w2 = wordlist.get_index(chunk[1])?;
            let w3 = wordlist.get_index(chunk[2])?;
            let n = list_len;
            let x = w1 + n * (((n - w1) + w2) % n) + n * n * (((n - w2) + w3) % n);

            if x % n != w1 {
                return Err("Invalid mnemonic phrase, cannot be decoded".to_string());
            }
            buffer.extend_from_slice(&u32::to_le_bytes(x as u32));
        }
        // Verify the checksum
        let expected_checksum = wordlist.checksum_word(&phrase.into());
        let expected = WordList::to_trimmed(&expected_checksum, prefix_len);
        let found = WordList::to_trimmed(&checksum, prefix_len);
        if expected != found {
            return Err(format!(
                "The mnemonic phrase has an invalid checksum word, expected {}, found {}",
                expected, found
            ));
        }

        match word_count {
            25 => {
                let mut secret_bytes = [0u8; 32];
                secret_bytes.copy_from_slice(&buffer);
                return Ok(secret_bytes.to_vec());
            }
            13 => {
                let mut secret_bytes = [0u8; 32];
                let mut buffer_twice = vec![];
                buffer_twice.extend_from_slice(&buffer);
                buffer_twice.extend_from_slice(&buffer);
                secret_bytes.copy_from_slice(&buffer_twice);
                return Ok(secret_bytes.to_vec());
            }
            _ => {
                return Err(
                    "Word count of the mnemonic phrase needs to be either 25 or 13".to_string(),
                )
            }
        }
    }
    
    fn seed_hex(&self) -> Result<String, String> {
        let seed = hex::encode(self.seed.as_slice());
        Ok(seed)
    }

    fn seed_bytes(&self) -> Result<&[u8], String> {
        Ok(&self.seed.as_slice())
    }
}
