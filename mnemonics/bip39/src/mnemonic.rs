use core::ops::Div;
use core::str;

use crate::mnemonic_type::ENTROPY_OFFSET;
use crate::{Bip39Language, Bip39MnemonicType, Error, WordList};
use bitvec::prelude::*;
use curve25519_dalek::scalar::Scalar;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256, Sha512};
use walletd_mnemonics_core::{Mnemonic, MnemonicBuilder, Seed};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Represents a mnemonic which follows the `BIP39 standard`(<https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki>).
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct Bip39Mnemonic {
    phrase: String,
    #[zeroize(skip)]
    lang: Bip39Language,
    seed: Seed,
    #[zeroize(skip)]
    mnemonic_type: Bip39MnemonicType,
}

/// Implements the builder pattern for
/// creating a [Bip39Mnemonic] struct.
///
/// The builder pattern allows for the
/// creation of a [Bip39Mnemonic] struct by only specifying the options
/// needed, allowing for options to be rewritten.
///
///
/// A [Bip39MnemonicBuilder] struct with default options can be
/// created using the [Bip39Mnemonic::builder()] or
/// [Bip39MnemonicBuilder::new()] methods.
///
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bip39MnemonicBuilder {
    mnemonic_phrase: Option<String>,
    language: Option<Bip39Language>,
    seed: Option<Seed>,
    mnemonic_type: Option<Bip39MnemonicType>,
    passphrase: Option<String>,
}

impl Default for Bip39MnemonicBuilder {
    /// Default trait is
    /// implemented for [`MnemonicBuilder`] with the language set to [`English`][Bip39Language::English], the
    /// [mnemonic type set to 12 words][] no seed , mnemonic_phrase, or passphrase specified.
    fn default() -> Self {
        Self {
            mnemonic_phrase: None,
            language: Some(Bip39Language::default()),
            seed: None,
            mnemonic_type: Some(Bip39MnemonicType::default()),
            passphrase: None,
        }
    }
}

impl Bip39Mnemonic {
    fn create_seed(
        language: Bip39Language,
        mnemonic_phrase: &str,
        provided_passphrase: Option<&str>,
    ) -> Result<Seed, Error> {
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
        Ok(Seed::new(seed_bytes))
    }
}

impl Mnemonic for Bip39Mnemonic {
    type ErrorType = Error;
    type Language = Bip39Language;
    type Mnemonic = Self;
    type MnemonicBuilder = Bip39MnemonicBuilder;
    type MnemonicType = Bip39MnemonicType;

    /// Generates a new mnemonic given the [language of the mnemonic phrase][Bip39Language], [mnemonic type related to number of words in the mnemonic phrase][Bip39MnemonicType], and
    /// an optional passphrase.
    fn new(
        language: Bip39Language,
        mnemonic_type: Bip39MnemonicType,
        passphrase: Option<&str>,
    ) -> Self {
        let wordlist = WordList::new(language);

        const DEFAULT_LENGTH: usize = 32;
        let random_bytes: [u8; DEFAULT_LENGTH] =
            Scalar::from_bytes_mod_order(thread_rng().gen()).to_bytes();

        let bytes_length = mnemonic_type.entropy_bits() / ENTROPY_OFFSET;
        let entropy_bytes = random_bytes[..bytes_length].to_vec();

        let mnemonic_phrase = Self::bytes_to_words(&entropy_bytes, &wordlist)
            .expect("Failed to generate mnemonic phrase");
        let seed = Self::create_seed(language, &mnemonic_phrase, passphrase)
            .expect("Failed to generate seed");

        Self {
            phrase: mnemonic_phrase,
            lang: language,
            seed,
            mnemonic_type,
        }
    }

    /// Restores a mnemonic from a mnemonic phrase and optional passphrase,
    /// requires specifying the [language][Bip39Language].
    fn from_phrase(
        language: Bip39Language,
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self, Error> {
        let mnemonic_type = Bip39MnemonicType::from_phrase(phrase)?;
        let seed = Self::create_seed(language, phrase, specified_passphrase)?;

        Ok(Self {
            phrase: phrase.into(),
            lang: language,
            seed,
            mnemonic_type,
        })
    }

    /// Restores a mnemonic from a mnemonic phrase and optional passphrase,
    /// automatically detects the language of the mnemonic phrase.
    /// Returns an [error][Error] if the language of the mnemonic phrase cannot be detected or if the mnemonic phrase provided was not valid.
    fn detect_language(
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self::Mnemonic, Self::ErrorType> {
        let mnemonic_type = Bip39MnemonicType::from_phrase(phrase)?;
        let language = WordList::detect_language(phrase.split(' ').collect())?;
        let seed = Self::create_seed(language, phrase, specified_passphrase)?;

        Ok(Self {
            phrase: phrase.into(),
            lang: language,
            seed,
            mnemonic_type,
        })
    }

    /// Provides the [Seed] object.
    fn to_seed(&self) -> Seed {
        self.seed.clone()
    }

    /// Returns the [mnemonic][Bip39Mnemonic]'s phrase.
    fn phrase(&self) -> String {
        self.phrase.clone()
    }

    /// Returns the [language][Bip39Language] of the [mnemonic][Bip39Mnemonic].
    fn language(&self) -> Bip39Language {
        self.lang
    }

    /// Returns the [mnemonic type][Bip39MnemonicType] (an indicator of the number of words in the mnemonic phrase).
    fn mnemonic_type(&self) -> Bip39MnemonicType {
        self.mnemonic_type
    }

    fn builder() -> Self::MnemonicBuilder {
        Bip39MnemonicBuilder::default()
    }
}

impl MnemonicBuilder for Bip39MnemonicBuilder {
    type ErrorType = Error;
    type Language = Bip39Language;
    type Mnemonic = Bip39Mnemonic;
    type MnemonicType = Bip39MnemonicType;

    /// Creates a new [Bip39MnemonicBuilder] struct with default values.
    fn new() -> Self {
        Self::default()
    }

    fn mnemonic_seed(&mut self, seed: &Seed) -> &mut Self {
        self.seed = Some(seed.clone());
        self
    }

    fn mnemonic_phrase(&mut self, mnemonic_phrase: &str) -> &mut Self {
        self.mnemonic_phrase = Some(mnemonic_phrase.to_string());
        self
    }

    fn language(&mut self, language: Self::Language) -> &mut Self {
        self.language = Some(language);
        self
    }

    fn passphrase(&mut self, passphrase: &str) -> &mut Self {
        self.passphrase = Some(passphrase.to_string());
        self
    }

    fn mnemonic_type(&mut self, mnemonic_type: Self::MnemonicType) -> &mut Self {
        self.mnemonic_type = Some(mnemonic_type);
        self
    }

    /// Sets the [specified language][Self::Language] to None.
    ///
    /// Useful for overriding the default [English language][Bip39Language::English] assumption when using a mnemonic phrase in a non-English language without needing to specify which [language][Bip39Language] the mnemonic phrase is in.
    ///   
    /// When used with [mnemonic_phrase][MnemonicBuilder::mnemonic_phrase] and [build][MnemonicBuilder::build] or [restore][MnemonicBuilder::restore], automatically detects the language of the mnemonic phrase and returns an error if the mnemonic phrase is invalid for every [mnemonic][Self::Mnemonic] [language][Self::Language] wordlist.
    fn detect_language(&mut self) -> &mut Self {
        self.language = None;
        self
    }

    fn restore(&self) -> Result<Self::Mnemonic, Self::ErrorType> {
        // early return of an error if neither the passphrase nor seed were provided
        if self.mnemonic_phrase.is_none() && self.seed.is_none() {
            return Err(Error::MissingInformation(
                "phrase or seed must be provided to recover a mnemonic".to_owned(),
            ));
        }

        let specified_passphrase = self.passphrase.as_deref();

        if let Some(phrase) = self.mnemonic_phrase.clone() {
            let mnemonic_type = Bip39MnemonicType::from_phrase(&phrase)?;
            if let Some(specified_mnemonic_type) = self.mnemonic_type {
                if mnemonic_type != specified_mnemonic_type {
                    return Err(Error::MismatchInSpecificationVersusImplict {
                        attribute: "mnemonic_type".to_string(),
                    });
                }
            }

            if let Some(language) = self.language {
                let seed = Bip39Mnemonic::create_seed(language, &phrase, specified_passphrase)?;

                Ok(Bip39Mnemonic {
                    phrase,
                    lang: language,
                    seed,
                    mnemonic_type,
                })
            } else {
                // language was not specified
                let phrase_words: Vec<&str> = phrase.split_whitespace().collect();
                let language = WordList::detect_language(phrase_words)?;
                let seed = Bip39Mnemonic::create_seed(language, &phrase, specified_passphrase)?;

                Ok(Bip39Mnemonic {
                    phrase,
                    lang: language,
                    seed,
                    mnemonic_type,
                })
            }
        }
        // use seed to recover mnemonic if phrase was not provided
        else {
            assert!(self.seed.is_some()); // this should be true because of early return check above
            let specified_seed = self
                .seed
                .clone()
                .expect("seed should be present due to earlier checks");
            if let Some(language) = self.language {
                if let Some(mnemonic_type) = self.mnemonic_type {
                    // assuming that the seed provided directly corresponds to the mnemonic phrase
                    let wordlist_info = WordList::new(language);
                    let bytes_length = mnemonic_type.entropy_bits() / ENTROPY_OFFSET;
                    let phrase = Bip39Mnemonic::bytes_to_words(
                        &specified_seed.as_bytes()[0..bytes_length].to_vec(),
                        &wordlist_info,
                    )?;
                    // Final seed will be encypted if a passphrase is provided
                    let seed = Bip39Mnemonic::create_seed(language, &phrase, specified_passphrase)?;
                    Ok(Bip39Mnemonic {
                        phrase,
                        lang: language,
                        seed,
                        mnemonic_type,
                    })
                } else {
                    Err(Error::MissingInformation("To recover a mnemonic phrase from a seed, a mnemonic type must be specified as well as the language".to_owned()))
                }
            } else {
                Err(Error::MissingInformation("To recover a mnemonic phrase from a seed, a language must be specified as well as the mnemonic type".to_owned()))
            }
        }
    }

    /// Generate a new mnemonic which follows given specifications
    fn generate(&self) -> Result<Self::Mnemonic, Self::ErrorType> {
        if self.language.is_none() {
            return Err(Error::MissingInformation(
                "language must be specified to generate a mnemonic".to_owned(),
            ));
        }
        let language = self
            .language
            .expect("language should be present due to earlier checks");
        if self.mnemonic_type.is_none() {
            return Err(Error::MissingInformation(
                "mnemonic type must be specified to generate a mnemonic".to_owned(),
            ));
        }
        let mnemonic_type = self
            .mnemonic_type
            .expect("mnemonic type should be present due to earlier checks");

        Ok(Bip39Mnemonic::new(
            language,
            mnemonic_type,
            self.passphrase.as_deref(),
        ))
    }

    /// Build a mnemonic struct based on the specifications provided
    fn build(&self) -> Result<Self::Mnemonic, Self::ErrorType> {
        if self.mnemonic_phrase.is_some() {
            self.restore()
        } else {
            self.generate()
        }
    }
}

impl Bip39Mnemonic {
    /// Converting entropy bytes to the mnemonic words, given a wordlist
    fn bytes_to_words(entropy_bytes: &Vec<u8>, wordlist_info: &WordList) -> Result<String, Error> {
        if entropy_bytes.len() % 4 != 0 {
            return Err(Error::InvalidEntropy(
                "Entropy must be a multiple of 4 bytes (32 bits) in length".to_owned(),
            ));
        }
        if (entropy_bytes.len() < 128 / ENTROPY_OFFSET)
            || (entropy_bytes.len() > 256 / ENTROPY_OFFSET)
        {
            return Err(Error::InvalidEntropy(
                "Entropy must be between 128 and 256 bits in length".to_owned(),
            ));
        }

        // Take the sh256 hash of the entropy
        let mut sha256 = Sha256::new();
        sha256.input(entropy_bytes.as_slice());
        let hash = sha256.result();

        // number of words in mnemonic phrase depends on the number of bits in
        // entropy_bytes the number of bits in entropy_bytes (entropy_bytes * 8)
        // + checksum length (1 bit per 32 bits in entropy_bytes) equals the
        // total number of bits which will be a multiple of 33 one word will be
        // specified per 11 bits word_count = (entropy_bytes * 8) +
        // (entropy_bytes/32)/11
        let entropy_bits = entropy_bytes.len() * ENTROPY_OFFSET;
        let word_count = (entropy_bits + (entropy_bits / 32)) / 11;

        // We then take 1 bit of that hash for every 32 bits of entropy, and add it to
        // the end of our entropy.
        let hash_0 = BitVec::<Msb0, u8>::from_element(hash[0]);
        let (checksum, _) = hash_0.split_at(word_count.div(3));
        let mut encoding = BitVec::<Msb0, u8>::from_vec(entropy_bytes.clone());
        encoding.append(&mut checksum.to_vec());

        // Compute the phrase in 11 bit chunks which encode an index into the word list
        let wordlist = &wordlist_info.inner();

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
    fn words_to_bytes(language: Bip39Language, mnemonic_phrase: &str) -> Result<Vec<u8>, Error> {
        let wordlist = WordList::new(language);
        let phrase: Vec<&str> = mnemonic_phrase.split(' ').collect();
        let word_count = phrase.len();

        // Each word in the mnemonic phrase represents 11 bits
        // A checksum was added to the entropy with a length equal to the number of
        // entropy bits divided by 32 So, the number of original entropy bits
        // can be found: phrase.len() * 11 - (entropy_bits/32) = entropy_bits 32
        // * 11 * phrase.len() - entropy_bits = 32 * entropy_bits
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
        match *mnemonic_phrase == Self::bytes_to_words(&entropy_bytes, &wordlist)? {
            true => Ok(entropy_bytes),
            false => Err(Error::InvalidMnemonicPhraseChecksum),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
        let mnemonic = Bip39Mnemonic::builder()
            .mnemonic_phrase(phrase)
            .build()
            .unwrap();
        assert_eq!(mnemonic.phrase(), phrase);
        assert_eq!(mnemonic.language(), Bip39Language::English);
        assert_eq!(mnemonic.mnemonic_type(), Bip39MnemonicType::Words12);
    }

    #[test]
    fn test_new_12_word() {
        let mnemonic = Bip39Mnemonic::builder()
            .language(Bip39Language::English)
            .mnemonic_type(Bip39MnemonicType::Words12)
            .build()
            .unwrap();
        assert_eq!(mnemonic.lang, Bip39Language::English);
        let phrase: Vec<&str> = mnemonic.phrase.split(' ').collect();
        let word_count = phrase.len();
        assert_eq!(word_count, 12);
    }

    #[test]
    fn test_new_24_word() {
        let mnemonic = Bip39Mnemonic::builder()
            .language(Bip39Language::English)
            .mnemonic_type(Bip39MnemonicType::Words24)
            .build()
            .unwrap();
        assert_eq!(mnemonic.lang, Bip39Language::English);
        let phrase: Vec<&str> = mnemonic.phrase.split(' ').collect();
        let word_count = phrase.len();
        assert_eq!(word_count, 24);
    }

    #[test]
    fn test_new_12_word_japanese() {
        let mnemonic = Bip39Mnemonic::builder()
            .language(Bip39Language::Japanese)
            .mnemonic_type(Bip39MnemonicType::Words12)
            .build()
            .unwrap();
        assert_eq!(mnemonic.language(), Bip39Language::Japanese);
        let phrase: Vec<&str> = mnemonic.phrase.split(' ').collect();
        let word_count = phrase.len();
        assert_eq!(word_count, 12);
    }

    #[test]
    fn test_from_phrase() {
        let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
        let mnemonic = Bip39Mnemonic::builder()
            .mnemonic_phrase(phrase)
            .build()
            .unwrap();
        assert_eq!(mnemonic.phrase(), phrase);
        assert_eq!(mnemonic.language(), Bip39Language::English);
        assert_eq!(
            mnemonic.to_seed(),
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238
            ])
        );
        assert_eq!(mnemonic.to_seed().to_string(),
"a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee"
);
    }

    #[test]
    fn test_detect_language() {
        let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
        let mut mnemonic_builder = Bip39Mnemonic::builder();
        mnemonic_builder.language = None;
        mnemonic_builder.mnemonic_type = None;
        let mnemonic = mnemonic_builder.mnemonic_phrase(phrase).restore().unwrap();
        assert_eq!(mnemonic.phrase(), phrase);
        assert_eq!(mnemonic.language(), Bip39Language::English);
        assert_eq!(
            mnemonic.to_seed(),
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238
            ])
        );
        assert_eq!(mnemonic.to_seed().to_string(),
"a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee"
);
    }

    #[test]
    fn test_with_passphrase() {
        let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
        let passphrase = "mypassphrase";
        let mnemonic = Bip39Mnemonic::builder()
            .mnemonic_phrase(phrase)
            .passphrase(passphrase)
            .build()
            .unwrap();
        assert_eq!(mnemonic.phrase(), phrase);
        assert_eq!(mnemonic.language(), Bip39Language::English);
        assert_eq!(
            mnemonic.to_seed(),
            Seed::new(hex::decode("3c536b023d71d81e6abc58b0b91c64caff8bb08fabf0c9f3cf948a9f3a494e8ecb0790b6e933834796c930a2d437170bd6071c00bc0553d06235d02315f2c229").unwrap())
        );
    }

    #[test]
    fn test_from_phrase_invalid_length() {
        let phrase: &str = "outer ride neither foil glue number place usage ball shed dry";
        assert!(Bip39Mnemonic::builder()
            .mnemonic_phrase(phrase)
            .build()
            .is_err());
    }

    #[test]
    fn test_from_phrase_invalid_word() {
        let phrase: &str = "outer ride neither foil glue number place usage ball shed dry invalid";
        assert!(Bip39Mnemonic::builder()
            .mnemonic_phrase(phrase)
            .build()
            .is_err());
    }

    #[test]
    fn test_from_phrase_empty_phrase() {
        let phrase: &str = "";
        assert!(Bip39Mnemonic::builder()
            .mnemonic_phrase(phrase)
            .build()
            .is_err());
    }

    #[test]
    fn test_error_conflicting_language_option() {
        let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
        let mnemonic = Bip39Mnemonic::builder()
            .language(Bip39Language::French)
            .mnemonic_phrase(phrase)
            .build();
        assert!(mnemonic.is_err());
        assert!(matches!(mnemonic.unwrap_err(), Error::InvalidWord(_)));
    }

    #[test]
    fn test_error_conflicting_mnemonic_type() {
        let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
        let mnemonic = Bip39Mnemonic::builder()
            .mnemonic_type(Bip39MnemonicType::Words15)
            .mnemonic_phrase(phrase)
            .build();
        assert!(mnemonic.is_err());
        assert_eq!(
            mnemonic.unwrap_err(),
            Error::MismatchInSpecificationVersusImplict {
                attribute: "mnemonic_type".to_string(),
            }
        );
    }

    #[test]
    fn test_zeroize() -> Result<(), Error> {
        let phrase: &str = "outer ride neither foil glue number place usage ball shed dry point";
        let mut mnemonic = Bip39Mnemonic::builder().mnemonic_phrase(phrase).build()?;
        mnemonic.zeroize();
        assert_eq!(mnemonic.seed.as_bytes(), &[]);
        assert_eq!(mnemonic.phrase, "");
        Ok(())
    }
}
