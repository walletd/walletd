use anyhow::anyhow;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint, VartimeEdwardsPrecomputation};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::{Identity, VartimePrecomputedMultiscalarMul};
use dalek_ff_group::{EdwardsPoint as EdwPntGrp, Scalar as ScGrp};
use monero_generators::hash_to_point;
use multiexp::multiexp;
use rand::{thread_rng, Rng};
use thiserror::Error;
use zeroize::Zeroize;

use crate::generators_bulletproof_plus::GENERATORS;
use crate::transaction::CryptoHash;
use crate::varint::VarInt;
use crate::{hash, Hash, PrivateKey, PublicKey};

const BULLETPROOF_PLUS_MAX_OUTPUTS: usize = 16;
const BPP_N_BITS: usize = 64;
const BPP_MAX_MN: usize = BULLETPROOF_PLUS_MAX_OUTPUTS * BPP_N_BITS;

use crate::varint::VarIntEncoding;
use crate::{keccak256, DoSerialize, SerializedArchive};

const HASH_KEY_CLSAG_AGG_0: &[u8; 11] = b"CLSAG_agg_0";
const HASH_KEY_CLSAG_AGG_1: &[u8; 11] = b"CLSAG_agg_1";
const HASH_KEY_CLSAG_ROUND: &[u8; 11] = b"CLSAG_round";

/// Key Identity value
const KEY_I: RctKey = RctKey {
    bytes: [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
};

/// Key zero value
const KEY_ZERO: RctKey = RctKey {
    bytes: [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
};

/// H basepoint
const H_BASEPOINT: RctKey = RctKey {
    bytes: [
        0x8b, 0x65, 0x59, 0x70, 0x15, 0x37, 0x99, 0xaf, 0x2a, 0xea, 0xdc, 0x9f, 0xf1, 0xad, 0xd0,
        0xea, 0x6c, 0x72, 0x51, 0xd5, 0x41, 0x54, 0xcf, 0xa9, 0x2c, 0x17, 0x3a, 0x0d, 0xd3, 0x9c,
        0x1f, 0x94,
    ],
};

/// G basepoint
const G_BASEPOINT: RctKey = RctKey {
    bytes: [
        0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66,
    ],
};

const INV_EIGHT: RctKey = RctKey {
    bytes: [
        0x79, 0x2f, 0xdc, 0xe2, 0x29, 0xe5, 0x06, 0x61, 0xd0, 0xda, 0x1c, 0x7d, 0xb3, 0x9d, 0xd3,
        0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x06,
    ],
};

#[allow(dead_code)]
const MINUS_INV_EIGHT: RctKey = RctKey {
    bytes: [
        0x74, 0xa4, 0x19, 0x7a, 0xf0, 0x7d, 0x0b, 0xf7, 0x05, 0xc2, 0xda, 0x25, 0x2b, 0x5c, 0x0b,
        0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x0a,
    ],
};

const MINUS_ONE: RctKey = RctKey {
    bytes: [
        0xec, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde,
        0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x10,
    ],
};

const TWO: RctKey = RctKey {
    bytes: [
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ], // 2
};

const ONE: RctKey = RctKey {
    bytes: [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ], // 1
};

#[derive(Error, Debug)]
pub enum Error {
    // Display custom error message
    #[error("Error: {0}")]
    ErrorMessage(String),
    // Convert anyhow::Error to Error and display message
    #[error("Error(anyhow): {0}")]
    AnyhowError(#[from] anyhow::Error),
}

/// Implemented in Rust based on Monero's rct::key struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L79-L88)
#[derive(Debug, Clone, Default, PartialEq, Eq, Copy, Zeroize)]
pub struct RctKey {
    pub bytes: [u8; 32],
}

impl DoSerialize for RctKey {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.serialize_key(&self.bytes)?;
        Ok(())
    }
}

impl RctKey {
    /// Returns a key from a slice of bytes
    pub fn from_slice(data: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(data);
        Self { bytes }
    }

    /// Returns the identity key
    pub fn identity() -> Self {
        KEY_I
    }

    /// Returns the zero key
    pub fn zero() -> Self {
        KEY_ZERO
    }

    /// Returns the key representing the H basepoint
    pub fn h_basepoint() -> Self {
        H_BASEPOINT
    }

    /// Returns the key from a scalar
    pub fn from_scalar(scalar: &Scalar) -> Self {
        Self::from_slice(&scalar.to_bytes())
    }

    /// Converts a key to a scalar
    pub fn as_scalar(&self) -> Scalar {
        Scalar::from_bytes_mod_order(self.bytes)
    }

    /// Converts a key to an edwards point, panics if key cannot be converted to
    /// a valid edwards point This function should only be used with a key
    /// that can be converted to a valid edwards point
    pub fn as_point(&self) -> EdwardsPoint {
        if let Some(point) = CompressedEdwardsY::from_slice(&self.bytes).decompress() {
            point
        } else {
            panic!("Invalid edwards point for key: {:?}", self.bytes)
        }
    }

    /// Uses the custom hash_to_point function to convert a key to an edwards
    /// point
    pub fn hash_to_point(&self) -> EdwardsPoint {
        hash_to_point(self.bytes)
    }

    /// Converts to a key from an edwards point
    pub fn from_point(point: &EdwardsPoint) -> Self {
        Self::from_slice(&point.compress().to_bytes())
    }

    /// Generates a commitment point associated with a given amount and mask
    /// Returns C  = aG + bH, where a is the mask key, and b is derived from the
    /// amount
    #[allow(non_snake_case)]
    pub fn commit(amount: u64, mask: &RctKey) -> Self {
        let mut b_bytes = [0u8; 32];
        b_bytes[0..8].copy_from_slice(&amount.to_le_bytes());
        let b_scalar = Scalar::from_bytes_mod_order(b_bytes);
        let a_scalar = Scalar::from_bytes_mod_order(mask.bytes);
        let c_point = a_scalar * G_BASEPOINT.as_point() + b_scalar * H_BASEPOINT.as_point();
        RctKey::from_point(&c_point)
    }

    /// Generates a commitment mask key for a given input key
    pub fn gen_commitment_mask(sk: &RctKey) -> RctKey {
        let mut commitment_key = "commitment_mask".as_bytes().to_vec();
        commitment_key.extend(sk.bytes);
        let hash_scalar = Hash::hash_to_scalar(&commitment_key);
        RctKey::from_scalar(&hash_scalar)
    }

    pub fn zero_commit(amount: u64) -> Self {
        let mask = KEY_I;
        Self::commit(amount, &mask)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the Key data as a vector of bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}

/// Implemented in Rust based on Monero's rct::key64 type
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L145)
#[derive(Debug, Clone)]
pub struct Key64(pub [RctKey; 64]);

/// Implemented in Rust based on Monero's rct::ctkey struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L97-L100)
#[derive(Debug, Clone, Default, Zeroize)]
pub struct CtKey {
    pub dest: RctKey,
    pub mask: RctKey,
}

/// Implemented in Rust based on Monero's rct::boroSig struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L147-L151)
#[derive(Debug, Clone)]
pub struct BoroSig {
    pub s0: Key64,
    pub s1: Key64,
    pub ee: RctKey,
}

/// Implemented in Rust based on Monero's rct::ecdhTuple struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L127-140)
#[derive(Debug, Clone, Default)]
pub struct EcdhTuple {
    pub mask: RctKey,
    pub amount: RctKey,
}

impl EcdhTuple {
    pub fn ecdh_hash(k: &RctKey) -> RctKey {
        let mut data = [0u8; 38];
        let amount_str = "amount";
        data[0..amount_str.len()].copy_from_slice(amount_str.as_bytes());
        data[amount_str.len()..].copy_from_slice(&k.bytes);
        RctKey::from_slice(&keccak256(&data))
    }

    /// Encodes a EcdhTuple from a shared secret for v2 transactions
    pub fn encode(&mut self, shared_secret: &RctKey) {
        let mask = RctKey::zero();
        let mut amount = &mut self.amount;
        let hash_secret = Self::ecdh_hash(shared_secret);
        for i in 0..8 {
            amount.bytes[i] ^= hash_secret.bytes[i];
        }
        self.amount = *amount;
        self.mask = mask;
    }
}

/// Implemented in Rust based on Monero's rctSigBase struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L310-L318)
#[derive(Debug, Clone)]
pub struct RctSigBase {
    pub rct_type: RCTType,
    pub message: RctKey,
    pub mix_ring: Vec<Vec<CtKey>>,
    pub pseudo_outs: Vec<RctKey>,
    pub ecdh_info: Vec<EcdhTuple>,
    pub out_pk: Vec<CtKey>,
    pub txn_fee: u64,
}

impl DoSerialize for RctSigBase {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.serialize_field("type", &VarInt(self.rct_type.value()))?;
        let rct_type = RCTType::from_u8(self.rct_type.value())?;
        use RCTType::*;
        if rct_type != Full
            && rct_type != Simple
            && rct_type != Bulletproof
            && rct_type != Bulletproof2
            && rct_type != CLSAG
            && rct_type != BulletproofPlus
        {
            return Err(anyhow!("Unsupported rct type"));
        }
        serialized.serialize_field("txnFee", &VarInt(self.txn_fee))?;
        if rct_type == Simple {
            let inputs = self.pseudo_outs.len();
            serialized.add_tag("pseudoOuts");
            for (i, pseudo_out) in self.pseudo_outs.iter().enumerate() {
                serialized.serialize_directly(pseudo_out.as_bytes())?;
                if i < inputs - 1 {
                    serialized.delimit_array();
                }
            }
        }

        serialized.add_tag("ecdhInfo");
        let outputs = self.ecdh_info.len();
        serialized.begin_array();
        for i in 0..outputs {
            if rct_type == Bulletproof2 || rct_type == CLSAG || rct_type == BulletproofPlus {
                serialized.begin_object();
                let hashed_amount = hash::Hash8::new(&self.ecdh_info[i].amount.bytes);
                serialized.add_tag("amount");
                serialized.serialize_key(hashed_amount.as_bytes())?;
                serialized.end_object();
            } else {
                serialized.serialize_key(&self.ecdh_info[i].amount.bytes)?;
            }
            if i < outputs - 1 {
                serialized.delimit_array();
            }
        }
        serialized.end_array();

        serialized.add_tag("outPk");
        if self.out_pk.len() != outputs {
            return Err(anyhow!("Invalid outPk size"));
        }
        serialized.begin_array();
        for (i, out_pk) in self.out_pk.iter().enumerate() {
            serialized.serialize_key(out_pk.mask.as_bytes())?;
            if i < outputs - 1 {
                serialized.delimit_array();
            }
        }
        serialized.end_array();
        Ok(())
    }
}

impl RctSigBase {}

/// Implemented in Rust based on Monero's rangeSig struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L188-L202)
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct RangeSig {
    pub asig: BoroSig,
    pub Ci: Key64,
}

/// Implemented in Rust based on Monero's Bulletproof struct
/// ***Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L204-L210)
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Bulletproof {
    pub V: Vec<RctKey>,
    pub A: RctKey,
    pub S: RctKey,
    pub T1: RctKey,
    pub T2: RctKey,
    pub taux: RctKey,
    pub mu: RctKey,
    pub L: Vec<RctKey>,
    pub R: Vec<RctKey>,
    pub a: RctKey,
    pub b: RctKey,
    pub t: RctKey,
}

/// Adopted from the monero_generators crate
/// Container struct for Bulletproofs(+) generators.
#[allow(non_snake_case)]
pub struct Generators {
    pub G: [EdwardsPoint; BPP_MAX_MN],
    pub H: [EdwardsPoint; BPP_MAX_MN],
}

/// Adopted from the monero_generators crate
/// Generate generators as needed for Bulletproofs(+), as Monero does.
pub fn bulletproofs_generators(dst: &'static [u8]) -> Result<Generators, anyhow::Error> {
    let identity_point = CompressedEdwardsY::identity()
        .decompress()
        .expect("The identity should be valid Edwards Point");
    let mut res = Generators {
        G: [identity_point; BPP_MAX_MN],
        H: [identity_point; BPP_MAX_MN],
    };
    for i in 0..BPP_MAX_MN {
        let i = 2 * i;

        let mut even = monero_generators::H.compress().to_bytes().to_vec();
        even.extend(dst);
        let mut odd = even.clone();
        let i_0 = VarInt(i as u64);
        even.extend(i_0.encode_to_bytes());
        let i_1 = VarInt((i + 1) as u64);
        odd.extend(i_1.encode_to_bytes());

        res.H[i / 2] = hash_to_point(keccak256(&even));
        res.G[i / 2] = hash_to_point(keccak256(&odd));
    }
    Ok(res)
}

/// Implemented in Rust based on Monero's BulletproofPlus struct
/// ***Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L241-L271)
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct BulletproofPlus {
    pub V: Vec<RctKey>,
    pub A: RctKey,
    pub A1: RctKey,
    pub B: RctKey,
    pub r1: RctKey,
    pub s1: RctKey,
    pub d1: RctKey,
    pub L: Vec<RctKey>,
    pub R: Vec<RctKey>,
}

impl DoSerialize for BulletproofPlus {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.begin_object();
        serialized.serialize_field("A", &self.A)?;
        serialized.serialize_field("A1", &self.A1)?;
        serialized.serialize_field("B", &self.B)?;
        serialized.serialize_field("r1", &self.r1)?;
        serialized.serialize_field("s1", &self.s1)?;
        serialized.serialize_field("d1", &self.d1)?;
        serialized.serialize_vector("L", &self.L)?;
        serialized.serialize_vector("R", &self.R)?;
        serialized.end_object();
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Transcript(pub RctKey);
impl Transcript {
    pub fn update(&mut self, update: &[RctKey]) {
        let mut data = vec![self.0];
        data.extend(update);
        let data_bytes = &data.iter().flat_map(|x| x.bytes).collect::<Vec<u8>>();
        let hash_to_scalar = Hash::hash_to_scalar(data_bytes.as_slice());
        *self = Transcript(RctKey::from_scalar(&hash_to_scalar));
    }
}

impl BulletproofPlus {
    /// Used to construct a real bulletproof plus range proof for a transaction
    /// Implemented in Rust based on Monero's bulletproof_plus_PROVE function
    /// ***Source** <`monero/src/ringct/bulletproofs_plus.cc`>(https://github.com/monero-project/monero/blob/1a568deacbb3bbcd65fd092fcd4c20b14ffa6a26/src/ringct/bulletproofs_plus.cc#L512-L776)
    // Given a set of values v [0..2**N) and masks gamma, construct a range proof
    // This function calculates the sv variable values first with the sv values stemming from the
    // amounts argument and the masks argument represents the gamma variable.
    #[allow(non_snake_case)]
    pub fn new_proof(amounts: &Vec<u64>, masks: &[RctKey]) -> Result<Self, Error> {
        let mut sv: Vec<RctKey> = Vec::new();
        for amount in amounts {
            let mut sv_key = RctKey::zero();
            sv_key.bytes[0..8].copy_from_slice(&amount.to_le_bytes());
            sv.push(sv_key);
        }

        let gamma = masks;

        // Sanity check on inputs
        if sv.len() != gamma.len() {
            return Err(Error::ErrorMessage(
                "Incompatible sizes of sv and gamma".to_string(),
            ));
        }
        if sv.is_empty() {
            return Err(Error::ErrorMessage("sv is empty".to_string()));
        }
        for sve in sv.iter() {
            if Scalar::from_canonical_bytes(sve.bytes).is_none() {
                return Err(Error::ErrorMessage("Invalid sv input".to_string()));
            }
        }
        // Useful proof bounds
        //
        // N: number of bits in each range (here, 64)
        // logN: base-2 logarithm
        // M: first power of 2 greater than or equal to the number of range proofs to
        // aggregate logM: base-2 logarithm
        let mut logM = 0;
        let mut M = 1 << logM;
        let maxM = BULLETPROOF_PLUS_MAX_OUTPUTS;
        let N = BPP_N_BITS;
        while M <= maxM && M < amounts.len() {
            logM += 1;
            M = 1 << logM;
        }
        let MN = M * N;

        // Prepare output commitments and offset by a factor of 8**(-1)
        let mut V = vec![RctKey::zero(); amounts.len()];
        for i in 0..amounts.len() {
            let gamma8 = gamma[i].as_scalar() * INV_EIGHT.as_scalar();
            let sv8 = sv[i].as_scalar() * INV_EIGHT.as_scalar();
            V[i] = RctKey::from_point(
                &(gamma8 * G_BASEPOINT.as_point() + sv8 * H_BASEPOINT.as_point()),
            );
        }

        let mut aL: Vec<RctKey> = vec![RctKey::zero(); MN];
        let mut aL8: Vec<RctKey> = vec![RctKey::zero(); MN];
        let mut aR: Vec<RctKey> = vec![RctKey::zero(); MN];
        let mut aR8: Vec<RctKey> = vec![RctKey::zero(); MN];

        for j in 0..M {
            for i in (0..N).rev() {
                if j < sv.len() && ((sv[j].bytes[i / 8] & ((1u64 << (i % 8)) as u8)) != 0) {
                    aL[j * N + i] = RctKey::identity();
                    aL8[j * N + i] = INV_EIGHT;
                    aR[j * N + i] = RctKey::zero();
                    aR8[j * N + i] = RctKey::zero();
                } else {
                    aL[j * N + i] = RctKey::zero();
                    aL8[j * N + i] = RctKey::zero();
                    aR[j * N + i] = MINUS_ONE;
                    aR8[j * N + i] = INV_EIGHT;
                }
            }
        }

        Self::try_again(&sv, gamma, &V, logM, &aL, &aL8, &aR, &aR8)
    }

    /// Adopted from Monero's try_agin goto portion within the
    /// bulletproof_plus_PROVE function
    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn try_again(
        _sv: &[RctKey],
        gamma: &[RctKey],
        V: &[RctKey],
        logM: usize,
        aL: &[RctKey],
        aL8: &[RctKey],
        aR: &[RctKey],
        aR8: &[RctKey],
    ) -> Result<Self, Error> {
        // initial transcript

        let inital_transcript = Transcript(RctKey::from_slice(
            &hash_to_point(keccak256(b"bulletproof_plus_transcript"))
                .compress()
                .to_bytes(),
        ));

        let M = 1 << logM;
        let N = BPP_N_BITS;
        let MN = M * N;
        let mut transcript = inital_transcript;

        // update transcript based on V, use all data in V for data to hash
        transcript.update(V);

        // compute A
        let random_bytes: [u8; 32] = thread_rng().gen();
        let alpha = Scalar::from_bytes_mod_order(random_bytes);
        let pre_A = Self::vector_exponent(aL8, aR8)?;
        let temp = alpha * INV_EIGHT.as_scalar();
        let A = RctKey::from_point(&(pre_A + temp * G_BASEPOINT.as_point()));

        // compute y and z (challenges)
        transcript.update(&[A]);
        let y = transcript.0;
        if y == RctKey::zero() {
            println!("y is 0, trying again");
            return Self::try_again(_sv, gamma, V, logM, aL, aL8, aR, aR8);
        }
        transcript = Transcript(RctKey::from_scalar(&Hash::hash_to_scalar(&y.bytes)));
        let z = transcript.0;
        if z == RctKey::zero() {
            println!("z is 0, trying again");
            return Self::try_again(_sv, gamma, V, logM, aL, aL8, aR, aR8);
        }

        let z_squared = z.as_scalar() * z.as_scalar();

        // Windowed vector
        // d[j*N+i] = z**(2*(j+1)) * 2**i
        //
        // We compute this iteratively in order to reduce scalar operations.
        let mut d: Vec<Scalar> = vec![Scalar::zero(); MN];
        d[0] = z_squared;
        for i in 1..N {
            d[i] = d[i - 1] * TWO.as_scalar();
        }
        for j in 1..M {
            for i in 0..N {
                d[j * N + i] = d[(j - 1) * N + i] * z_squared;
            }
        }

        // compute y_powers
        let y_powers = Self::vector_of_scalar_powers(&y, MN + 2);

        // Prepare inner product terms
        let aL1: Vec<Scalar> = aL.iter().map(|a| a.as_scalar() - z.as_scalar()).collect();

        let mut aR1: Vec<Scalar> = aR.iter().map(|a| a.as_scalar() + z.as_scalar()).collect();
        let d_y: Vec<Scalar> = d
            .iter()
            .enumerate()
            .map(|(i, x)| x * y_powers[MN - i])
            .collect();

        aR1 = aR1.iter().enumerate().map(|(i, a)| a + d_y[i]).collect();

        let mut alpha1 = alpha;
        let mut temp = ONE.as_scalar();

        for item in gamma {
            temp *= z_squared;
            let temp2 = y_powers[MN + 1] * temp;
            alpha1 = temp2 * item.as_scalar() + alpha1;
        }

        // Preparing for inner product
        let mut nprime = MN;
        let mut GPrime = vec![EdwPntGrp(EdwardsPoint::identity()); MN];
        let mut HPrime = vec![EdwPntGrp(EdwardsPoint::identity()); MN];
        let mut aprime = vec![Scalar::zero(); MN];
        let mut bprime = vec![Scalar::zero(); MN];

        let yinv = y.as_scalar().invert();
        let mut yinvpow: Vec<Scalar> = vec![Scalar::zero(); MN];
        yinvpow[0] = ONE.as_scalar();
        for i in 0..MN {
            GPrime[i] = EdwPntGrp(GENERATORS.G[i]);
            HPrime[i] = EdwPntGrp(GENERATORS.H[i]);
            if i > 0 {
                yinvpow[i] = yinvpow[i - 1] * yinv;
            }
            aprime[i] = aL1[i];
            bprime[i] = aR1[i];
        }

        let logN = 6; // based on N = 64 always, log2(64)
        let logMN = logM + logN;

        let mut L: Vec<EdwPntGrp> = vec![EdwPntGrp(EdwardsPoint::identity()); logMN];
        let mut R: Vec<EdwPntGrp> = vec![EdwPntGrp(EdwardsPoint::identity()); logMN];
        let mut round = 0;

        // Inner product rounds
        while nprime > 1 {
            nprime /= 2;

            let (aprime_left, aprime_right) = aprime.split_at(nprime);
            let (bprime_left, bprime_right) = bprime.split_at(nprime);
            let (GPrime_left, _GPrime_right) = GPrime.split_at(nprime);
            let (_HPrime_left, HPrime_right) = HPrime.split_at(nprime);
            let cL = Self::weighted_inner_product(
                &aprime_left.to_vec(),
                &bprime_right.to_vec(),
                &y.as_scalar(),
            );
            let vec_scalar = aprime_left
                .to_vec()
                .iter()
                .map(|a| a * y_powers[nprime])
                .collect::<Vec<Scalar>>();
            let cR =
                Self::weighted_inner_product(&vec_scalar, &bprime_left.to_vec(), &y.as_scalar());
            let dL = Scalar::from_bytes_mod_order(thread_rng().gen());
            let dR = Scalar::from_bytes_mod_order(thread_rng().gen());

            L[round] = Self::compute_LR(
                nprime,
                &yinvpow[nprime],
                GPrime_left,
                HPrime_right,
                aprime_right,
                bprime_left,
                &cL,
                &dL,
            );

            R[round] = Self::compute_LR(
                nprime,
                &y_powers[nprime],
                &GPrime,
                &HPrime,
                &aprime,
                &bprime,
                &cR,
                &dR,
            );

            transcript.update(&[RctKey::from_point(&L[round]), RctKey::from_point(&R[round])]);
            let challenge = transcript.0;

            if challenge == RctKey::zero() {
                println!("challenge is 0, trying again");
                return Self::try_again(_sv, gamma, V, logM, aL, aL8, aR, aR8);
            }

            let challenge_inv = challenge.as_scalar().invert();

            let temp = yinvpow[nprime] * challenge.as_scalar();

            Self::hadamard_fold(&mut GPrime, ScGrp(challenge_inv), ScGrp(temp));
            Self::hadamard_fold(
                &mut HPrime,
                ScGrp(challenge.as_scalar()),
                ScGrp(challenge_inv),
            );

            let temp = challenge_inv * y_powers[nprime];

            let ap1: Vec<Scalar> = aprime_left
                .iter()
                .map(|x| x * challenge.as_scalar())
                .collect();
            let ap2: Vec<Scalar> = aprime_right.iter().map(|x| x * temp).collect();
            assert!(ap1.len() == ap2.len());
            aprime = ap1.iter().zip(ap2.iter()).map(|(a, b)| a + b).collect();
            let bp1: Vec<Scalar> = bprime_left.iter().map(|x| x * challenge_inv).collect();
            let bp2: Vec<Scalar> = bprime_right
                .iter()
                .map(|x| x * challenge.as_scalar())
                .collect();
            bprime = bp1.iter().zip(bp2.iter()).map(|(a, b)| a + b).collect();
            let challenge_squared = challenge.as_scalar() * challenge.as_scalar();
            let challenge_squred_inv = challenge_inv * challenge_inv;
            alpha1 += dL * challenge_squared;
            alpha1 += dR * challenge_squred_inv;
            round += 1;
        }

        // Final round computations
        let r = PrivateKey::new().0;
        let s = PrivateKey::new().0;
        let d_ = PrivateKey::new().0;
        let eta = PrivateKey::new().0;

        let mut A1_data: Vec<(ScGrp, EdwPntGrp)> = vec![
            (
                ScGrp(RctKey::zero().as_scalar()),
                EdwPntGrp(RctKey::zero().as_point())
            );
            4
        ];
        A1_data[0] = (ScGrp(r * INV_EIGHT.as_scalar()), GPrime[0]);
        A1_data[1] = (ScGrp(s * INV_EIGHT.as_scalar()), HPrime[0]);
        A1_data[2] = (
            ScGrp(d_ * INV_EIGHT.as_scalar()),
            EdwPntGrp(G_BASEPOINT.hash_to_point()),
        );

        let mut temp = r * y.as_scalar();
        temp *= bprime[0];
        let mut temp2 = s * y.as_scalar();
        temp2 *= aprime[0];
        temp += temp2;
        A1_data[3].0 = ScGrp(temp * INV_EIGHT.as_scalar());
        A1_data[3].1 = EdwPntGrp(H_BASEPOINT.hash_to_point());

        let A1 = multiexp(A1_data.as_slice());

        let mut temp = r * y.as_scalar();
        temp *= s;
        temp *= INV_EIGHT.as_scalar();
        let temp2 = eta * INV_EIGHT.as_scalar();
        let B = temp2 * G_BASEPOINT.as_point() + temp * H_BASEPOINT.as_point();

        transcript.update(&[RctKey::from_point(&A1), RctKey::from_point(&B)]);
        let e = transcript.0;
        if e == RctKey::zero() {
            println!("e is 0, trying again");
            return Self::try_again(_sv, gamma, V, logM, aL, aL8, aR, aR8);
        }

        let e_squared = e.as_scalar() * e.as_scalar();
        let r1 = r + (aprime[0] * e.as_scalar());
        let s1 = s + (bprime[0] * e.as_scalar());
        let d1 = eta + (d_ * e.as_scalar()) + (alpha1 * e_squared);

        Ok(BulletproofPlus {
            V: V.to_vec(),
            A,
            A1: RctKey::from_point(&A1),
            B: RctKey::from_point(&B),
            r1: RctKey::from_slice(&r1.to_bytes()),
            s1: RctKey::from_slice(&s1.to_bytes()),
            d1: RctKey::from_slice(&d1.to_bytes()),
            L: L.iter()
                .map(|x| RctKey::from_slice(&x.compress().to_bytes()))
                .collect(),
            R: R.iter()
                .map(|x| RctKey::from_slice(&x.compress().to_bytes()))
                .collect(),
        })
    }

    fn hadamard_fold(v: &mut Vec<EdwPntGrp>, a: ScGrp, b: ScGrp) {
        assert!(v.len() % 2 == 0, "Vector size should be even");
        let sz = v.len() / 2;
        let mut res = vec![EdwPntGrp(RctKey::zero().as_point()); sz];

        for n in 0..sz {
            res[n] = multiexp(&[(a, v[n]), (b, v[sz + n])]);
        }
        *v = res
    }

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn compute_LR(
        size: usize,
        y: &Scalar,
        g: &[EdwPntGrp],
        h: &[EdwPntGrp],
        a: &[Scalar],
        b: &[Scalar],
        c: &Scalar,
        d: &Scalar,
    ) -> EdwPntGrp {
        let mut multiexp_data: Vec<(ScGrp, EdwPntGrp)> =
            vec![(ScGrp(Scalar::zero()), EdwPntGrp(EdwardsPoint::identity())); size * 2 + 2];
        let inv_eight = INV_EIGHT.as_scalar();
        for i in 0..size {
            let temp = a[i] * y;
            multiexp_data[i * 2].0 = ScGrp(temp * inv_eight);
            multiexp_data[i * 2].1 = g[i];

            multiexp_data[i * 2 + 1].0 = ScGrp(b[i] * inv_eight);
            multiexp_data[i * 2 + 1].1 = h[i];
        }

        multiexp_data[size * 2].0 = ScGrp(*c * inv_eight);
        multiexp_data[size * 2].1 = EdwPntGrp(H_BASEPOINT.hash_to_point());

        multiexp_data[size * 2 + 1].0 = ScGrp(*d * inv_eight);
        multiexp_data[size * 2 + 1].1 = EdwPntGrp(G_BASEPOINT.hash_to_point());

        let result = multiexp(multiexp_data.as_slice());
        result
    }

    /// Given two scalar arrays, construct the weighted inner product against
    /// another scalar
    ///
    /// Output a_0*b_0*y**1 + a_1*b_1*y**2 + ... + a_{n-1}*b_{n-1}*y**n
    fn weighted_inner_product(a: &Vec<Scalar>, b: &Vec<Scalar>, y: &Scalar) -> Scalar {
        assert_eq!(a.len(), b.len(), "expected a and b to be the same length");
        let mut res = Scalar::zero();
        let mut y_power = ONE.as_scalar();
        for i in 0..a.len() {
            let temp = a[i] * b[i];
            y_power *= y;
            res = temp * y_power + res;
        }
        res
    }

    /// Given a scalar, construct a vector of its powers:
    ///
    /// Output (1,x,x**2,...,x**{n-1})
    fn vector_of_scalar_powers(base: &RctKey, n: usize) -> Vec<Scalar> {
        assert!(n > 0, "expected n > 0");
        let mut powers: Vec<Scalar> = Vec::new();
        let power = RctKey::identity().as_scalar();
        powers.push(power);
        if n == 1 {
            return powers;
        }
        powers.push(base.as_scalar());
        for i in 2..n {
            powers.push(powers[i - 1] * base.as_scalar());
        }
        powers
    }

    // Adopting from Monero's vector_exponent function
    // Given two scalar arrays, construct a vector pre-commitment:
    //
    // a = (a_0, ..., a_{n-1})
    // b = (b_0, ..., b_{n-1})
    //
    // Outputs a_0*Gi_0 + ... + a_{n-1}*Gi_{n-1} +
    //         b_0*Hi_0 + ... + b_{n-1}*Hi_{n-1}
    fn vector_exponent(a: &[RctKey], b: &[RctKey]) -> Result<EdwardsPoint, anyhow::Error> {
        if a.len() != b.len() {
            return Err(anyhow!("a and b must be the same length"));
        }
        // let mut data: Vec<(Scalar, EdwardsPoint)> = Vec::new();
        // for i in 0..a.len() {
        //     data.push((Scalar::from_bytes_mod_order(a[i].bytes), G_POINT));
        //     data.push((Scalar::from_bytes_mod_order(b[i].bytes), H_POINT));
        // }
        let mut sum = EdwardsPoint::default();
        for i in 0..a.len() {
            let a_scalar = Scalar::from_bytes_mod_order(a[i].bytes);
            let b_scalar = Scalar::from_bytes_mod_order(b[i].bytes);
            let a_point = a_scalar * GENERATORS.G[i];
            let b_point = b_scalar * GENERATORS.H[i];
            sum = a_point + b_point;
        }
        // TODO(#68): check if this should be a scalar sum
        Ok(sum)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum RCTType {
    Null = 0,
    Full = 1,
    Simple = 2,
    Bulletproof = 3,
    Bulletproof2 = 4,
    CLSAG = 5,
    BulletproofPlus = 6,
}

impl RCTType {
    pub fn is_rct_simple(&self) -> bool {
        matches!(
            self,
            RCTType::Simple
                | RCTType::Bulletproof
                | RCTType::Bulletproof2
                | RCTType::CLSAG
                | RCTType::BulletproofPlus
        )
    }

    pub fn is_rct_clsag(&self) -> bool {
        matches!(self, RCTType::CLSAG | RCTType::BulletproofPlus)
    }

    pub fn from_u8(value: u8) -> Result<Self, anyhow::Error> {
        match value {
            0 => Ok(RCTType::Null),
            1 => Ok(RCTType::Full),
            2 => Ok(RCTType::Simple),
            3 => Ok(RCTType::Bulletproof),
            4 => Ok(RCTType::Bulletproof2),
            5 => Ok(RCTType::CLSAG),
            6 => Ok(RCTType::BulletproofPlus),
            _ => Err(anyhow!("Invalid RCTType")),
        }
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }
}

/// Implemented in Rust based on Monero's RangeProofType enum
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#299)
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum RangeProofType {
    RangeProofBorromean,
    RangeProofBulletproof,
    RangeProofMultiOutputBulletproof,
    RangeProofPaddedBulletproof,
}

/// Implemented in Rust based on Monero's RCRConfig struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L300-309)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RctConfig {
    pub range_proof_type: RangeProofType,
    pub bp_version: isize,
}

/// Implemented in Rust based on Monero's mgSig struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L160-L170)
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct MgSig {
    pub ss: Vec<Vec<RctKey>>,
    pub cc: RctKey,
    pub II: Vec<RctKey>,
}

/// Implemented in Rust based on Monero's clsag struct
/// ***Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L172-L186)
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct Clsag {
    pub s: Vec<RctKey>,
    pub c1: RctKey,
    pub I: RctKey,
    pub D: RctKey,
}

impl Clsag {
    /// Implementation of Monero's proveRctCLSAGSimple function
    /// **Source** <`monero/src/ringct/rctSigs.cpp`>(https://github.com/monero-project/monero/blob/1a568deacbb3bbcd65fd092fcd4c20b14ffa6a26/src/ringct/rctSigs.cpp#L765-L792)
    #[allow(non_snake_case)]
    pub fn new_proof(
        message: &RctKey,
        pubs: &Vec<CtKey>,
        in_sk: &CtKey,
        a: &RctKey,
        Cout: &RctKey,
        index: usize,
    ) -> Result<Self, anyhow::Error> {
        let _rows = 1;
        let cols = pubs.len();
        if cols == 0 {
            return Err(anyhow!("Empty pubs"));
        }

        let mut P: Vec<RctKey> = Vec::new();
        let mut C: Vec<RctKey> = Vec::new();
        let mut C_nonzero: Vec<RctKey> = Vec::new();
        for k in pubs {
            P.push(k.dest);
            C_nonzero.push(k.mask);
            let tmp = k.mask.as_point() - Cout.as_point();
            C.push(RctKey::from_point(&tmp));
        }

        let mut sk: Vec<RctKey> = Vec::new();
        sk.push(in_sk.dest);
        sk.push(RctKey::from_scalar(
            &(in_sk.mask.as_scalar() - a.as_scalar()),
        ));
        let result = Clsag::generate(message, &P, &sk[0], &C, &sk[1], &C_nonzero, Cout, index)?;
        sk.zeroize();
        Ok(result)
    }

    /// Generate a CLSAG signature
    /// Implementing based on Monero's CLSAG_Gen function
    /// **Source** <`monero/src/ringct/rctSigs.cpp`>(https://github.com/monero-project/monero/blob/1a568deacbb3bbcd65fd092fcd4c20b14ffa6a26/src/ringct/rctSigs.cpp#L235-L365)
    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    pub fn generate(
        message: &RctKey,
        P: &Vec<RctKey>,
        p: &RctKey,
        C: &Vec<RctKey>,
        z: &RctKey,
        C_nonzero: &Vec<RctKey>,
        C_offset: &RctKey,
        l: usize,
    ) -> Result<Clsag, anyhow::Error> {
        let mut sig = Clsag::default();
        let n = P.len();
        if n != C.len() {
            return Err(anyhow!(
                "Signing and commitment key vector sizes must match!"
            ));
        }
        if n != C_nonzero.len() {
            return Err(anyhow!(
                "Signing and commitment key vector sizes must match!"
            ));
        }
        if l >= n {
            return Err(anyhow!("Signing index out of range!"));
        }

        // Key images
        let H = RctKey::from_point(&P[l].hash_to_point());

        // clsag_prepare
        let mut a = RctKey::from_slice(PrivateKey::new().as_slice());
        let aG = RctKey::from_slice(
            PublicKey::from_private_key(&PrivateKey::from_slice(&a.bytes)?).as_slice(),
        );
        let aH = RctKey::from_point(&(a.as_scalar() * H_BASEPOINT.as_point()));
        sig.I = RctKey::from_point(&(p.as_scalar() * H.as_point()));
        let D = RctKey::from_point(&(z.as_scalar() * H.as_point()));

        // Precompute I_precomp and D_precomp
        let I_precomp = VartimeEdwardsPrecomputation::new([&sig.I.as_point()]);
        let D_precomp = VartimeEdwardsPrecomputation::new([&D.as_point()]);

        // Offset key image
        sig.D = RctKey::from_point(&(D.hash_to_point() * INV_EIGHT.as_scalar()));

        // Aggregation hashes
        let mut mu_P_to_hash: Vec<RctKey> = vec![RctKey::default(); 2 * n + 4];
        let mut mu_C_to_hash: Vec<RctKey> = vec![RctKey::default(); 2 * n + 4];
        mu_P_to_hash[0] = RctKey::zero();
        mu_P_to_hash[0].bytes[0..HASH_KEY_CLSAG_AGG_0.len()].copy_from_slice(HASH_KEY_CLSAG_AGG_0);
        mu_C_to_hash[0] = RctKey::zero();
        mu_C_to_hash[0].bytes[0..HASH_KEY_CLSAG_AGG_1.len()].copy_from_slice(HASH_KEY_CLSAG_AGG_1);

        mu_P_to_hash[1..(n + 1)].copy_from_slice(&P[0..n]);
        mu_C_to_hash[1..(n + 1)].copy_from_slice(&C[0..n]);

        for i in (n + 1)..(2 * n + 1) {
            mu_P_to_hash[i] = C_nonzero[i - n - 1];
            mu_C_to_hash[i] = C_nonzero[i - n - 1];
        }
        mu_P_to_hash[2 * n + 1] = sig.I;
        mu_P_to_hash[2 * n + 2] = sig.D;
        mu_P_to_hash[2 * n + 3] = *C_offset;
        mu_C_to_hash[2 * n + 1] = sig.I;
        mu_C_to_hash[2 * n + 2] = sig.D;
        mu_C_to_hash[2 * n + 3] = *C_offset;

        let mu_P = RctKey::from_scalar(&Hash::hash_to_scalar(
            mu_P_to_hash
                .iter()
                .map(|x| x.as_bytes())
                .collect::<Vec<&[u8]>>()
                .concat()
                .as_slice(),
        ));
        let mu_C = RctKey::from_scalar(&Hash::hash_to_scalar(
            mu_C_to_hash
                .iter()
                .map(|x| x.as_bytes())
                .collect::<Vec<&[u8]>>()
                .concat()
                .as_slice(),
        ));

        // Initial commitment
        let mut c_to_hash = vec![RctKey::default(); 2 * n + 5];
        c_to_hash[0] = RctKey::zero();
        c_to_hash[0].bytes[0..HASH_KEY_CLSAG_ROUND.len()].copy_from_slice(HASH_KEY_CLSAG_ROUND);

        c_to_hash[1..(n + 1)].copy_from_slice(&P[..n]);
        c_to_hash[(1 + n)..(n + 1 + n)].copy_from_slice(&C_nonzero[..n]);

        c_to_hash[2 * n + 1] = *C_offset;
        c_to_hash[2 * n + 2] = *message;

        c_to_hash[2 * n + 3] = aG;
        c_to_hash[2 * n + 4] = aH;

        // clsag_hash
        let mut c = RctKey::from_scalar(&Hash::hash_to_scalar(
            c_to_hash
                .iter()
                .map(|x| x.as_bytes())
                .collect::<Vec<&[u8]>>()
                .concat()
                .as_slice(),
        ));

        let mut i = (l + 1) % n;
        if i == 0 {
            sig.c1 = c;
        }

        // Decoy indices
        sig.s = vec![RctKey::default(); n];
        let mut P_precomp;
        let mut C_precomp;
        // let mut Hi_p3;

        while i != l {
            sig.s[i] = RctKey::from_slice(PrivateKey::new().as_slice());
            let c_p = RctKey::from_scalar(&(mu_P.as_scalar() * c.as_scalar()));
            let c_c = RctKey::from_scalar(&(mu_C.as_scalar() * c.as_scalar()));

            // Precompute points
            P_precomp = VartimeEdwardsPrecomputation::new([P[i].as_point()]);
            C_precomp = VartimeEdwardsPrecomputation::new([C[i].as_point()]);

            // Compute L
            let L = sig.s[i].as_scalar() * G_BASEPOINT.as_point()
                + P_precomp.vartime_multiscalar_mul([&c_p.as_scalar()])
                + C_precomp.vartime_multiscalar_mul([&c_c.as_scalar()]);

            // Compute R
            let hash8_p3 = &hash_to_point(P[i].bytes).mul_by_cofactor();
            let hash_precomp = VartimeEdwardsPrecomputation::new([hash8_p3]);
            let R = hash_precomp.vartime_multiscalar_mul([&sig.s[i].as_scalar()])
                + I_precomp.vartime_multiscalar_mul([&c_p.as_scalar()])
                + D_precomp.vartime_multiscalar_mul([&c_c.as_scalar()]);

            c_to_hash[2 * n + 3] = RctKey::from_point(&L);
            c_to_hash[2 * n + 4] = RctKey::from_point(&R);

            // update clsag_hash
            let c_new = RctKey::from_scalar(&Hash::hash_to_scalar(
                c_to_hash
                    .iter()
                    .map(|x| x.as_bytes())
                    .collect::<Vec<&[u8]>>()
                    .concat()
                    .as_slice(),
            ));
            c = c_new;

            i = (i + 1) % n;
            if i == 0 {
                sig.c1 = c;
            }
        }

        // Compute final scalar

        // clsag_sign

        let s0_p_mu_P = RctKey::from_scalar(&(mu_P.as_scalar() * p.as_scalar()));
        let s0_add_z_mu_C =
            RctKey::from_scalar(&(mu_C.as_scalar() * z.as_scalar() + s0_p_mu_P.as_scalar()));
        let scalar = a.as_scalar() - c.as_scalar() * s0_add_z_mu_C.as_scalar();
        sig.s[l] = RctKey::from_scalar(&scalar);
        a.zeroize();

        Ok(sig)
    }
}

/// Implemented in Rust based on Monero's rctSigPrunable struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L398-L404)
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct RctSigPrunable {
    pub range_sigs: Vec<RangeSig>,
    pub bulletproofs: Vec<Bulletproof>,
    pub bulletproofs_plus: Vec<BulletproofPlus>,
    pub MGs: Vec<MgSig>,
    pub CLSAGs: Vec<Clsag>,
    pub pseudo_outs: Vec<RctKey>,
}

impl DoSerialize for RctSigPrunable {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        let nbp = self.bulletproofs_plus.len();
        serialized.serialize_field("nbp", &VarInt(nbp as u32))?;
        serialized.add_tag("bpp");
        serialized.begin_array();
        // this assumes we are using bulletproof plus
        for (i, bp) in self.bulletproofs_plus.iter().enumerate() {
            bp.do_serialize(serialized)?;

            if i < nbp - 1 {
                serialized.delimit_array();
            }
        }
        serialized.end_array();
        // this assumes we are using bulletproof plus, and CLSAGs are relevant
        serialized.add_tag("CLSAGs");
        serialized.begin_array();
        for (i, clsag) in self.CLSAGs.iter().enumerate() {
            serialized.begin_object();
            serialized.add_tag("s");
            serialized.begin_array();
            for (j, s) in clsag.s.iter().enumerate() {
                s.do_serialize(serialized)?;
                if j < clsag.s.len() - 1 {
                    serialized.delimit_array();
                }
            }
            serialized.end_array();
            serialized.add_tag("c1");
            clsag.c1.do_serialize(serialized)?;
            serialized.add_tag("D");
            clsag.D.do_serialize(serialized)?;
            serialized.end_object();

            if i < self.CLSAGs.len() - 1 {
                serialized.delimit_array();
            }
        }
        serialized.end_array();

        // again using the assumption that we are using RCTType::BulletproofPlus
        // previous parts in the workflow, should have checked this assumption

        serialized.add_tag("pseudoOuts");
        serialized.begin_array_with_size(self.pseudo_outs.len())?;
        for (i, pseudo_out) in self.pseudo_outs.iter().enumerate() {
            pseudo_out.do_serialize(serialized)?;
            if i < self.pseudo_outs.len() - 1 {
                serialized.delimit_array();
            }
        }
        serialized.end_array();
        Ok(())
    }
}

/// Implemented in Rust based on Monero's rctSigPrunable struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L595-612)
#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct RctSig {
    pub base: RctSigBase,
    pub p: RctSigPrunable,
}

impl RctSig {
    /// Implemented in Rust based on Monero's rctSig::get_pre_mlsag_hash()
    /// **Source** <`monero/src/ringct/rctSigs.cpp`>(https://github.com/monero-project/monero/blob/1a568deacbb3bbcd65fd092fcd4c20b14ffa6a26/src/ringct/rctSigs.cpp#L601-L678)
    #[allow(non_snake_case)]
    pub fn pre_mlsag_hash(&self) -> Result<RctKey, anyhow::Error> {
        let base = self.base.clone();
        let mut hashes: Vec<RctKey> = Vec::new();
        hashes.reserve(3);
        hashes.push(base.message);
        let mut _h: CryptoHash;
        if base.mix_ring.is_empty() {
            return Err(anyhow!("Empty mix_ring"));
        }
        let rct_type = base.rct_type;

        let inputs: usize = if rct_type.is_rct_simple() {
            base.mix_ring.len()
        } else {
            base.mix_ring[0].len()
        };

        if base.pseudo_outs.len() != inputs {
            return Err(anyhow!("Invalid pseudo_outs size"));
        }

        let mut serialized_rctsig_base = SerializedArchive::new();
        base.do_serialize(&mut serialized_rctsig_base)?;
        let base_str_hash = hash::keccak256(serialized_rctsig_base.json_stream.as_bytes());
        hashes.push(RctKey::from_slice(&base_str_hash));

        let mut key_vec: Vec<RctKey> = Vec::new();

        // Right now assuming that RCTType is always BulletproofPlus
        if rct_type != RCTType::BulletproofPlus {
            return Err(anyhow!(
                "Currently only supporting the BulletproofPlus RCT type"
            ));
        }
        key_vec.reserve((6 * 2 + 6) * self.p.bulletproofs_plus.len());
        for p in self.p.bulletproofs_plus.clone() {
            key_vec.push(p.A);
            key_vec.push(p.A1);
            key_vec.push(p.B);
            key_vec.push(p.r1);
            key_vec.push(p.s1);
            key_vec.push(p.d1);
            for pL in p.L {
                key_vec.push(pL);
            }
            for pR in p.R {
                key_vec.push(pR);
            }
        }
        let key_vec_data: Vec<u8> = key_vec
            .iter()
            .map(|k| k.to_bytes())
            .collect::<Vec<Vec<u8>>>()
            .concat();
        let key_vec_hash = hash::keccak256(&key_vec_data);
        hashes.push(RctKey::from_slice(&key_vec_hash));
        let hashes_data: Vec<u8> = hashes
            .iter()
            .map(|k| k.to_bytes())
            .collect::<Vec<Vec<u8>>>()
            .concat();
        let mlsag_prehash = hash::keccak256(&hashes_data);
        Ok(RctKey::from_slice(&mlsag_prehash))
    }

    /// Implemented in Rust based on Monero's verRctCLSAGSimple function
    /// **Source** <`monero/src/ringct/rctSigs.cpp`>(https://github.com/monero-project/monero/blob/1a568deacbb3bbcd65fd092fcd4c20b14ffa6a26/src/ringct/rctSigs.cpp#L873-984)
    #[allow(non_snake_case)]
    pub fn verify_rct_clsag_simple(
        &self,
        message: &RctKey,
        i: usize,
    ) -> Result<bool, anyhow::Error> {
        let sig = self.p.CLSAGs[i].clone();
        let pubs = self.base.mix_ring[i].clone();
        let C_offset = self.base.pseudo_outs[i];
        let n = pubs.len();

        // Check data
        if n < 1 {
            return Err(anyhow!("Empty pubs"));
        }
        if n != sig.s.len() {
            return Err(anyhow!("Signature scalar vector is the wrong size!"));
        }
        for i in 0..n {
            if Scalar::from_canonical_bytes(sig.s[i].bytes).is_none() {
                return Err(anyhow!("Bad signature scalar!"));
            }
        }
        if Scalar::from_canonical_bytes(sig.c1.bytes).is_none() {
            return Err(anyhow!("Bad signature commitment!"));
        }
        if sig.I == RctKey::default() {
            return Err(anyhow!("Bad key image!"));
        }

        let C_offset_p3 = hash_to_point(C_offset.bytes);

        // Prepare key images
        let mut c = sig.c1;
        let D_8 = RctKey::from_point(&sig.D.as_point().mul_by_cofactor());
        if D_8 == RctKey::identity() {
            return Err(anyhow!("Bad auxiliary key image!"));
        }
        // Precompute I_precomp and D_precomp
        let I_precomp = VartimeEdwardsPrecomputation::new([&sig.I.as_point()]);
        let D_precomp = VartimeEdwardsPrecomputation::new([&D_8.as_point()]);
        // Aggregation hashes
        let mut mu_P_to_hash: Vec<RctKey> = Vec::new();
        mu_P_to_hash.resize(2 * n + 4, RctKey::default());
        let mut mu_C_to_hash: Vec<RctKey> = Vec::new();
        mu_C_to_hash.resize(2 * n + 4, RctKey::default());
        mu_P_to_hash[0] = RctKey::zero();
        mu_P_to_hash[0].bytes[0..HASH_KEY_CLSAG_AGG_0.len()].copy_from_slice(HASH_KEY_CLSAG_AGG_0);
        mu_C_to_hash[0] = RctKey::zero();
        mu_C_to_hash[0].bytes[0..HASH_KEY_CLSAG_AGG_1.len()].copy_from_slice(HASH_KEY_CLSAG_AGG_1);
        for i in 1..n + 1 {
            mu_P_to_hash[i] = pubs[i - 1].dest;
            mu_C_to_hash[i] = pubs[i - 1].dest;
        }
        for i in n + 1..2 * n + 1 {
            mu_P_to_hash[i] = pubs[i - n - 1].mask;
            mu_C_to_hash[i] = pubs[i - n - 1].mask;
        }
        mu_P_to_hash[2 * n + 1] = sig.I;
        mu_P_to_hash[2 * n + 2] = sig.D;
        mu_P_to_hash[2 * n + 3] = C_offset;
        mu_C_to_hash[2 * n + 1] = sig.I;
        mu_C_to_hash[2 * n + 2] = sig.D;
        mu_C_to_hash[2 * n + 3] = C_offset;
        let mu_P = RctKey::from_scalar(&Hash::hash_to_scalar(
            mu_P_to_hash
                .iter()
                .map(|x| x.as_bytes())
                .collect::<Vec<&[u8]>>()
                .concat()
                .as_slice(),
        ));

        let mu_C = RctKey::from_scalar(&Hash::hash_to_scalar(
            mu_C_to_hash
                .iter()
                .map(|x| x.as_bytes())
                .collect::<Vec<&[u8]>>()
                .concat()
                .as_slice(),
        ));

        // Set up round hash
        let mut c_to_hash: Vec<RctKey> = vec![RctKey::default(); 2 * n + 5];
        c_to_hash[0] = RctKey::zero();
        c_to_hash[0].bytes[0..HASH_KEY_CLSAG_ROUND.len()].copy_from_slice(HASH_KEY_CLSAG_ROUND);
        for i in 1..n + 1 {
            c_to_hash[i] = pubs[i - 1].dest;
            c_to_hash[i + n] = pubs[i - 1].mask;
        }
        c_to_hash[2 * n + 1] = C_offset;
        c_to_hash[2 * n + 2] = *message;

        let mut i = 0;

        while i < n {
            let c_p = RctKey::from_scalar(&(mu_P.as_scalar() * c.as_scalar()));
            let c_c = RctKey::from_scalar(&(mu_C.as_scalar() * c.as_scalar()));
            let P_precomp = VartimeEdwardsPrecomputation::new([&pubs[i].dest.as_point()]);
            let temp_p3 = hash_to_point(pubs[i].mask.bytes);
            let temp_p1 = temp_p3 - C_offset_p3;
            let C_precomp = VartimeEdwardsPrecomputation::new([&temp_p1]);

            // Compute L
            let L = sig.s[i].as_scalar() * G_BASEPOINT.as_point()
                + P_precomp.vartime_multiscalar_mul([&c_p.as_scalar()])
                + C_precomp.vartime_multiscalar_mul([&c_c.as_scalar()]);

            // Compute R
            let hash8_p3 = &hash_to_point(pubs[i].dest.bytes).mul_by_cofactor();
            let hash_precomp = VartimeEdwardsPrecomputation::new([hash8_p3]);
            let R = hash_precomp.vartime_multiscalar_mul([&sig.s[i].as_scalar()])
                + I_precomp.vartime_multiscalar_mul([&c_p.as_scalar()])
                + D_precomp.vartime_multiscalar_mul([&c_c.as_scalar()]);

            c_to_hash[2 * n + 3] = RctKey::from_point(&L);
            c_to_hash[2 * n + 4] = RctKey::from_point(&R);

            // update clsag_hash
            let c_new = RctKey::from_scalar(&Hash::hash_to_scalar(
                c_to_hash
                    .iter()
                    .map(|x| x.as_bytes())
                    .collect::<Vec<&[u8]>>()
                    .concat()
                    .as_slice(),
            ));
            if c_new == RctKey::zero() {
                return Err(anyhow!("Bad signature hash"));
            }
            c = c_new;
            i += 1;
        }
        let c_new = c.as_scalar() - sig.c1.as_scalar();
        if c_new == Scalar::zero() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn verify_rct_simple(&self) -> Result<bool, anyhow::Error> {
        if !self.base.rct_type.is_rct_simple() {
            return Err(anyhow!(
                "verify_rct_simple function called on a non simple RctSig"
            ));
        }
        let bulletproof = self.base.rct_type == RCTType::Bulletproof
            || self.base.rct_type == RCTType::Bulletproof2
            || self.base.rct_type == RCTType::CLSAG;
        let bulletproof_plus = self.base.rct_type == RCTType::BulletproofPlus;

        if bulletproof || bulletproof_plus {
            if self.p.pseudo_outs.len() != self.base.mix_ring.len() {
                return Err(anyhow!("Mismatched sizes of rv.p.pseudoOuts and mixRing"));
            }
        } else if self.base.pseudo_outs.len() != self.base.mix_ring.len() {
            return Err(anyhow!("Mismatched sizes of rv.pseudoOuts and mixRing"));
        }

        let message = self.pre_mlsag_hash()?;

        let mut results: Vec<bool> = vec![false; self.base.mix_ring.len()];
        for (i, result) in results.iter_mut().enumerate() {
            // could make use of multiple threads here (parallel programming)
            // assuming that rct is clsag type, throws error otherwise
            if !self.base.rct_type.is_rct_clsag() {
                return Err(anyhow!("Currently only handling clsag rct type"));
            }
            *result = Self::verify_rct_clsag_simple(self, &message, i)?;
        }

        for (i, result) in results.iter().enumerate() {
            if !result {
                println!("verify_rct_simple: bad signature for input {}", i);
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Implemented based on Monero's `rct::genRctSimple` function
    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    pub fn generate_rct_simple(
        message: &RctKey,
        in_sk: &Vec<CtKey>,
        destinations: &Vec<RctKey>,
        in_amounts: &Vec<u64>,
        out_amounts: &Vec<u64>,
        txn_fee: u64,
        mix_ring: &Vec<Vec<CtKey>>,
        amount_keys: &Vec<RctKey>,
        indx: &Vec<u64>,
        out_sk: &mut Vec<CtKey>,
        rct_config: RctConfig,
    ) -> Result<Self, anyhow::Error> {
        let bulletproof_or_plus = rct_config.range_proof_type > RangeProofType::RangeProofBorromean;
        if in_amounts.is_empty() {
            return Err(anyhow!("Empty in_amounts"));
        }
        if in_amounts.len() != in_sk.len() {
            return Err(anyhow!("Different number of in_amounts and in_sk"));
        }
        if out_amounts.len() != destinations.len() {
            return Err(anyhow!("Different number of out_amounts and destinations"));
        }
        if amount_keys.len() != destinations.len() {
            return Err(anyhow!("Different number of amount_keys and destinations"));
        }
        if indx.len() != in_sk.len() {
            return Err(anyhow!("Different number of indx and in_sk"));
        }
        if mix_ring.len() != in_sk.len() {
            return Err(anyhow!("Different number of mix_ring and in_sk"));
        }
        for n in 0..mix_ring.len() {
            if indx[n] >= mix_ring[n].len() as u64 {
                return Err(anyhow!(
                    "Bad index into mix_ring, mix_ring[{}][{}] does not exist",
                    n,
                    indx[n]
                ));
            }
        }

        assert!(bulletproof_or_plus);
        let rct_type;
        if bulletproof_or_plus {
            match rct_config.bp_version {
                0 | 4 => rct_type = RCTType::BulletproofPlus,
                3 => rct_type = RCTType::CLSAG,
                2 => rct_type = RCTType::Bulletproof2,
                1 => rct_type = RCTType::Bulletproof,
                _ => return Err(anyhow!("Unsupported bulletproof version")),
            }
        } else {
            rct_type = RCTType::Simple;
        }
        let _rct_message = *message;
        let mut rct_out_pk: Vec<CtKey> = Vec::new();
        rct_out_pk.resize(destinations.len(), CtKey::default());
        let _rct_p_range_sigs: Vec<RangeSig> = Vec::new();
        let mut rct_ecdh_info: Vec<EcdhTuple> = Vec::new();
        rct_ecdh_info.resize(destinations.len(), EcdhTuple::default());

        out_sk.resize(destinations.len(), CtKey::default());

        for i in 0..destinations.len() {
            rct_out_pk[i].dest = destinations[i];
        }
        let _rv_p_bulletproofs_plus: Vec<BulletproofPlus> = Vec::new();
        // if bulletproof_or_plus
        let plus = rct_type == RCTType::BulletproofPlus;
        assert!(plus);
        let _n_amounts = out_amounts.len();

        let _amounts_proved = 0;
        let mut rct_p_bulletproofs_plus: Vec<BulletproofPlus> = Vec::new();
        assert!(rct_config.range_proof_type == RangeProofType::RangeProofPaddedBulletproof);

        let mut masks = vec![RctKey::identity(); out_amounts.len()];
        for i in 0..masks.len() {
            masks[i] = RctKey::gen_commitment_mask(&amount_keys[i])
        }
        let proof = BulletproofPlus::new_proof(out_amounts, &masks)?;
        let C = proof.V.clone();
        rct_p_bulletproofs_plus.push(proof);

        for i in 0..out_amounts.len() {
            rct_out_pk[i].mask = RctKey::from_point(&C[i].as_point().mul_by_cofactor());
            out_sk[i].mask = masks[i];
        }

        let mut sum_out = RctKey::zero();
        for i in 0..out_sk.len() {
            sum_out = RctKey::from_scalar(&(out_sk[i].mask.as_scalar() + sum_out.as_scalar()));

            rct_ecdh_info[i].mask = out_sk[i].mask;
            rct_ecdh_info[i].amount.bytes[0..8].copy_from_slice(&out_amounts[i].to_le_bytes());
            // assuming version 2
            rct_ecdh_info[i].encode(&amount_keys[i]);
        }

        // hardcoded now to use with bulletproofs_plus
        let mut pseudo_outs: Vec<RctKey> = Vec::new();
        pseudo_outs.resize(in_amounts.len(), RctKey::zero());

        // if bulletproof_or_plus it is clsag
        let mut clsag_s = Vec::new();
        clsag_s.resize(in_amounts.len(), RctKey::zero());

        let mut sum_pouts = RctKey::zero(); // sum of pseudo_outs
        let mut a: Vec<RctKey> = vec![RctKey::zero(); in_amounts.len()];
        for i in 0..(in_amounts.len() - 1) {
            a[i] = RctKey::from_scalar(&PrivateKey::new().0);
            sum_pouts = RctKey::from_scalar(&(a[i].as_scalar() + sum_pouts.as_scalar()));
            pseudo_outs[i] = RctKey::commit(in_amounts[i], &a[i]);
        }
        a[in_amounts.len() - 1] =
            RctKey::from_scalar(&(sum_out.as_scalar() - sum_pouts.as_scalar()));

        pseudo_outs[in_amounts.len() - 1] =
            RctKey::commit(in_amounts[in_amounts.len() - 1], &a[in_amounts.len() - 1]);

        let rct_prunable = RctSigPrunable {
            range_sigs: Vec::new(),
            bulletproofs: Vec::new(),
            bulletproofs_plus: rct_p_bulletproofs_plus,
            MGs: Vec::new(),
            CLSAGs: Vec::new(),
            pseudo_outs: pseudo_outs.clone(),
        };

        let rct_base = RctSigBase {
            txn_fee,
            rct_type,
            message: *message,
            mix_ring: mix_ring.clone(),
            pseudo_outs: pseudo_outs.clone(),
            ecdh_info: rct_ecdh_info.clone(),
            out_pk: rct_out_pk.clone(),
        };

        let mut rv = RctSig {
            base: rct_base,
            p: rct_prunable,
        };

        let full_message: RctKey = rv.pre_mlsag_hash()?;

        for i in 0..in_amounts.len() {
            if rv.base.rct_type.is_rct_clsag() {
                // proveRctCLSAGSimple
                rv.p.CLSAGs.push(Clsag::new_proof(
                    &full_message,
                    &rv.base.mix_ring[i],
                    &in_sk[i],
                    &a[i],
                    &pseudo_outs[i],
                    indx[i] as usize,
                )?);
            } else {
                return Err(anyhow!("Currently only handling RCTType::BulletproofPlus"));
            }
        }
        Ok(rv)
    }
}

/// Implemented in Rust based on Monero's multisig_out struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L114-L125)
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct MultiSigOut {
    c: Vec<RctKey>,
    mu_p: Vec<RctKey>,
    c0: Vec<RctKey>,
}

/// Implemented in Rust based on Monero's multisig_kLRki struct
/// **Source** <`monero/src/ringct/rctTypes.h`>(https://github.com/monero-project/monero/blob/50410d1f7d04bf60053f2263410c39e81d3ddad1/src/ringct/rctTypes.h#L104-112)
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct MultiSigkLRki {
    k: RctKey,
    L: RctKey,
    R: RctKey,
    ki: RctKey,
}

#[cfg(test)]
mod tests {

    // TODO(#68): Add tests
}
