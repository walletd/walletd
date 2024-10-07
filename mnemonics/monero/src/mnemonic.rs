use core::str;
use std::fmt;

use curve25519_dalek::scalar::Scalar;
use rand::{thread_rng, Rng};
use walletd_mnemonics_core::{MnemonicExt, MnemonicStyleBuilder, Seed};
use crate::mnemonic_type::BITS_IN_BYTES;
use crate::{Error, Language, MnemonicType, WordList};

/// The primary type in this crate, most tasks require creating or using one.
///
/// To create a *new* [`Mnemonic`][Mnemonic] from a randomly generated key, call
/// [`Mnemonic::new()`][Mnemonic::new()].
///
/// To get a [`Mnemonic`][Mnemonic] instance for an existing mnemonic phrase,
/// including those generated by other software or hardware wallets, use
/// [`Mnemonic::from_phrase()`][Mnemonic::from_phrase()].
///
/// Alternatively, the [`MnemonicBuilder`][MnemonicBuilder] struct can be used
/// to create a [`Mnemonic`][Mnemonic] struct by only specifying the options
/// needed, allowing for options to be rewritten. The
/// [`MnemonicBuilder`][MnemonicBuilder] struct with default options can be
/// created using the [`Mnemonic::builder()`][Mnemonic::builder()] function.
///
/// You can get the HD wallet [`Seed`][Seed] from a [`Mnemonic`][Mnemonic] by
/// calling [`Seed::new()`][Seed::new()]. From there you can either get the raw
/// byte value with [`Seed::as_bytes()`][Seed::as_bytes()], or the hex
/// representation using Rust formatting: `format!("{:X}", seed)`.
///
/// You can also get the original entropy value back from a
/// [`Mnemonic`][Mnemonic] with [`Mnemonic::entropy()`][Mnemonic::entropy()],
///
/// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
/// [Mnemonic::new()]: ./mnemonic/struct.Mnemonic.html#method.new
/// [Mnemonic::from_phrase()]: ./mnemonic/struct.Mnemonic.html#method.from_phrase
/// [Mnemonic::entropy()]: ./mnemonic/struct.Mnemonic.html#method.entropy
/// [Seed]: ./seed/struct.Seed.html
/// [Seed::new()]: ./seed/struct.Seed.html#method.new
/// [Seed::as_bytes()]: ./seed/struct.Seed.html#method.as_bytes
/// [MnemonicBuilder]: ./mnemonic/struct.MnemonicBuilder.html
/// [Mnemonic::builder()]: ./mnemonic/struct.Mnemonic.html#method.builder
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Mnemonic {
    phrase: String,
    lang: Language,
    seed: Seed,
    mnemonic_type: MnemonicType,
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mnemonic Phrase: {}", self.phrase)?;
        writeln!(f, "Language: {}", self.lang)?;
        writeln!(f, "Seed: {}", self.seed)?;
        writeln!(f, "Mnemonic Type: {}", self.mnemonic_type)?;
        Ok(())
    }
}

/// The builder pattern allows for the
/// creation of a [`Mnemonic`][Mnemonic] struct by only specifying the options
/// needed, allowing for options to be rewritten.
///
/// The [`MnemonicBuilder`][MnemonicBuilder] struct with default options can be
/// created using the [`Mnemonic::builder()`][Mnemonic::builder()] function or
/// [`MnemonicBuilder::new()`][MnemonicBuilder::new()].  The default trait is
/// implemented for MnemonicBuilder with the language set to English, the
/// mnemonic type set to 25 words, and the other fields not set.
///
/// The [`MnemonicBuilder::generate()`][MnemonicBuilder::generate()] function
/// will create a new mnemonic and return a [`Mnemonic`][Mnemonic] struct.
/// The [`MnemonicBuilder::restore()`][MnemonicBuilder::restore()] function will
/// restore a mnemonic from a mnemonic phrase or seed and return a
/// [`Mnemonic`][Mnemonic] struct. The various set methods can be used to set
/// the options for the mnemonic.
/// The [`MnemonicBuilder::detect_language()`][MnemonicBuilder::detect_language()] function can be used to specify that the language should be automatically detected from the mnemonic phrase.
///
/// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
/// [MnemonicBuilder]: ./mnemonic/struct.MnemonicBuilder.html
/// [Mnemonic::builder()]: ./mnemonic/struct.Mnemonic.html#method.builder
/// [MnemonicBuilder::restore()]: ./mnemonic/struct.MnemonicBuilder.html#method.restore
/// [MnemonicBuilder::generate()]: ./mnemonic/struct.MnemonicBuilder.html#method.generate
/// [MnemonicBuilder::new()]: ./mnemonic/struct.MnemonicBuilder.html#method.new
/// [MnemonicBuilder::detect_language()]: ./mnemonic/struct.MnemonicBuilder.html#method.detect_language
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MnemonicBuilder {
    pub mnemonic_phrase: Option<String>,
    pub language: Option<Language>,
    pub seed: Option<Seed>,
    pub mnemonic_type: Option<MnemonicType>,
    pub passphrase: Option<String>,
}

impl Default for MnemonicBuilder {
    fn default() -> Self {
        Self {
            mnemonic_phrase: None,
            language: Some(Language::default()),
            seed: None,
            mnemonic_type: Some(MnemonicType::default()),
            passphrase: None,
        }
    }
}

impl MnemonicExt for Mnemonic {
    type ErrorType = Error;
    type LanguageExt = Language;
    type MnemonicStyle = Self;
    type MnemonicStyleBuilder = MnemonicBuilder;
    type MnemonicTypeSpec = MnemonicType;

    /// Constructs the mnemonic struct from the specified phrase, language, and
    /// optional passphrase Returns custom errors if unable to recover the
    /// mnemonic struct from the phrase
    fn from_phrase(
        language: Language,
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self, Error> {
        let mnemonic_type = MnemonicType::from_phrase(phrase)?;
        let seed = Mnemonic::create_seed(language, phrase, mnemonic_type, specified_passphrase)?;

        Ok(Mnemonic {
            phrase: phrase.into(),
            lang: language,
            seed,
            mnemonic_type,
        })
    }

    /// Returns the builder for the mnemonic style with default values
    /// The default language is English, the default mnemonic type for the
    /// monero mnemonic is 25 words The default passphrase is None and no
    /// phrase is specified by default
    fn builder() -> Self::MnemonicStyleBuilder {
        Self::MnemonicStyleBuilder::default()
    }

    /// Generates a new mnemonic given the language, length of mnemonic, and
    /// optional passphrase
    fn new(
        language: Self::LanguageExt,
        mnemonic_type: Self::MnemonicTypeSpec,
        passphrase: Option<&str>,
    ) -> Self {
        let wordlist = WordList::new(language);

        const DEFAULT_LENGTH: usize = 32;
        let random_bytes: [u8; DEFAULT_LENGTH] =
            Scalar::from_bytes_mod_order(thread_rng().gen()).to_bytes();

        let bytes_length = mnemonic_type.entropy_bits() / BITS_IN_BYTES;
        let entropy_bytes = random_bytes[..bytes_length].to_vec();

        let mnemonic_phrase = Self::bytes_to_words(&entropy_bytes, &wordlist)
            .expect("entropy bytes should have a length greater than 0 and divisible by 4");

        let seed = Self::create_seed(language, &mnemonic_phrase, mnemonic_type, passphrase)
            .expect("Failed to generate seed");

        Mnemonic {
            phrase: mnemonic_phrase,
            lang: language,
            seed,
            mnemonic_type,
        }
    }

    /// Restores a mnemonic from a mnemonic phrase and optional passphrase,
    /// automatically detects the language
    fn detect_language(
        phrase: &str,
        specified_passphrase: Option<&str>,
    ) -> Result<Self::MnemonicStyle, Self::ErrorType> {
        let mnemonic_type = MnemonicType::from_phrase(phrase)?;
        let language = WordList::detect_language(phrase.split(' ').collect())?;
        let seed = Mnemonic::create_seed(language, phrase, mnemonic_type, specified_passphrase)?;

        Ok(Mnemonic {
            phrase: phrase.into(),
            lang: language,
            seed,
            mnemonic_type,
        })
    }

    /// Returns the ['Seed'][Seed] associated with the mnemonic phrase
    /// [Seed]: ./seed/struct.Seed.html
    fn to_seed(&self) -> Seed {
        self.seed.clone()
    }

    // Returns the language for the mnemonic
    fn language(&self) -> Self::LanguageExt {
        self.lang
    }

    // Returns the mnemonic phrase
    fn phrase(&self) -> String {
        self.phrase.clone()
    }

    // Returns the mnemonic type
    fn mnemonic_type(&self) -> Self::MnemonicTypeSpec {
        self.mnemonic_type
    }
}

impl MnemonicStyleBuilder for MnemonicBuilder {
    type ErrorType = Error;
    type LanguageExt = Language;
    type MnemonicStyle = Mnemonic;
    type MnemonicTypeSpec = MnemonicType;

    fn new() -> MnemonicBuilder {
        Self::default()
    }

    fn set_seed(&mut self, seed: &Seed) -> Self {
        self.seed = Some(seed.to_owned());
        self.clone()
    }

    fn set_phrase(&mut self, mnemonic_phrase: &str) -> Self {
        self.mnemonic_phrase = Some(mnemonic_phrase.to_owned());
        self.clone()
    }

    fn set_language(&mut self, language: Self::LanguageExt) -> Self {
        self.language = Some(language);
        self.clone()
    }

    fn set_passphrase(&mut self, passphrase: &str) -> Self {
        self.passphrase = Some(passphrase.to_owned());
        self.clone()
    }

    fn set_mnemonic_type(&mut self, mnemonic_type: Self::MnemonicTypeSpec) -> Self {
        self.mnemonic_type = Some(mnemonic_type);
        self.clone()
    }

    fn detect_language(&mut self) -> Self {
        self.language = None;
        self.clone()
    }

    /// Restore a mnemonic struct from a specified phrase or seed that had been
    /// generated before. It's recommended to restore a mnemonic struct from
    /// the phrase rather than a seed. Other optional parameters can be
    /// specified to ensure that the recovered mnemonic matches the
    /// specifications. This includes the language, mnemonic type, and
    /// passphrase. If a phrase is provided, the language and mnemonic type
    /// will be derived from the phrase, but an error will be reported if
    /// the language or mnemonic type specified do not match the language or
    /// mnemonic type derived from the phrase. If a passphrase is provided
    /// along with the mnemonic phrase, the seed will be derived from the
    /// mnemonic phrase and encrypted using the passphrase. If both a
    /// mnemonic phrase and a seed are provided, the specified seed will be
    /// ignored and only the mnemonic phrase will be used to construct the
    /// Mnemonic struct with no checks in place to compare with the
    /// specified seed. If a seed is directly provided without a phrase
    /// being specified, the mnemonic will we recovered using the seed info
    /// and the specifications for the language and mnemonic type, there are
    /// defaults in place for the language and mnemonic type. It is assumed
    /// that the seed provided is the "unencrypted" seed. That means that if
    /// a seed is provided and a passphrase is also provided, the final seed
    /// stored to the mnemonic struct will be the "encrypted" seed with the
    /// mnemonic phrase stemming directly from the "unencrypted" seed. The
    /// seed stored to the Mnemonic struct will be the final one which is
    /// used to derive the private key. If a seed is provided without a
    /// passphrase, the seed will be stored as the final seed and a mnemonic
    /// phrase will be derived based on the options that were specified or
    /// are default for the language and mnemonic type.
    fn restore(&self) -> Result<Self::MnemonicStyle, Error> {
        // early return of an error if neither the passphrase nor seed were provided
        if self.mnemonic_phrase.is_none() && self.seed.is_none() {
            return Err(Error::MissingInformation(
                "phrase or seed must be provided to recover a mnemonic".to_owned(),
            ));
        }

        let specified_passphrase = self.passphrase.as_deref();

        if let Some(phrase) = self.mnemonic_phrase.clone() {
            let mnemonic_type = MnemonicType::from_phrase(&phrase)?;
            if let Some(specified_mnemonic_type) = self.mnemonic_type {
                if mnemonic_type != specified_mnemonic_type {
                    return Err(Error::MismatchInSpecificationVersusImplict {
                        attribute: "mnemonic_type".to_string(),
                        spec: specified_mnemonic_type.to_string(),
                        implict: mnemonic_type.to_string(),
                    });
                }
            }

            if let Some(language) = self.language {
                let seed =
                    Mnemonic::create_seed(language, &phrase, mnemonic_type, specified_passphrase)?;

                Ok(Mnemonic {
                    phrase,
                    lang: language,
                    seed,
                    mnemonic_type,
                })
            } else {
                // language was not specified
                let phrase_words: Vec<&str> = phrase.split_whitespace().collect();
                let language = WordList::detect_language(phrase_words)?;
                let seed =
                    Mnemonic::create_seed(language, &phrase, mnemonic_type, specified_passphrase)?;

                Ok(Mnemonic {
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
                    let bytes_length = mnemonic_type.entropy_bits() / BITS_IN_BYTES;
                    let phrase = Mnemonic::bytes_to_words(
                        &specified_seed.as_bytes()[0..bytes_length].to_vec(),
                        &wordlist_info,
                    )?;
                    // Final seed will be encypted if a passphrase is provided
                    let seed = Mnemonic::create_seed(
                        language,
                        &phrase,
                        mnemonic_type,
                        specified_passphrase,
                    )?;

                    Ok(Mnemonic {
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

    /// Generates a new mnemonic struct using the specifications provided.
    fn generate(&self) -> Result<Self::MnemonicStyle, Error> {
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

        Ok(Mnemonic::new(
            language,
            mnemonic_type,
            self.passphrase.as_deref(),
        ))
    }
}

impl Mnemonic {
    /// Finalizes the seed by encrypting the initial seed with the passphrase
    /// Currently this function does not actually change the input bytes so is
    /// nonfunctioning We plan to fix this in issue #25 by implementing a
    /// hash that matches the one currently in Monero's codebase
    fn encrypt_seed(entropy_bytes: [u8; 32], _passphrase: &str) -> [u8; 32] {
        // TODO(#25) encrypt the seed with the cn_slow_hash function
        // Currently this hash does not match the one in Monero's codebase
        // Plan to fix this hash in issue #25 to use cn_slow_hash so that it will
        // reproduce Monero's implementation of seed offset phrase
        // let mut hasher = sha3::Keccak256Full::new();
        // let mut hash = [0u8; 32];
        // hasher.update(passphrase.as_bytes());
        // hash.copy_from_slice(&hasher.finalize().as_slice()[0..32]);
        // let encrypted_seed = Scalar::from_bytes_mod_order(entropy_bytes)
        //    + Scalar::from_bytes_mod_order(hash);
        // let encrypted_bytes = encrypted_seed.to_bytes();
        // encrypted_bytes
        entropy_bytes
    }

    /// Creates a mnemonic seed from a phrase, with language and mnemonic type
    /// specified. If a passphrase is provided the final seed will be
    /// encrypted with the passphrase with the initial seed stemming directly
    /// from the mnemonic phrase. If no passphrase is provided the final
    /// seed will be the initial seed stemming directly from the mnemonic
    /// phrase.
    fn create_seed(
        language: Language,
        mnemonic_phrase: &str,
        mnemonic_type: MnemonicType,
        provided_passphrase: Option<&str>,
    ) -> Result<Seed, Error> {
        // check if mnemonic phrase is valid first
        let mut entropy_bytes = [0u8; 32];
        entropy_bytes.copy_from_slice(
            Self::words_to_bytes(language, mnemonic_phrase, mnemonic_type)?.as_slice(),
        );

        match provided_passphrase {
            Some(passphrase) => {
                let encrypted_bytes = Self::encrypt_seed(entropy_bytes, passphrase);
                Ok(Seed::new(encrypted_bytes.to_vec()))
            }

            None => Ok(Seed::new(entropy_bytes.to_vec())),
        }
    }

    /// Directly converts bytes to words following the algorithm in Monero's
    /// codebase Assumes that the bytes given as an argument correspond to a
    /// valid mnemonic type length
    fn bytes_to_words(entropy_bytes: &Vec<u8>, wordlist_info: &WordList) -> Result<String, Error> {
        let wordlist = &wordlist_info.inner();
        if entropy_bytes.len() % 4 != 0 || entropy_bytes.is_empty() {
            return Err(Error::ErrorInBytes(
                "Length of secret_bytes must be greater than 0 and divisible by 4".into(),
            ));
        }

        let list_len = wordlist.len() as u32;
        // Going to map 4 bytes to 3 words
        // First, each chunk of 4 bytes gets converted to a u32 number, using little
        // endian representation
        let inputs = entropy_bytes
            .chunks(4)
            .map(|chunk| {
                let mut input: [u8; 4] = [0u8; 4];
                input.copy_from_slice(chunk);

                u32::from_le_bytes(input)
            })
            .collect::<Vec<u32>>();

        // Next, three words are generated from each of the 4 byte chunks, using the u32
        // numbers generated Indices are calculated to represent each word in
        // the mnemonic phrase in reference to the wordlist
        let mut phrase: Vec<&str> = vec![];
        for index in inputs {
            let w1 = index % list_len;
            let w2 = ((index / list_len) + w1) % list_len;
            let w3 = (((index / list_len) / list_len) + w2) % list_len;

            phrase.push(wordlist.get(w1 as usize).unwrap());
            phrase.push(wordlist.get(w2 as usize).unwrap());
            phrase.push(wordlist.get(w3 as usize).unwrap());
        }

        // The last word added to the mnemonic is a checksum word, it will repeat one of
        // the previous words in the phrase
        let checksum_word = wordlist_info.checksum_word(&phrase);
        phrase.push(&checksum_word);
        Ok(phrase.join(" "))
    }

    /// Directly converts words to bytes following the algorithm in Monero's
    /// codebase, using the wordlist specified by the language
    fn words_to_bytes(
        language: Language,
        mnemonic_phrase: &str,
        mnemonic_type: MnemonicType,
    ) -> Result<Vec<u8>, Error> {
        let wordlist = WordList::new(language);
        let mut phrase: Vec<&str> = mnemonic_phrase.split(' ').collect();
        let prefix_len = wordlist.prefix_length();
        let list_len = wordlist.inner().len();

        // The last word in the phrase is the checksum word
        let checksum = phrase.pop().ok_or(Error::ErrorInWords(
            "The mnemonic phrase is missing words".into(),
        ))?;
        // Decode the phrase three words at a time, three words equals 4 bytes or one
        // u32 number
        let mut buffer = vec![];
        for chunk in phrase.chunks(3) {
            let w1 = wordlist.trimmed_word_index(chunk[0])?;
            let w2 = wordlist.trimmed_word_index(chunk[1])?;
            let w3 = wordlist.trimmed_word_index(chunk[2])?;
            let n = list_len;
            let x = w1 + n * (((n - w1) + w2) % n) + n * n * (((n - w2) + w3) % n);

            if x % n != w1 {
                return Err(Error::ErrorInWords(format!(
                    "Invalid mnemonic phrase {}, cannot be decoded",
                    mnemonic_phrase
                )));
            }
            buffer.extend_from_slice(&u32::to_le_bytes(x as u32));
        }
        // Verify the checksum
        let expected_checksum = wordlist.checksum_word(&phrase);
        let expected = WordList::to_trimmed(&expected_checksum, prefix_len);
        let found = WordList::to_trimmed(checksum, prefix_len);
        if expected != found {
            return Err(Error::ErrorInWords(format!(
                "The mnemonic phrase has an invalid checksum word, expected {}, found {}",
                expected, found
            )));
        }

        match mnemonic_type {
            MnemonicType::Words25 => {
                let mut secret_bytes = [0u8; 32];
                secret_bytes.copy_from_slice(&buffer);
                Ok(secret_bytes.to_vec())
            }
            MnemonicType::Words13 => {
                let mut secret_bytes = [0u8; 32];
                let mut buffer_twice = vec![];
                buffer_twice.extend_from_slice(&buffer);
                buffer_twice.extend_from_slice(&buffer);
                secret_bytes.copy_from_slice(&buffer_twice);
                Ok(secret_bytes.to_vec())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
        let mnemonic = Mnemonic::builder().set_phrase(&phrase).restore().unwrap();
        assert_eq!(mnemonic.phrase(), phrase);
        assert_eq!(mnemonic.language(), Language::English);
        assert_eq!(mnemonic.mnemonic_type(), MnemonicType::Words25);
    }
}

#[test]
fn test_new_13_word() {
    let mnemonic = Mnemonic::builder()
        .set_language(Language::English)
        .set_mnemonic_type(MnemonicType::Words13)
        .generate()
        .unwrap();
    assert_eq!(mnemonic.language(), Language::English);
    let phrase: Vec<&str> = mnemonic.phrase.split(' ').collect();
    let word_count = phrase.len();
    assert_eq!(word_count, 13);
}

#[test]
fn test_new_25_word() {
    let mnemonic = Mnemonic::builder()
        .set_language(Language::English)
        .set_mnemonic_type(MnemonicType::Words25)
        .generate()
        .unwrap();
    assert_eq!(mnemonic.lang, Language::English);
    let phrase: Vec<&str> = mnemonic.phrase.split(' ').collect();
    let word_count = phrase.len();
    assert_eq!(word_count, 25);
}

#[test]
fn test_new_13_word_japanese() {
    let mnemonic = Mnemonic::builder()
        .set_language(Language::Japanese)
        .set_mnemonic_type(MnemonicType::Words13)
        .generate()
        .unwrap();
    assert_eq!(mnemonic.lang, Language::Japanese);
    let phrase: Vec<&str> = mnemonic.phrase.split(' ').collect();
    let word_count = phrase.len();
    assert_eq!(word_count, 13);
}

#[test]
fn test_from_phrase() {
    let phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
    let mnemonic = Mnemonic::builder().set_phrase(phrase).restore().unwrap();
    assert_eq!(mnemonic.phrase, phrase);
    assert_eq!(mnemonic.lang, Language::English);
    assert_eq!(
        mnemonic.seed,
        Seed::new(vec![
            52, 15, 191, 192, 185, 227, 59, 13, 88, 153, 143, 44, 203, 95, 52, 17, 251, 170, 199,
            194, 208, 244, 26, 178, 68, 62, 184, 63, 155, 243, 5, 14
        ])
    );
    assert_eq!(
        mnemonic.to_seed().to_string(),
        "340fbfc0b9e33b0d58998f2ccb5f3411fbaac7c2d0f41ab2443eb83f9bf3050e"
    );
}

#[test]
fn test_detect_language() {
    let phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
    let mut mnemonic_builder = Mnemonic::builder();
    // Overriding the defaults for language and mnemonic type
    mnemonic_builder.language = None;
    mnemonic_builder.mnemonic_type = None;
    // This checks the detect language capability because the language was not
    // specified in the builder, it is detected from the specified phrase
    let mnemonic = mnemonic_builder.set_phrase(phrase).restore().unwrap();

    assert_eq!(mnemonic.phrase, phrase);
    assert_eq!(mnemonic.lang, Language::English);
    assert_eq!(
        mnemonic.seed,
        Seed::new(vec![
            52, 15, 191, 192, 185, 227, 59, 13, 88, 153, 143, 44, 203, 95, 52, 17, 251, 170, 199,
            194, 208, 244, 26, 178, 68, 62, 184, 63, 155, 243, 5, 14
        ])
    );
    assert_eq!(
        mnemonic.to_seed().to_string(),
        "340fbfc0b9e33b0d58998f2ccb5f3411fbaac7c2d0f41ab2443eb83f9bf3050e"
    );
}

#[test]
fn test_from_phrase_invalid_length() {
    let phrase: &str = "buzzer eject zeal algebra adept arrow shipped
mobile reorder light plus rover fawns fight aphid powder tufts niche plotting
acumen equip civilian camp dialect";
    assert!(Mnemonic::builder().set_phrase(phrase).restore().is_err());
}

#[test]
fn test_from_phrase_invalid_word() {
    let phrase: &str = "buzzer eject zeal algebra adept arrow shipped
mobile reorder light plus rover fawns fight aphid powder tufts niche plotting
acumen equip civilian camp invalid algebra";
    assert!(Mnemonic::builder().set_phrase(phrase).restore().is_err());
}

#[test]
fn test_from_phrase_empty_phrase() {
    let phrase: &str = "";
    assert!(Mnemonic::builder().set_phrase(phrase).restore().is_err());
}

#[test]
fn test_error_conflicting_language_option() {
    let phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
    let mnemonic = Mnemonic::builder()
        .set_language(Language::French)
        .set_phrase(phrase)
        .restore();
    assert!(mnemonic.is_err());
    assert!(matches!(mnemonic.unwrap_err(), Error::InvalidWord(_, _)));
}

#[test]
fn test_error_conflicting_mnemonic_type() {
    let phrase: &str = "buzzer eject zeal algebra adept arrow shipped mobile reorder light plus rover fawns fight aphid powder tufts niche plotting acumen equip civilian camp dialect algebra";
    let mnemonic = Mnemonic::builder()
        .set_mnemonic_type(MnemonicType::Words13)
        .set_phrase(phrase)
        .restore();
    assert!(mnemonic.is_err());
    assert_eq!(
        mnemonic.unwrap_err(),
        Error::MismatchInSpecificationVersusImplict {
            attribute: "mnemonic_type".to_string(),
            spec: MnemonicType::Words13.to_string(),
            implict: MnemonicType::Words25.to_string()
        }
    );
}

// TODO(#25): Uncomment this test when we have the cn_slow_hash function done
// #[test]
// fn test_with_seed_offset_passphrase() {
//     let mnemonic_phrase: &str = "buzzer eject zeal algebra adept arrow
// shipped mobile reorder light plus rover fawns fight aphid powder tufts
// niche plotting acumen equip civilian camp dialect algebra";
//     let provided_passphrase: &str = "example passphrase";
//     let mnemonic = Mnemonic::from_phrase(Language::English,
// mnemonic_phrase, Some(provided_passphrase)).unwrap();
//     let actual_seed_hex = mnemonic.to_seed().to_string();
//     let expected_encrypted_seed_hex =
// "294d3c091d80e36cf2a551ed3ee36c694dda232d8e6b9ea5de8f7442c01fa50d";
//     assert_eq!(actual_seed_hex, expected_encrypted_seed_hex);

// }
//}
