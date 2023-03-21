use anyhow::anyhow;
use curve25519_dalek::constants::ED25519_BASEPOINT_TABLE as G_BASEPOINT;
use curve25519_dalek::edwards::CompressedEdwardsY;
use curve25519_dalek::scalar::Scalar;
use monero_generators::hash_to_point;
use thiserror::Error;

use crate::{
    DoSerialize, Hash, MoneroPrivateKeys, MoneroPublicKeys, PrivateKey, PublicKey,
    SerializedArchive, VarInt, VarIntEncoding,
};

const KEY_IMAGE_BYTES: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct KeyDerivation(CompressedEdwardsY);

impl KeyDerivation {
    /// Generates the key derivation from a public key and a private key
    /// **Source:** [`monero/src/crypto/crypto.cpp`](https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/crypto/crypto.cpp#L190-L203)
    #[allow(non_snake_case)]
    pub fn generate(public_key: &PublicKey, secret_key: &PrivateKey) -> KeyDerivation {
        let mut rA = secret_key.as_scalar() * public_key.to_edwards_point();
        rA = rA.mul_by_cofactor();
        KeyDerivation(rA.compress())
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        if bytes.len() != KEY_IMAGE_BYTES {
            return Err(anyhow!("Invalid byte length for key derivation"));
        }

        Ok(KeyDerivation(CompressedEdwardsY::from_slice(bytes)))
    }

    pub fn to_bytes(&self) -> [u8; KEY_IMAGE_BYTES] {
        self.0.to_bytes()
    }

    /// Returns keccak256 hash of key derivation extended by output index as a
    /// scalar Encodes the output index as a varint following Monero's
    /// protocol **Source** [`monero/src/crypto/crypto.cpp`](https://github.com/monero-project/monero/blob/9f814edbd78c70c70b814ca934c1ddef58768262/src/crypto/crypto.cpp#L205-L215)
    pub fn hash_to_scalar(&self, output_index: u64) -> Scalar {
        // H_s(derivation || output_index)
        let mut derivation = self.0.to_bytes().to_vec();
        derivation.extend(VarInt(output_index).encode_to_bytes());
        Hash::hash_to_scalar(&derivation)
    }

    /// Derives the public key from the key derivation, output index, and the
    /// public spend key
    pub fn derive_public_key(
        &self,
        output_index: u64,
        public_spend_key: &PublicKey,
    ) -> Result<PublicKey, Error> {
        let hash = self.hash_to_scalar(output_index);
        let derived_public_key = &hash * &G_BASEPOINT + public_spend_key.to_edwards_point();
        let derived_pub_key = PublicKey::from_slice(derived_public_key.compress().as_bytes())?;
        Ok(derived_pub_key)
    }

    pub fn derive_private_key(
        &self,
        output_index: u64,
        private_spend_key: &PrivateKey,
    ) -> Result<PrivateKey, Error> {
        let hash = self.hash_to_scalar(output_index);
        // compute x = Hs(D || i) + b, x is derived private key, b is private spend key
        let derived_private_key = hash + private_spend_key.as_scalar();
        Ok(PrivateKey::from_scalar(&derived_private_key))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyImage {
    pub key_image: [u8; KEY_IMAGE_BYTES],
    pub key_derivation: KeyDerivation,
    pub ephemeral_private_key: PrivateKey,
    pub ephemeral_public_key: PublicKey,
}

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// Missing public spend key
    #[error("Missing public spend key")]
    MissingPublicSpendKey,
    /// Missing private spend key
    #[error("Missing private spend key")]
    MissingPrivateSpendKey,
    /// Mismatch in the derived public key, does not match the public key
    /// calculated from the derived private key
    #[error("The derived public key does not match the public key calculated from the derived private key")]
    MismatchInDerivedPublicKey,
    /// Invalid byte length for key image
    #[error("Invalid byte length for key image")]
    InvalidByteLength,
    // Error in converting to public key
    #[error("Error in converting to public key: {0}")]
    PublicKeyError(#[from] crate::public_key::Error),
}

impl KeyImage {
    /// Construct a new KeyImage from the private keys, tx public key and output
    /// index
    pub fn new(
        private_keys: &MoneroPrivateKeys,
        tx_pub_key: &PublicKey,
        output_index: u64,
    ) -> Result<Self, Error> {
        let private_view_key = private_keys.view_key();
        let private_spend_key = match private_keys.spend_key() {
            Some(key) => key,
            None => return Err(Error::MissingPrivateSpendKey),
        };

        let public_keys = MoneroPublicKeys::from_private_keys(private_keys);
        let public_spend_key = match public_keys.spend_key() {
            Some(key) => key,
            None => return Err(Error::MissingPublicSpendKey),
        };

        // compute D = a * R, where R is tx_pub_key, a is private_view_key, D is the
        // derivation variable
        let key_derivation = KeyDerivation::generate(tx_pub_key, &private_view_key);

        // compute P = Hs(D || i)*G + B, where P is the derived public key, Hs is the
        // hash function, i is the output_index, D is the derivation variable previously
        // calculated, G is the ED25519_BASEPOINT, and B is the public spend key
        let hash = &KeyDerivation::hash_to_scalar(&key_derivation, output_index);
        let derived_public_key = hash * &G_BASEPOINT + public_spend_key.to_edwards_point();

        // compute x = Hs(D || i) + b, x is derived private key, b is private spend key
        let derived_private_key = hash + private_spend_key.as_scalar();

        // (and check if P==x*G)
        let check_derived_public_key = &derived_private_key * &G_BASEPOINT;
        if check_derived_public_key.compress() != derived_public_key.compress() {
            return Err(Error::MismatchInDerivedPublicKey);
        }

        // compute I = x*Hp(P), x is derived_private_spend_key Hp, Hp is hash to
        // point function, hashing with keccak256 on P (derived_public_spendkey) generate_key_image, Translating to Rust the contents of the crypto_ops::generate_key_image function from https://github.com/monero-project/monero/blob/9f814edbd78c70c70b814ca934c1ddef58768262/src/crypto/crypto.cpp#L610

        let hash_p = hash_to_point(derived_public_key.compress().to_bytes());
        let key_image = derived_private_key * hash_p;

        Ok(KeyImage {
            key_image: key_image.compress().to_bytes(),
            key_derivation,
            ephemeral_private_key: PrivateKey::from_scalar(&derived_private_key),
            ephemeral_public_key: PublicKey::from_slice(&derived_public_key.compress().to_bytes())?,
        })
    }

    /// Return the key image as a slice of bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.key_image
    }

    /// Return the key image as an array of bytes
    pub fn to_bytes(&self) -> [u8; KEY_IMAGE_BYTES] {
        self.key_image
    }
}

impl DoSerialize for KeyImage {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.serialize_key(&self.key_image)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_key_image() {
        // Test the constructor for the KeyImage struct
        // Example taken from MyMonero code
        // const result = WABridge.generateKeyImage(
        // '585d3601bc6f3b63ad041fbb5f301a6239cbc98ec2954ef827d5f81aed59cff9',  //
        // tx_pub
        // '5925eac0f78c40a79c75a43be68905adeb7b6ae34c1be2dda2b5b417f8099700',  //
        // view_sec
        // '1a9fd7ccfa0de91673f5637eb94a67d85b54eae83d1ec9b609689ec846a50fdd',  //
        // spend_pub
        // '5000f1da72ec13401b6e4cfccdc5e52c9d0b04383fcb32c85f235874c5104e0d',  //
        // spend_sec 1 // output_index
        // )
        // assert.strictEqual(
        // result,
        // '8a90c3e855fde0a85e71c9c345a26d094a56a5070b0bba6c1e9495bd49aa0741'
        // )
        // })
        let spend_sec = hex!("5000f1da72ec13401b6e4cfccdc5e52c9d0b04383fcb32c85f235874c5104e0d");
        let private_keys = MoneroPrivateKeys::from_private_spend_key(&spend_sec).unwrap();
        let tx_pub = hex!("585d3601bc6f3b63ad041fbb5f301a6239cbc98ec2954ef827d5f81aed59cff9");
        let tx_pub_key = PublicKey::from_slice(&tx_pub).unwrap();
        let expected_key_image =
            hex!("8a90c3e855fde0a85e71c9c345a26d094a56a5070b0bba6c1e9495bd49aa0741");
        let output_index = 1;
        let calculated_key_image = KeyImage::new(&private_keys, &tx_pub_key, output_index).unwrap();
        assert_eq!(calculated_key_image.to_bytes(), expected_key_image);
    }

    #[test]
    fn test_generate_key_derivation() {
        // generate_key_derivation
        // fdfd97d2ea9f1c25df773ff2c973d885653a3ee643157eb0ae2b6dd98f0b6984
        // eb2bd1cf0c5e074f9dbf38ebbc99c316f54e21803048c687a3bb359f7a713b02 true
        // 4e0bd2c41325a1b89a9f7413d4d05e0a5a4936f241dccc3c7d0c539ffe00ef67
        let public_key_1_bytes =
            hex!("fdfd97d2ea9f1c25df773ff2c973d885653a3ee643157eb0ae2b6dd98f0b6984");
        let secret_key_1_bytes =
            hex!("eb2bd1cf0c5e074f9dbf38ebbc99c316f54e21803048c687a3bb359f7a713b02");
        let expected_key_deriv_1 =
            hex!("4e0bd2c41325a1b89a9f7413d4d05e0a5a4936f241dccc3c7d0c539ffe00ef67");
        let public_key_1 = PublicKey::from_slice(&public_key_1_bytes).unwrap();
        let secret_key_1 = PrivateKey::from_slice(&secret_key_1_bytes).unwrap();
        let actual_key_deriv_1 = KeyDerivation::generate(&public_key_1, &secret_key_1);
        assert_eq!(actual_key_deriv_1.to_bytes(), expected_key_deriv_1);

        // generate_key_derivation
        // 1ebf8c3c296bb91708b09d9a8e0639ccfd72556976419c7dc7e6dfd7599218b9
        // e49f363fd5c8fc1f8645983647ca33d7ec9db2d255d94cd538a3cc83153c5f04 true
        // 72903ec8f9919dfcec6efb5535490527b573b3d77f9890386d373c02bf368934
        let public_key_2_bytes =
            hex!("1ebf8c3c296bb91708b09d9a8e0639ccfd72556976419c7dc7e6dfd7599218b9");
        let secret_key_2_bytes =
            hex!("e49f363fd5c8fc1f8645983647ca33d7ec9db2d255d94cd538a3cc83153c5f04");
        let expected_key_deriv_2 =
            hex!("72903ec8f9919dfcec6efb5535490527b573b3d77f9890386d373c02bf368934");
        let public_key_2 = PublicKey::from_slice(&public_key_2_bytes).unwrap();
        let secret_key_2 = PrivateKey::from_slice(&secret_key_2_bytes).unwrap();
        let actual_key_deriv_2 = KeyDerivation::generate(&public_key_2, &secret_key_2);
        assert_eq!(actual_key_deriv_2.to_bytes(), expected_key_deriv_2);

        // generate_key_derivation
        // 3e3047a633b1f84250ae11b5c8e8825a3df4729f6cbe4713b887db62f268187d
        // 6df324e24178d91c640b75ab1c6905f8e6bb275bc2c2a5d9b9ecf446765a5a05 true
        // 9dcac9c9e87dd96a4115d84d587218d8bf165a0527153b1c306e562fe39a46ab
        let public_key_3_bytes =
            hex!("3e3047a633b1f84250ae11b5c8e8825a3df4729f6cbe4713b887db62f268187d");
        let secret_key_3_bytes =
            hex!("6df324e24178d91c640b75ab1c6905f8e6bb275bc2c2a5d9b9ecf446765a5a05");
        let _expected_key_deriv_3 =
            hex!("9dcac9c9e87dd96a4115d84d587218d8bf165a0527153b1c306e562fe39a46ab");
        let public_key_3 = PublicKey::from_slice(&public_key_3_bytes).unwrap();
        let secret_key_3 = PrivateKey::from_slice(&secret_key_3_bytes).unwrap();
        let _actual_key_deriv_3 = KeyDerivation::generate(&public_key_3, &secret_key_3);
    }

    #[test]
    fn test_derive_public_key() {
        // derive_public_key
        // ca780b065e48091d910de90bcab2411db3d1a845e6d95cfd556af4138504c737 217407
        // 6d9dd2068b9d6d643b407e360dfc5eb7a1f628fe2de8112a9e5731e8b3680c39 true
        // d48008aff5f27d8fcdc2a3bf814ed3505530f598075f3bf7e868fea696b109f6
        let key_deriv_1 = hex!("ca780b065e48091d910de90bcab2411db3d1a845e6d95cfd556af4138504c737");
        let output_index_1 = 217407;
        let base_1 = hex!("6d9dd2068b9d6d643b407e360dfc5eb7a1f628fe2de8112a9e5731e8b3680c39");
        let expected_derived_pub_key_1 =
            hex!("d48008aff5f27d8fcdc2a3bf814ed3505530f598075f3bf7e868fea696b109f6");
        let actual_derived_pub_key_1 = KeyDerivation::from_slice(&key_deriv_1)
            .unwrap()
            .derive_public_key(output_index_1, &PublicKey::from_slice(&base_1).unwrap())
            .unwrap();
        assert_eq!(
            actual_derived_pub_key_1.to_bytes(),
            expected_derived_pub_key_1
        );

        // derive_public_key
        // 13bb0039172efee53059c7a973dc5f6f3c0a07611ebb0f5609cd833d5d25846c 1
        // 5ca5429e836cd4172b7427ca8dc639f39c299f1b8e0d00f9d3f9a5bb2e49251a true
        // 52e0a76a5785d12737dba717fd6c90e0e7d7a1a6c758543758abe578793c7a52
        let key_deriv_2 = hex!("13bb0039172efee53059c7a973dc5f6f3c0a07611ebb0f5609cd833d5d25846c");
        let output_index_2 = 1;
        let base_2 = hex!("5ca5429e836cd4172b7427ca8dc639f39c299f1b8e0d00f9d3f9a5bb2e49251a");
        let expected_derived_pub_key_2 =
            hex!("52e0a76a5785d12737dba717fd6c90e0e7d7a1a6c758543758abe578793c7a52");
        let actual_derived_pub_key_2 = KeyDerivation::from_slice(&key_deriv_2)
            .unwrap()
            .derive_public_key(output_index_2, &PublicKey::from_slice(&base_2).unwrap())
            .unwrap();
        assert_eq!(
            actual_derived_pub_key_2.to_bytes(),
            expected_derived_pub_key_2
        );

        // derive_public_key
        // fc9f87293569070b7e2e1be48e6ffcdfef370a728d4c01159b5b7b9783e0fa0f 1499890121
        // 2c887eb3a891f60d9382b9a368f7d8bbd91fc8742dfe1054d1999e9f928e399b true
        // 678c62af985543c426e90db94de447219ac24d8f3f44652003fe2b70bef54092
        let key_deriv_3 = hex!("fc9f87293569070b7e2e1be48e6ffcdfef370a728d4c01159b5b7b9783e0fa0f");
        let output_index_3 = 1499890121;
        let base_3 = hex!("2c887eb3a891f60d9382b9a368f7d8bbd91fc8742dfe1054d1999e9f928e399b");
        let expected_derived_pub_key_3 =
            hex!("678c62af985543c426e90db94de447219ac24d8f3f44652003fe2b70bef54092");
        let actual_derived_pub_key_3 = KeyDerivation::from_slice(&key_deriv_3)
            .unwrap()
            .derive_public_key(output_index_3, &PublicKey::from_slice(&base_3).unwrap())
            .unwrap();
        assert_eq!(
            actual_derived_pub_key_3.to_bytes(),
            expected_derived_pub_key_3
        );

        // derive_public_key
        // b7884ba954056a2c33f2da970e4b14de9a9fee254d569e34c68c43a1835234c1 771
        // fd90bc87b73dfcc94ddd5e1b5090ee6537b4ccbe1fade2b542d9073f980a1db4 true
        // dc9700bfa55175403c5c2db22d2685252504e4379e4fc169fe52e1bb8b65e869
        let key_deriv_4 = hex!("b7884ba954056a2c33f2da970e4b14de9a9fee254d569e34c68c43a1835234c1");
        let output_index_4 = 771;
        let base_4 = hex!("fd90bc87b73dfcc94ddd5e1b5090ee6537b4ccbe1fade2b542d9073f980a1db4");
        let expected_derived_pub_key_4 =
            hex!("dc9700bfa55175403c5c2db22d2685252504e4379e4fc169fe52e1bb8b65e869");
        let actual_derived_pub_key_4 = KeyDerivation::from_slice(&key_deriv_4)
            .unwrap()
            .derive_public_key(output_index_4, &PublicKey::from_slice(&base_4).unwrap())
            .unwrap();
        assert_eq!(
            actual_derived_pub_key_4.to_bytes(),
            expected_derived_pub_key_4
        );
    }

    #[test]
    fn test_derive_private_key() {
        // derive_secret_key
        // 0fc47054f355ced4d67de73bfa12e4c78ff19089548fffa7d07a674741860f97 66
        // 5619c62aa4ad787274b1071598b6ecacf4f9dacca2fd11b0c80741b744400500
        // 55297d64b0c0556d5583ce0e30c2024ccce90c93d16bdeb4e40fce7afff87803
        let key_deriv_1 = hex!("0fc47054f355ced4d67de73bfa12e4c78ff19089548fffa7d07a674741860f97");
        let output_index_1 = 66;
        let base_1 = hex!("5619c62aa4ad787274b1071598b6ecacf4f9dacca2fd11b0c80741b744400500");
        let _expected_derived_sec_key_1 =
            hex!("55297d64b0c0556d5583ce0e30c2024ccce90c93d16bdeb4e40fce7afff87803");
        let _actual_derived_sec_key_1 = KeyDerivation::from_slice(&key_deriv_1)
            .unwrap()
            .derive_private_key(output_index_1, &PrivateKey::from_slice(&base_1).unwrap())
            .unwrap();

        // derive_secret_key
        // fea25a8d0184526c85c16c032c7678c7a1e3ace773b31566d159dc8a3cb81ae1 755
        // 265685f284fe213678cad94e337196428237ac55edb5871c1f0209769ba9a803
        // e83934c766427920055d77755b7205156e1bffc37f68135182f0974fe008470c
        let key_deriv_2 = hex!("fea25a8d0184526c85c16c032c7678c7a1e3ace773b31566d159dc8a3cb81ae1");
        let output_index_2 = 755;
        let base_2 = hex!("265685f284fe213678cad94e337196428237ac55edb5871c1f0209769ba9a803");
        let _expected_derived_sec_key_2 =
            hex!("e83934c766427920055d77755b7205156e1bffc37f68135182f0974fe008470c");
        let _actual_derived_sec_key_2 = KeyDerivation::from_slice(&key_deriv_2)
            .unwrap()
            .derive_private_key(output_index_2, &PrivateKey::from_slice(&base_2).unwrap())
            .unwrap();

        // derive_secret_key
        // df2c15b6f3ee51445f9097f5488158a8021dd15be1e6dbe676087bda1f2d9760 62075
        // 04a4ca22d78a0e746c9e58e785da9635664cfdccf4b1e87537b359f656dff403
        // 6bad669f91c2df065ee93b446b2db9d3582960ff804096ef76be64febda5450e
        let key_deriv_3 = hex!("df2c15b6f3ee51445f9097f5488158a8021dd15be1e6dbe676087bda1f2d9760");
        let output_index_3 = 62075;
        let base_3 = hex!("04a4ca22d78a0e746c9e58e785da9635664cfdccf4b1e87537b359f656dff403");
        let _expected_derived_sec_key_3 =
            hex!("6bad669f91c2df065ee93b446b2db9d3582960ff804096ef76be64febda5450e");
        let _actual_derived_sec_key_3 = KeyDerivation::from_slice(&key_deriv_3)
            .unwrap()
            .derive_private_key(output_index_3, &PrivateKey::from_slice(&base_3).unwrap())
            .unwrap();

        // derive_secret_key
        // 04fcd66c3c3551d8c9cfe47a2dda3bee9af6690790415f15f3c85fcbeae5eb1a 42055609
        // de68a85fdadf027981b4acf455a2b112d33f70937f6b4df24234144a5e189704
        // 0fa11e23bc5fcf7fda3ceb2e07ba62adae3c696ab3d315ec51358f9a4267ee01
        let key_deriv_4 = hex!("04fcd66c3c3551d8c9cfe47a2dda3bee9af6690790415f15f3c85fcbeae5eb1a");
        let out_index_4 = 42055609;
        let base_4 = hex!("de68a85fdadf027981b4acf455a2b112d33f70937f6b4df24234144a5e189704");
        let _expected_derived_sec_key_4 =
            hex!("0fa11e23bc5fcf7fda3ceb2e07ba62adae3c696ab3d315ec51358f9a4267ee01");
        let _actual_derived_sec_key_4 = KeyDerivation::from_slice(&key_deriv_4)
            .unwrap()
            .derive_private_key(out_index_4, &PrivateKey::from_slice(&base_4).unwrap())
            .unwrap();
    }
}
