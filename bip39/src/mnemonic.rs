use crate::MnemonicType;
use crate::{Language, WordList};
use crate::mnemonic_type::ENTROPY_OFFSET;
use mnemonic_model::MnemonicHandler;
use bitvec::prelude::*;
use core::{ops::Div, str};
use curve25519_dalek::scalar::Scalar;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256, Sha512};
use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct Mnemonic {
    phrase: String,
    lang: Language,
    seed: Vec<u8>
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, " Language: {}", self.lang)?;
        writeln!(fmt, " Mnemonic Phrase: {}", &self.phrase)?;
        writeln!(fmt, " Seed hex: {}", hex::encode(self.seed.as_slice()))?;
        Ok(())
    }
}

impl Mnemonic {

    pub fn to_seed(language: Language, mnemonic_phrase: &String, provided_passphrase: Option<&str>) -> Result<String, String> {
        
        let mut passphrase = "".to_string();
        if let Some(pass) = provided_passphrase {
            passphrase = pass.to_string();
        }
        // check if mnemonic phrase is valid first
        Self::words_to_bytes(language, mnemonic_phrase)?;

        const PBKDF2_ROUNDS: usize = 2048;
        const PBKDF2_BYTES: usize = 64;
        let mut seed_bytes = vec![0u8; PBKDF2_BYTES];
        let salt = format!("mnemonic{}", passphrase);

        pbkdf2::<Hmac<Sha512>>(
            mnemonic_phrase.as_bytes(),
            salt.as_bytes(),
            PBKDF2_ROUNDS,
            &mut seed_bytes,
        );
        Ok(hex::encode(seed_bytes))
    }

}
impl MnemonicHandler for Mnemonic {
    type MnemonicStyle = Mnemonic;
    type LanguageHandler = Language;
    type MnemonicTypeSpec = MnemonicType;
    type WordListHandler = WordList;

    /// Generates a new mnemonic given the language, length of mnemonic, and optional passphrase
    fn new(language: Language, mnemonic_type: MnemonicType, passphrase: Option<&str>) -> Mnemonic {
        let wordlist = WordList::new(language);

        const DEFAULT_LENGTH: usize = 32;
        let random_bytes: [u8; DEFAULT_LENGTH] =
            Scalar::from_bytes_mod_order(thread_rng().gen()).to_bytes();

        let bytes_length = mnemonic_type.entropy_bits() / ENTROPY_OFFSET;
        let entropy_bytes = random_bytes[..bytes_length].to_vec();

        let mnemonic_phrase = Self::bytes_to_words(&entropy_bytes, &wordlist).unwrap();
        let seed_hex = Self::to_seed(language, &mnemonic_phrase, passphrase).unwrap();

        Mnemonic {
            phrase: mnemonic_phrase,
            lang: language,
            seed: hex::decode(seed_hex).unwrap().to_vec(),       
        }
    }
    
    /// Creates a mnemonic object given a mnemonic_phrase, language and an optional passphrase
    fn from_phrase(language: Language, mnemonic_phrase: &str, passphrase: Option<&str>) -> Result<Mnemonic, String> {
        let phrase: Vec<&str> = mnemonic_phrase.split(" ").collect();
        println!("phrase: {:?}", phrase);

        let word_count = phrase.len();

        if word_count % 3 != 0 {
            return Err(
                "The number of words in the mnemonic phrase should be a multiple of 3.".to_string(),
            );
        }
        if word_count == 0 {
            return Err("The mnemonic phrase does not contain any words".to_string());
        }
        if (word_count < 12) || (word_count > 24) {
            return Err("The number of words in the mnemonic phrase should not be less than 12 or greater than 24".to_string());
        }
        
        let seed_hex = Self::to_seed(language, &mnemonic_phrase.to_string(), passphrase)?;
        println!("seed_hex {}", seed_hex);

        Ok(Mnemonic {
            phrase: mnemonic_phrase.to_string(),
            lang: language,
            seed: hex::decode(seed_hex).unwrap(),
        })
    }

    /// Converting entropy bytes to the mnemonic words, given a wordlist
    fn bytes_to_words(entropy_bytes: &Vec<u8>, wordlist_info: &WordList) -> Result<String, String> {
        if entropy_bytes.len() % 4 != 0 {
            return Err("Entropy must be a multiple of 4 bytes (32 bits) in length".to_string());
        }
        if (entropy_bytes.len() < 128 / ENTROPY_OFFSET)
            || (entropy_bytes.len() > 256 / ENTROPY_OFFSET)
        {
            return Err("Entropy must be between 128 and 256 bits in length".to_string());
        }

        // Take the sh256 hash of the entropy
        let mut sha256 = Sha256::new();
        sha256.input(entropy_bytes.as_slice());
        let hash = sha256.result();

        // number of words in mnemoic phrase depends on the number of bits in entropy_bytes
        // the number of bits in entropy_bytes (entropy_bytes * 8) + checksum length (1 bit per 32 bits in entropy_bytes)
        // equals the total number of bits which will be a multiple of 33
        // one word will be specified per 11 bits
        // word_count = (entropy_bytes * 8) + (entropy_bytes/32)/11
        let entropy_bits = entropy_bytes.len() * ENTROPY_OFFSET;
        let word_count = (entropy_bits + (entropy_bits / 32)) / 11;

        // We then take 1 bit of that hash for every 32 bits of entropy, and add it to the end of our entropy.
        let hash_0 = BitVec::<Msb0, u8>::from_element(hash[0]);
        let (checksum, _) = hash_0.split_at(word_count.div(3) as usize);
        let mut encoding = BitVec::<Msb0, u8>::from_vec(entropy_bytes.clone());
        encoding.append(&mut checksum.to_vec());

        // Compute the phrase in 11 bit chunks which encode an index into the word list
        let wordlist = &wordlist_info.inner;

        let phrase = encoding
            .chunks(11)
            .map(|index| {
                // Convert a vector of 11 bits into a u11 number.
                let index = index
                    .iter()
                    .enumerate()
                    .map(|(i, &bit)| (bit as u16) * 2u16.pow(10 - i as u32))
                    .sum::<u16>();

                wordlist[index as usize]
            })
            .collect::<Vec<&str>>();
        Ok(phrase.join(" "))
    }

    /// Converts the words of a mnemonic phrase to the bytes representation
    fn words_to_bytes(language: Language, mnemonic_phrase: &String) -> Result<Vec<u8>, String> {
        let wordlist = WordList::new(language);
        let phrase: Vec<&str> = mnemonic_phrase.split(" ").collect();
        let word_count = phrase.len();

        // Each word in the mnemonic phrase represents 11 bits
        // A checksum was added to the entropy with a length equal to the number of entropy bits divided by 32
        // So, the number of original entropy bits can be found: phrase.len() * 11 - (entropy_bits/32) = entropy_bits
        // 32 * 11 * phrase.len() - entropy_bits = 32 * entropy_bits
        // 33 * entropy_bits = 32 * 11 * phrase.len()
        // entropy_bits = (32 * 11 * phrase.len()) /  33
        let entropy_bits = (32 * 11 * word_count) / 33;
        let mut entropy: BitVec<Msb0, u8> = BitVec::new();

        for word in phrase {
            let index = wordlist.get_index(word)?;
            let index_u8: [u8; 2] = (index as u16).to_be_bytes();
            let index_slice = &BitVec::from_slice(&index_u8)[5..];
            entropy.append(&mut BitVec::<Msb0, u8>::from_bitslice(index_slice));
        }
        let entropy_bytes = entropy[..entropy_bits].as_slice().to_vec();
        match *mnemonic_phrase == Self::bytes_to_words(&entropy_bytes, &wordlist).unwrap() {
            true => Ok(entropy_bytes),
            false => Err("Invalid mnemonic phrase, the checksum word does not match".to_string()),
        }
    }

    /// Returns the seed as in a hexadecimal representation
    fn seed_hex(&self) -> Result<String, String> {
        let seed = hex::encode(self.seed.as_slice());
        Ok(seed)
    }

    /// Returns the seed as bytes
    fn seed_bytes(&self) -> Result<&[u8], String> {
        Ok(&self.seed.as_slice())
    }
}
