use crate::monero_serialize::{DoSerialize, SerializedArchive};
use anyhow::anyhow;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;
use monero::consensus::encode::{Encodable, VarInt};
use monero::cryptonote::hash::{Hash, Hash8};
use rand::{thread_rng, RngCore};
use serde::Serialize;
use thiserror::Error;
use tiny_keccak::{Hasher, Keccak};
use zeroize::Zeroize;

const BPP_MAX_MN: usize = 1024; // BULLETPROOF_PLUS_MAX_OUTPUTS * BPP_N_BITS
const BPP_N_BITS: usize = 64;
const BULLETPROOF_PLUS_MAX_OUTPUTS: usize = 8;
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
    ],
};

const ONE: RctKey = RctKey {
    bytes: [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    ErrorMessage(String),
    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(data);
    keccak.finalize(&mut output);
    output
}

fn hash_to_point(data: [u8; 32]) -> EdwardsPoint {
    EdwardsPoint::mul_base(&Scalar::from_bytes_mod_order(data))
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy, Zeroize)]
pub struct RctKey {
    pub bytes: [u8; 32],
}

impl serde::Serialize for RctKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}

impl DoSerialize for RctKey {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.serialize_field("key", &self.bytes)?;
        Ok(())
    }
}

impl RctKey {
    pub fn from_slice(data: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(data);
        Self { bytes }
    }

    pub fn identity() -> Self {
        KEY_I
    }

    pub fn zero() -> Self {
        KEY_ZERO
    }

    pub fn h_basepoint() -> Self {
        H_BASEPOINT
    }

    pub fn from_scalar(scalar: &Scalar) -> Self {
        Self::from_slice(&scalar.to_bytes())
    }

    pub fn as_scalar(&self) -> Scalar {
        Scalar::from_bytes_mod_order(self.bytes)
    }

    pub fn as_point(&self) -> Result<EdwardsPoint, Error> {
        CompressedEdwardsY::from_slice(&self.bytes)
            .map_err(|_| Error::AnyhowError(anyhow!("Invalid edwards point")))
            .and_then(|point| {
                point
                    .decompress()
                    .ok_or_else(|| Error::AnyhowError(anyhow!("Invalid edwards point")))
            })
    }

    pub fn hash_to_point(&self) -> EdwardsPoint {
        hash_to_point(self.bytes)
    }

    pub fn from_point(point: &EdwardsPoint) -> Self {
        Self::from_slice(&point.compress().to_bytes())
    }

    pub fn commit(amount: u64, mask: &RctKey) -> Self {
        let mut b_bytes = [0u8; 32];
        b_bytes[0..8].copy_from_slice(&amount.to_le_bytes());
        let b_scalar = Scalar::from_bytes_mod_order(b_bytes);
        let a_scalar = Scalar::from_bytes_mod_order(mask.bytes);
        let c_point =
            a_scalar * G_BASEPOINT.as_point().unwrap() + b_scalar * H_BASEPOINT.as_point().unwrap();
        RctKey::from_point(&c_point)
    }

    pub fn gen_commitment_mask(sk: &RctKey) -> RctKey {
        let mut commitment_key = b"commitment_mask".to_vec();
        commitment_key.extend(sk.bytes);
        let hash_scalar = Hash::hash_to_scalar(&commitment_key);
        RctKey::from_scalar(&private_key_to_scalar(&hash_scalar))
    }

    pub fn zero_commit(amount: u64) -> Self {
        let mask = KEY_I;
        Self::commit(amount, &mask)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}

#[derive(Debug, Clone)]
pub struct Key64(pub [RctKey; 64]);

#[derive(Debug, Clone, Default, Zeroize)]
pub struct CtKey {
    pub dest: RctKey,
    pub mask: RctKey,
}

#[derive(Debug, Clone)]
pub struct BoroSig {
    pub s0: Key64,
    pub s1: Key64,
    pub ee: RctKey,
}

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

    pub fn encode(&mut self, shared_secret: &RctKey) {
        self.mask = RctKey::zero();
        let hash_secret = Self::ecdh_hash(shared_secret);
        for i in 0..8 {
            self.amount.bytes[i] ^= hash_secret.bytes[i];
        }
    }
}

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
        serialized.serialize_field("type", &(self.rct_type.value() as usize))?;
        serialized.serialize_field("txnFee", &(self.txn_fee as usize))?;
        for (i, pseudo_out) in self.pseudo_outs.iter().enumerate() {
            serialized.serialize_field(&format!("pseudoOuts[{i}]"), pseudo_out)?;
        }
        for (i, ecdh) in self.ecdh_info.iter().enumerate() {
            let hashed_amount = Hash8::from_slice(&ecdh.amount.bytes);
            serialized
                .serialize_field(&format!("ecdhInfo[{i}].amount"), &hashed_amount.as_bytes())?;
        }
        for (i, out_pk) in self.out_pk.iter().enumerate() {
            serialized.serialize_field(&format!("outPk[{i}].mask"), &out_pk.mask)?;
        }
        Ok(())
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct RangeSig {
    pub asig: BoroSig,
    pub Ci: Key64,
}

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

#[allow(non_snake_case)]
pub struct Generators {
    pub G: [EdwardsPoint; BPP_MAX_MN],
    pub H: [EdwardsPoint; BPP_MAX_MN],
}

pub fn bulletproofs_generators(dst: &'static [u8]) -> Result<Generators, Error> {
    let identity_point = EdwardsPoint::identity();
    let mut res = Generators {
        G: [identity_point; BPP_MAX_MN],
        H: [identity_point; BPP_MAX_MN],
    };
    for i in 0..BPP_MAX_MN {
        let idx = 2 * i;
        let mut even = [0u8; 32].to_vec();
        even.extend(dst);
        let mut odd = even.clone();
        let i_0 = VarInt(idx as u64);
        let mut i_0_bytes = Vec::new();
        i_0.consensus_encode(&mut i_0_bytes)
            .map_err(|e| Error::AnyhowError(anyhow::anyhow!(e)))?;
        even.extend(i_0_bytes);
        let i_1 = VarInt((idx + 1) as u64);
        let mut i_1_bytes = Vec::new();
        i_1.consensus_encode(&mut i_1_bytes)
            .map_err(|e| Error::AnyhowError(anyhow::anyhow!(e)))?;
        odd.extend(i_1_bytes);
        res.H[i / 2] = hash_to_point(keccak256(&even));
        res.G[i / 2] = hash_to_point(keccak256(&odd));
    }
    Ok(res)
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize)]
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
        serialized.serialize_field("A", &self.A)?;
        serialized.serialize_field("A1", &self.A1)?;
        serialized.serialize_field("B", &self.B)?;
        serialized.serialize_field("r1", &self.r1)?;
        serialized.serialize_field("s1", &self.s1)?;
        serialized.serialize_field("d1", &self.d1)?;
        serialized.serialize_vector("L", &self.L)?;
        serialized.serialize_vector("R", &self.R)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Transcript(pub RctKey);

impl Transcript {
    pub fn update(&mut self, update: &[RctKey]) {
        let mut data = vec![self.0];
        data.extend(update);
        let data_bytes: Vec<u8> = data.iter().flat_map(|x| x.bytes).collect();
        let hash_to_scalar = Hash::hash_to_scalar(&data_bytes);
        *self = Transcript(RctKey::from_scalar(&private_key_to_scalar(&hash_to_scalar)));
    }
}

impl BulletproofPlus {
    #[allow(non_snake_case)]
    pub fn new_proof(amounts: &Vec<u64>, masks: &[RctKey]) -> Result<Self, Error> {
        let mut sv: Vec<RctKey> = Vec::new();
        for amount in amounts {
            let mut sv_key = RctKey::zero();
            sv_key.bytes[0..8].copy_from_slice(&amount.to_le_bytes());
            sv.push(sv_key);
        }

        let gamma = masks;

        if sv.len() != gamma.len() {
            return Err(Error::ErrorMessage(
                "Incompatible sizes of sv and gamma".to_string(),
            ));
        }
        if sv.is_empty() {
            return Err(Error::ErrorMessage("sv is empty".to_string()));
        }
        for sve in sv.iter() {
            if Scalar::from_canonical_bytes(sve.bytes).is_none().into() {
                return Err(Error::ErrorMessage("Invalid sv input".to_string()));
            }
        }

        let mut logM = 0;
        let mut M = 1 << logM;
        let maxM = BULLETPROOF_PLUS_MAX_OUTPUTS;
        let N = BPP_N_BITS;
        while M <= maxM && M < amounts.len() {
            logM += 1;
            M = 1 << logM;
        }
        let MN = M * N;

        let mut V = vec![RctKey::zero(); amounts.len()];
        for i in 0..amounts.len() {
            let gamma8 = gamma[i].as_scalar() * INV_EIGHT.as_scalar();
            let sv8 = sv[i].as_scalar() * INV_EIGHT.as_scalar();
            V[i] = RctKey::from_point(
                &(gamma8 * G_BASEPOINT.as_point().unwrap() + sv8 * H_BASEPOINT.as_point().unwrap()),
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
        let initial_transcript = Transcript(RctKey::from_slice(
            &hash_to_point(keccak256(b"bulletproof_plus_transcript"))
                .compress()
                .to_bytes(),
        ));

        let M = 1 << logM;
        let N = BPP_N_BITS;
        let MN = M * N;
        let mut transcript = initial_transcript;

        transcript.update(V);

        let mut random_bytes = [0u8; 32];
        thread_rng().fill_bytes(&mut random_bytes);
        let alpha = Scalar::from_bytes_mod_order(random_bytes);
        let pre_A = Self::vector_exponent(aL8, aR8)?;
        let A = RctKey::from_point(
            &(pre_A + alpha * INV_EIGHT.as_scalar() * G_BASEPOINT.as_point().unwrap()),
        );

        transcript.update(&[A]);
        let y = transcript.0;
        if y == RctKey::zero() {
            return Self::try_again(_sv, gamma, V, logM, aL, aL8, aR, aR8);
        }
        transcript = Transcript(RctKey::from_scalar(&private_key_to_scalar(
            &Hash::hash_to_scalar(y.bytes),
        )));
        let z = transcript.0;
        if z == RctKey::zero() {
            return Self::try_again(_sv, gamma, V, logM, aL, aL8, aR, aR8);
        }

        let z_squared = z.as_scalar() * z.as_scalar();

        let mut d: Vec<Scalar> = vec![Scalar::ZERO; MN];
        d[0] = z_squared;
        for i in 1..N {
            d[i] = d[i - 1] * TWO.as_scalar();
        }
        for j in 1..M {
            for i in 0..N {
                d[j * N + i] = d[(j - 1) * N + i] * z_squared;
            }
        }

        let y_powers = Self::vector_of_scalar_powers(&y, MN + 2);

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
            alpha1 += temp2 * item.as_scalar();
        }

        let mut nprime = MN;
        let mut GPrime = vec![EdwardsPoint::identity(); MN];
        let mut HPrime = vec![EdwardsPoint::identity(); MN];
        let mut aprime = vec![Scalar::ZERO; MN];
        let mut bprime = vec![Scalar::ZERO; MN];

        let yinv = y.as_scalar().invert();
        let mut yinvpow: Vec<Scalar> = vec![Scalar::ZERO; MN];
        yinvpow[0] = ONE.as_scalar();
        for i in 0..MN {
            let generators = bulletproofs_generators(b"bulletproof_plus")?;
            GPrime[i] = generators.G[i];
            HPrime[i] = generators.H[i];
            if i > 0 {
                yinvpow[i] = yinvpow[i - 1] * yinv;
            }
            aprime[i] = aL1[i];
            bprime[i] = aR1[i];
        }

        let logN = 6;
        let logMN = logM + logN;

        let mut L: Vec<RctKey> = vec![RctKey::zero(); logMN];
        let mut R: Vec<RctKey> = vec![RctKey::zero(); logMN];
        let mut round = 0;

        while nprime > 1 {
            nprime /= 2;

            let (aprime_left, aprime_right) = aprime.split_at(nprime);
            let (bprime_left, bprime_right) = bprime.split_at(nprime);
            let (GPrime_left, GPrime_right) = GPrime.split_at(nprime);
            let (HPrime_left, HPrime_right) = HPrime.split_at(nprime);
            let cL = Self::weighted_inner_product(aprime_left, bprime_right, &y.as_scalar());
            let vec_scalar: Vec<Scalar> =
                aprime_left.iter().map(|a| a * y_powers[nprime]).collect();
            let cR = Self::weighted_inner_product(&vec_scalar, bprime_left, &y.as_scalar());
            let dL = {
                let mut rng = thread_rng();
                let mut bytes = [0u8; 32];
                rng.fill_bytes(&mut bytes);
                Scalar::from_bytes_mod_order(bytes)
            };
            let dR = {
                let mut rng = thread_rng();
                let mut bytes = [0u8; 32];
                rng.fill_bytes(&mut bytes);
                Scalar::from_bytes_mod_order(bytes)
            };

            L[round] = RctKey::from_point(&Self::compute_LR(
                nprime,
                &yinvpow[nprime],
                GPrime_left,
                HPrime_right,
                aprime_right,
                bprime_left,
                &cL,
                &dL,
            ));

            R[round] = RctKey::from_point(&Self::compute_LR(
                nprime,
                &y_powers[nprime],
                GPrime_right,
                HPrime_left,
                aprime_left,
                bprime_right,
                &cR,
                &dR,
            ));

            transcript.update(&[L[round], R[round]]);
            let challenge = transcript.0;
            if challenge == RctKey::zero() {
                return Self::try_again(_sv, gamma, V, logM, aL, aL8, aR, aR8);
            }

            let challenge_inv = challenge.as_scalar().invert();
            let temp = yinvpow[nprime] * challenge.as_scalar();

            Self::hadamard_fold(&mut GPrime, challenge_inv, temp);
            Self::hadamard_fold(&mut HPrime, challenge.as_scalar(), challenge_inv);

            let temp = challenge_inv * y_powers[nprime];
            let ap1: Vec<Scalar> = aprime_left
                .iter()
                .map(|x| x * challenge.as_scalar())
                .collect();
            let ap2: Vec<Scalar> = aprime_right.iter().map(|x| x * temp).collect();
            aprime = ap1.iter().zip(ap2.iter()).map(|(a, b)| a + b).collect();
            let bp1: Vec<Scalar> = bprime_left.iter().map(|x| x * challenge_inv).collect();
            let bp2: Vec<Scalar> = bprime_right
                .iter()
                .map(|x| x * challenge.as_scalar())
                .collect();
            bprime = bp1.iter().zip(bp2.iter()).map(|(a, b)| a + b).collect();
            let challenge_squared = challenge.as_scalar() * challenge.as_scalar();
            let challenge_squared_inv = challenge_inv * challenge_inv;
            alpha1 += dL * challenge_squared + dR * challenge_squared_inv;
            round += 1;
        }

        let r = {
            let mut rng = thread_rng();
            let mut bytes = [0u8; 32];
            rng.fill_bytes(&mut bytes);
            Scalar::from_bytes_mod_order(bytes)
        };
        let s = {
            let mut rng = thread_rng();
            let mut bytes = [0u8; 32];
            rng.fill_bytes(&mut bytes);
            Scalar::from_bytes_mod_order(bytes)
        };
        let d_ = {
            let mut rng = thread_rng();
            let mut bytes = [0u8; 32];
            rng.fill_bytes(&mut bytes);
            Scalar::from_bytes_mod_order(bytes)
        };
        let eta = {
            let mut rng = thread_rng();
            let mut bytes = [0u8; 32];
            rng.fill_bytes(&mut bytes);
            Scalar::from_bytes_mod_order(bytes)
        };

        let mut A1_data: Vec<(Scalar, EdwardsPoint)> =
            vec![(Scalar::ZERO, EdwardsPoint::identity()); 4];
        A1_data[0] = (r * INV_EIGHT.as_scalar(), GPrime[0]);
        A1_data[1] = (s * INV_EIGHT.as_scalar(), HPrime[0]);
        A1_data[2] = (d_ * INV_EIGHT.as_scalar(), hash_to_point(G_BASEPOINT.bytes));
        let mut temp = r * y.as_scalar() * bprime[0];
        temp += s * y.as_scalar() * aprime[0];
        A1_data[3] = (
            temp * INV_EIGHT.as_scalar(),
            hash_to_point(H_BASEPOINT.bytes),
        );

        let A1 = Self::multiexp(&A1_data);

        let temp = r * y.as_scalar() * s * INV_EIGHT.as_scalar();
        let temp2 = eta * INV_EIGHT.as_scalar();
        let B = RctKey::from_point(
            &(temp2 * G_BASEPOINT.as_point().unwrap() + temp * H_BASEPOINT.as_point().unwrap()),
        );

        transcript.update(&[RctKey::from_point(&A1), B]);
        let e = transcript.0;
        if e == RctKey::zero() {
            return Self::try_again(_sv, gamma, V, logM, aL, aL8, aR, aR8);
        }

        let e_squared = e.as_scalar() * e.as_scalar();
        let r1 = r + aprime[0] * e.as_scalar();
        let s1 = s + bprime[0] * e.as_scalar();
        let d1 = eta + d_ * e.as_scalar() + alpha1 * e_squared;

        Ok(BulletproofPlus {
            V: V.to_vec(),
            A,
            A1: RctKey::from_point(&A1),
            B,
            r1: RctKey::from_scalar(&r1),
            s1: RctKey::from_scalar(&s1),
            d1: RctKey::from_scalar(&d1),
            L,
            R,
        })
    }

    fn hadamard_fold(v: &mut Vec<EdwardsPoint>, a: Scalar, b: Scalar) {
        assert!(v.len() % 2 == 0, "Vector size should be even");
        let sz = v.len() / 2;
        let mut res = vec![EdwardsPoint::identity(); sz];
        for n in 0..sz {
            res[n] = a * v[n] + b * v[sz + n];
        }
        *v = res;
    }

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn compute_LR(
        size: usize,
        y: &Scalar,
        g: &[EdwardsPoint],
        h: &[EdwardsPoint],
        a: &[Scalar],
        b: &[Scalar],
        c: &Scalar,
        d: &Scalar,
    ) -> EdwardsPoint {
        let mut multiexp_data: Vec<(Scalar, EdwardsPoint)> =
            vec![(Scalar::ZERO, EdwardsPoint::identity()); size * 2 + 2];
        let inv_eight = INV_EIGHT.as_scalar();
        for i in 0..size {
            multiexp_data[i * 2] = (a[i] * y * inv_eight, g[i]);
            multiexp_data[i * 2 + 1] = (b[i] * inv_eight, h[i]);
        }
        multiexp_data[size * 2] = (*c * inv_eight, hash_to_point(H_BASEPOINT.bytes));
        multiexp_data[size * 2 + 1] = (*d * inv_eight, hash_to_point(G_BASEPOINT.bytes));
        Self::multiexp(&multiexp_data)
    }

    fn weighted_inner_product(a: &[Scalar], b: &[Scalar], y: &Scalar) -> Scalar {
        assert_eq!(a.len(), b.len(), "expected a and b to be the same length");
        let mut res = Scalar::ZERO;
        let mut y_power = Scalar::ONE;
        for i in 0..a.len() {
            y_power *= y;
            res += a[i] * b[i] * y_power;
        }
        res
    }

    fn vector_of_scalar_powers(base: &RctKey, n: usize) -> Vec<Scalar> {
        assert!(n > 0, "expected n > 0");
        let mut powers = vec![Scalar::ONE];
        if n == 1 {
            return powers;
        }
        powers.push(base.as_scalar());
        for i in 2..n {
            powers.push(powers[i - 1] * base.as_scalar());
        }
        powers
    }

    fn vector_exponent(a: &[RctKey], b: &[RctKey]) -> Result<EdwardsPoint, Error> {
        if a.len() != b.len() {
            return Err(Error::AnyhowError(anyhow!(
                "a and b must be the same length"
            )));
        }
        let generators = bulletproofs_generators(b"bulletproof_plus")?;
        let mut sum = EdwardsPoint::identity();
        for i in 0..a.len() {
            sum += a[i].as_scalar() * generators.G[i] + b[i].as_scalar() * generators.H[i];
        }
        Ok(sum)
    }

    fn multiexp(data: &[(Scalar, EdwardsPoint)]) -> EdwardsPoint {
        data.iter()
            .fold(EdwardsPoint::identity(), |acc, (s, p)| acc + s * p)
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

    pub fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(RCTType::Null),
            1 => Ok(RCTType::Full),
            2 => Ok(RCTType::Simple),
            3 => Ok(RCTType::Bulletproof),
            4 => Ok(RCTType::Bulletproof2),
            5 => Ok(RCTType::CLSAG),
            6 => Ok(RCTType::BulletproofPlus),
            _ => Err(Error::AnyhowError(anyhow!("Invalid RCTType"))),
        }
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RctConfig {
    pub range_proof_type: RangeProofType,
    pub bp_version: isize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum RangeProofType {
    RangeProofBorromean,
    RangeProofBulletproof,
    RangeProofMultiOutputBulletproof,
    RangeProofPaddedBulletproof,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct MgSig {
    pub ss: Vec<Vec<RctKey>>,
    pub cc: RctKey,
    pub II: Vec<RctKey>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct Clsag {
    pub s: Vec<RctKey>,
    pub c1: RctKey,
    pub I: RctKey,
    pub D: RctKey,
}

impl Clsag {
    #[allow(non_snake_case)]
    pub fn new_proof(
        message: &RctKey,
        pubs: &Vec<CtKey>,
        in_sk: &CtKey,
        a: &RctKey,
        Cout: &RctKey,
        index: usize,
    ) -> Result<Self, Error> {
        let cols = pubs.len();
        if cols == 0 {
            return Err(Error::AnyhowError(anyhow!("Empty pubs")));
        }

        let mut P: Vec<RctKey> = Vec::new();
        let mut C: Vec<RctKey> = Vec::new();
        let mut C_nonzero: Vec<RctKey> = Vec::new();
        for k in pubs {
            P.push(k.dest);
            C_nonzero.push(k.mask);
            let tmp = k.mask.as_point()? - Cout.as_point()?;
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

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    pub fn generate(
        message: &RctKey,
        P: &[RctKey],
        p: &RctKey,
        C: &[RctKey],
        z: &RctKey,
        C_nonzero: &[RctKey],
        C_offset: &RctKey,
        l: usize,
    ) -> Result<Self, Error> {
        let mut sig = Clsag::default();
        let n = P.len();
        if n != C.len() || n != C_nonzero.len() {
            return Err(Error::AnyhowError(anyhow!("Mismatched vector sizes")));
        }
        if l >= n {
            return Err(Error::AnyhowError(anyhow!("Signing index out of range")));
        }

        let H = hash_to_point(P[l].bytes);

        let mut random_bytes = [0u8; 32];
        thread_rng().fill_bytes(&mut random_bytes);
        let mut a = RctKey::from_scalar(&Scalar::from_bytes_mod_order(random_bytes));
        let aG = RctKey::from_point(&(a.as_scalar() * G_BASEPOINT.as_point()?));
        let aH = RctKey::from_point(&(a.as_scalar() * H));
        sig.I = RctKey::from_point(&(p.as_scalar() * H));
        let D = RctKey::from_point(&(z.as_scalar() * H));
        sig.D = RctKey::from_point(&(D.as_point()? * INV_EIGHT.as_scalar()));

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

        let mu_P = RctKey::from_scalar(&private_key_to_scalar(&Hash::hash_to_scalar(
            mu_P_to_hash
                .iter()
                .flat_map(|x| x.bytes)
                .collect::<Vec<u8>>()
                .as_slice(),
        )));
        let mu_C = RctKey::from_scalar(&private_key_to_scalar(&Hash::hash_to_scalar(
            mu_C_to_hash
                .iter()
                .flat_map(|x| x.bytes)
                .collect::<Vec<u8>>()
                .as_slice(),
        )));

        let mut c_to_hash = vec![RctKey::default(); 2 * n + 5];
        c_to_hash[0] = RctKey::zero();
        c_to_hash[0].bytes[0..HASH_KEY_CLSAG_ROUND.len()].copy_from_slice(HASH_KEY_CLSAG_ROUND);
        c_to_hash[1..(n + 1)].copy_from_slice(&P[0..n]);
        c_to_hash[(1 + n)..(n + 1 + n)].copy_from_slice(&C_nonzero[0..n]);
        c_to_hash[2 * n + 1] = *C_offset;
        c_to_hash[2 * n + 2] = *message;
        c_to_hash[2 * n + 3] = aG;
        c_to_hash[2 * n + 4] = aH;

        let c = RctKey::from_scalar(&private_key_to_scalar(&Hash::hash_to_scalar(
            c_to_hash
                .iter()
                .flat_map(|x| x.bytes)
                .collect::<Vec<u8>>()
                .as_slice(),
        )));

        let mut i = (l + 1) % n;
        if i == 0 {
            sig.c1 = c;
        }

        sig.s = vec![RctKey::default(); n];

        while i != l {
            sig.s[i] = RctKey::from_scalar(&{
                let mut rng = thread_rng();
                let mut bytes = [0u8; 32];
                rng.fill_bytes(&mut bytes);
                Scalar::from_bytes_mod_order(bytes)
            });
            let c_p = mu_P.as_scalar() * c.as_scalar();
            let c_c = mu_C.as_scalar() * c.as_scalar();

            let L = sig.s[i].as_scalar() * G_BASEPOINT.as_point()?
                + c_p * P[i].as_point()?
                + c_c * C[i].as_point()?;

            let hash8_p3 = hash_to_point(P[i].bytes);
            let R = sig.s[i].as_scalar() * hash8_p3
                + c_p * sig.I.as_point()?
                + c_c * sig.D.as_point()?;

            c_to_hash[2 * n + 3] = RctKey::from_point(&L);
            c_to_hash[2 * n + 4] = RctKey::from_point(&R);

            let _c_new = RctKey::from_scalar(&private_key_to_scalar(&Hash::hash_to_scalar(
                c_to_hash
                    .iter()
                    .flat_map(|x| x.bytes)
                    .collect::<Vec<u8>>()
                    .as_slice(),
            )));
            i = (i + 1) % n;
            if i == 0 {
                sig.c1 = c;
            }
        }

        let s0_p_mu_P = mu_P.as_scalar() * p.as_scalar();
        let s0_add_z_mu_C = mu_C.as_scalar() * z.as_scalar() + s0_p_mu_P;
        let scalar = a.as_scalar() - c.as_scalar() * s0_add_z_mu_C;
        sig.s[l] = RctKey::from_scalar(&scalar);
        a.zeroize();

        Ok(sig)
    }
}

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
        serialized.serialize_field("nbp", &{ nbp })?;
        for (i, bp) in self.bulletproofs_plus.iter().enumerate() {
            serialized.serialize_field(&format!("bpp[{i}]"), bp)?;
        }
        for (i, clsag) in self.CLSAGs.iter().enumerate() {
            serialized.serialize_vector(&format!("CLSAGs[{i}].s"), &clsag.s)?;
            serialized.serialize_field(&format!("CLSAGs[{i}].c1"), &clsag.c1)?;
            serialized.serialize_field(&format!("CLSAGs[{i}].D"), &clsag.D)?;
        }
        for (i, pseudo_out) in self.pseudo_outs.iter().enumerate() {
            serialized.serialize_field(&format!("pseudoOuts[{i}]"), pseudo_out)?;
        }
        Ok(())
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct RctSig {
    pub base: RctSigBase,
    pub p: RctSigPrunable,
}

impl RctSig {
    #[allow(non_snake_case)]
    pub fn pre_mlsag_hash(&self) -> Result<RctKey, Error> {
        let base = self.base.clone();
        let mut hashes: Vec<RctKey> = Vec::new();
        hashes.push(base.message);
        if base.mix_ring.is_empty() {
            return Err(Error::AnyhowError(anyhow!("Empty mix_ring")));
        }

        let inputs: usize = if base.rct_type.is_rct_simple() {
            base.mix_ring.len()
        } else {
            base.mix_ring[0].len()
        };

        if base.pseudo_outs.len() != inputs {
            return Err(Error::AnyhowError(anyhow!("Invalid pseudo_outs size")));
        }

        let mut serialized_rctsig_base = SerializedArchive::new();
        base.do_serialize(&mut serialized_rctsig_base)?;
        let base_str_hash = keccak256(serialized_rctsig_base.json_stream.as_bytes());
        hashes.push(RctKey::from_slice(&base_str_hash));

        let mut key_vec: Vec<RctKey> = Vec::new();
        if base.rct_type != RCTType::BulletproofPlus {
            return Err(Error::AnyhowError(anyhow!(
                "Only supporting BulletproofPlus RCT type"
            )));
        }
        for p in &self.p.bulletproofs_plus {
            key_vec.push(p.A);
            key_vec.push(p.A1);
            key_vec.push(p.B);
            key_vec.push(p.r1);
            key_vec.push(p.s1);
            key_vec.push(p.d1);
            for pL in &p.L {
                key_vec.push(*pL);
            }
            for pR in &p.R {
                key_vec.push(*pR);
            }
        }
        let key_vec_data: Vec<u8> = key_vec.iter().flat_map(|k| k.bytes).collect();
        let key_vec_hash = keccak256(&key_vec_data);
        hashes.push(RctKey::from_slice(&key_vec_hash));
        let hashes_data: Vec<u8> = hashes.iter().flat_map(|k| k.bytes).collect();
        let mlsag_prehash = keccak256(&hashes_data);
        Ok(RctKey::from_slice(&mlsag_prehash))
    }

    #[allow(non_snake_case)]
    pub fn verify_rct_clsag_simple(&self, message: &RctKey, i: usize) -> Result<bool, Error> {
        let sig = self
            .p
            .CLSAGs
            .get(i)
            .ok_or_else(|| Error::AnyhowError(anyhow!("Invalid CLSAG index")))?
            .clone();
        let pubs = self
            .base
            .mix_ring
            .get(i)
            .ok_or_else(|| Error::AnyhowError(anyhow!("Invalid mix_ring index")))?
            .clone();
        let C_offset = *self
            .base
            .pseudo_outs
            .get(i)
            .ok_or_else(|| Error::AnyhowError(anyhow!("Invalid pseudo_outs index")))?;
        let n = pubs.len();

        if n < 1 {
            return Err(Error::AnyhowError(anyhow!("Empty pubs")));
        }
        if n != sig.s.len() {
            return Err(Error::AnyhowError(anyhow!(
                "Signature scalar vector is the wrong size"
            )));
        }
        for s in &sig.s {
            if Scalar::from_canonical_bytes(s.bytes).is_none().into() {
                return Err(Error::AnyhowError(anyhow!("Bad signature scalar")));
            }
        }
        if Scalar::from_canonical_bytes(sig.c1.bytes).is_none().into() {
            return Err(Error::AnyhowError(anyhow!("Bad signature commitment")));
        }
        if sig.I == RctKey::default() {
            return Err(Error::AnyhowError(anyhow!("Bad key image")));
        }

        let D_8 = RctKey::from_point(&(sig.D.as_point()? * Scalar::from(8u8)));
        if D_8 == RctKey::identity() {
            return Err(Error::AnyhowError(anyhow!("Bad auxiliary key image")));
        }

        let mut mu_P_to_hash: Vec<RctKey> = vec![RctKey::default(); 2 * n + 4];
        let mut mu_C_to_hash: Vec<RctKey> = vec![RctKey::default(); 2 * n + 4];
        mu_P_to_hash[0] = RctKey::zero();
        mu_P_to_hash[0].bytes[0..HASH_KEY_CLSAG_AGG_0.len()].copy_from_slice(HASH_KEY_CLSAG_AGG_0);
        mu_C_to_hash[0] = RctKey::zero();
        mu_C_to_hash[0].bytes[0..HASH_KEY_CLSAG_AGG_1.len()].copy_from_slice(HASH_KEY_CLSAG_AGG_1);
        for j in 0..n {
            mu_P_to_hash[j + 1] = pubs[j].dest;
            mu_C_to_hash[j + 1] = pubs[j].dest;
            mu_P_to_hash[j + n + 1] = pubs[j].mask;
            mu_C_to_hash[j + n + 1] = pubs[j].mask;
        }
        mu_P_to_hash[2 * n + 1] = sig.I;
        mu_P_to_hash[2 * n + 2] = sig.D;
        mu_P_to_hash[2 * n + 3] = C_offset;
        mu_C_to_hash[2 * n + 1] = sig.I;
        mu_C_to_hash[2 * n + 2] = sig.D;
        mu_C_to_hash[2 * n + 3] = C_offset;

        let mu_P = RctKey::from_scalar(&private_key_to_scalar(&Hash::hash_to_scalar(
            mu_P_to_hash
                .iter()
                .flat_map(|x| x.bytes)
                .collect::<Vec<u8>>()
                .as_slice(),
        )));
        let mu_C = RctKey::from_scalar(&private_key_to_scalar(&Hash::hash_to_scalar(
            mu_C_to_hash
                .iter()
                .flat_map(|x| x.bytes)
                .collect::<Vec<u8>>()
                .as_slice(),
        )));

        let mut c_to_hash: Vec<RctKey> = vec![RctKey::default(); 2 * n + 5];
        c_to_hash[0] = RctKey::zero();
        c_to_hash[0].bytes[0..HASH_KEY_CLSAG_ROUND.len()].copy_from_slice(HASH_KEY_CLSAG_ROUND);
        for j in 0..n {
            c_to_hash[j + 1] = pubs[j].dest;
            c_to_hash[j + n + 1] = pubs[j].mask;
        }
        c_to_hash[2 * n + 1] = C_offset;
        c_to_hash[2 * n + 2] = *message;

        let mut c = sig.c1;
        let mut i = 0;

        while i < n {
            let c_p = mu_P.as_scalar() * c.as_scalar();
            let c_c = mu_C.as_scalar() * c.as_scalar();

            let L = sig.s[i].as_scalar() * G_BASEPOINT.as_point()?
                + c_p * pubs[i].dest.as_point()?
                + c_c * (pubs[i].mask.as_point()? - C_offset.as_point()?);

            let hash8_p3 = hash_to_point(pubs[i].dest.bytes);
            let R =
                sig.s[i].as_scalar() * hash8_p3 + c_p * sig.I.as_point()? + c_c * D_8.as_point()?;

            c_to_hash[2 * n + 3] = RctKey::from_point(&L);
            c_to_hash[2 * n + 4] = RctKey::from_point(&R);

            let c_new = RctKey::from_scalar(&private_key_to_scalar(&Hash::hash_to_scalar(
                c_to_hash
                    .iter()
                    .flat_map(|x| x.bytes)
                    .collect::<Vec<u8>>()
                    .as_slice(),
            )));
            if c_new == RctKey::zero() {
                return Err(Error::AnyhowError(anyhow!("Bad signature hash")));
            }
            c = c_new;
            i += 1;
        }
        Ok(c.as_scalar() == sig.c1.as_scalar())
    }

    pub fn verify_rct_simple(&self) -> Result<bool, Error> {
        if !self.base.rct_type.is_rct_simple() {
            return Err(Error::AnyhowError(anyhow!("Non-simple RctSig")));
        }
        if self.p.pseudo_outs.len() != self.base.mix_ring.len() {
            return Err(Error::AnyhowError(anyhow!(
                "Mismatched sizes of pseudoOuts and mixRing"
            )));
        }

        let message = self.pre_mlsag_hash()?;
        for i in 0..self.base.mix_ring.len() {
            if !self.base.rct_type.is_rct_clsag() {
                return Err(Error::AnyhowError(anyhow!("Only handling clsag rct type")));
            }
            if !Self::verify_rct_clsag_simple(self, &message, i)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    pub fn generate_rct_simple(
        message: &RctKey,
        in_sk: &[CtKey],
        destinations: &[RctKey],
        in_amounts: &[u64],
        out_amounts: &Vec<u64>,
        txn_fee: u64,
        mix_ring: &[Vec<CtKey>],
        amount_keys: &[RctKey],
        indices: &[u64],
        out_sk: &mut Vec<CtKey>,
        rct_config: RctConfig,
    ) -> Result<Self, Error> {
        if in_amounts.is_empty() {
            return Err(Error::AnyhowError(anyhow!("Empty in_amounts")));
        }
        if in_amounts.len() != in_sk.len() {
            return Err(Error::AnyhowError(anyhow!(
                "Mismatched in_amounts and in_sk"
            )));
        }
        if out_amounts.len() != destinations.len() {
            return Err(Error::AnyhowError(anyhow!(
                "Mismatched out_amounts and destinations"
            )));
        }
        if amount_keys.len() != destinations.len() {
            return Err(Error::AnyhowError(anyhow!(
                "Mismatched amount_keys and destinations"
            )));
        }
        if indices.len() != in_sk.len() {
            return Err(Error::AnyhowError(anyhow!("Mismatched indices and in_sk")));
        }
        if mix_ring.len() != in_sk.len() {
            return Err(Error::AnyhowError(anyhow!("Mismatched mix_ring and in_sk")));
        }
        for (n, idx) in indices.iter().enumerate() {
            if *idx >= mix_ring[n].len() as u64 {
                return Err(Error::AnyhowError(anyhow!("Bad index into mix_ring")));
            }
        }

        let rct_type = match rct_config.bp_version {
            0 | 4 => RCTType::BulletproofPlus,
            _ => {
                return Err(Error::AnyhowError(anyhow!(
                    "Unsupported bulletproof version"
                )))
            }
        };

        let mut rct_out_pk: Vec<CtKey> = vec![CtKey::default(); destinations.len()];
        let mut rct_ecdh_info: Vec<EcdhTuple> = vec![EcdhTuple::default(); destinations.len()];
        out_sk.resize(destinations.len(), CtKey::default());

        for i in 0..destinations.len() {
            rct_out_pk[i].dest = destinations[i];
        }

        let mut masks = vec![RctKey::identity(); out_amounts.len()];
        for i in 0..masks.len() {
            masks[i] = RctKey::gen_commitment_mask(&amount_keys[i]);
        }
        let proof = BulletproofPlus::new_proof(out_amounts, &masks)?;
        let C = proof.V.clone();
        let rct_p_bulletproofs_plus = vec![proof];

        for i in 0..out_amounts.len() {
            rct_out_pk[i].mask = C[i];
            out_sk[i].mask = masks[i];
            rct_ecdh_info[i].mask = out_sk[i].mask;
            rct_ecdh_info[i].amount.bytes[0..8].copy_from_slice(&out_amounts[i].to_le_bytes());
            rct_ecdh_info[i].encode(&amount_keys[i]);
        }

        let mut pseudo_outs: Vec<RctKey> = vec![RctKey::zero(); in_amounts.len()];
        let mut sum_out = RctKey::zero();
        for (i, _) in out_sk.iter().enumerate() {
            sum_out = RctKey::from_scalar(&(out_sk[i].mask.as_scalar() + sum_out.as_scalar()));
        }

        let mut sum_pouts = RctKey::zero();
        let mut a: Vec<RctKey> = vec![RctKey::zero(); in_amounts.len()];
        for i in 0..in_amounts.len() - 1 {
            a[i] = RctKey::from_scalar(&{
                let mut rng = thread_rng();
                let mut bytes = [0u8; 32];
                rng.fill_bytes(&mut bytes);
                Scalar::from_bytes_mod_order(bytes)
            });
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
            mix_ring: mix_ring.to_owned().clone(),
            pseudo_outs,
            ecdh_info: rct_ecdh_info,
            out_pk: rct_out_pk,
        };

        let mut rv = RctSig {
            base: rct_base,
            p: rct_prunable,
        };

        let full_message = rv.pre_mlsag_hash()?;

        for i in 0..in_amounts.len() {
            if rv.base.rct_type.is_rct_clsag() {
                rv.p.CLSAGs.push(Clsag::new_proof(
                    &full_message,
                    &rv.base.mix_ring[i],
                    &in_sk[i],
                    &a[i],
                    &rv.base.pseudo_outs[i],
                    indices[i] as usize,
                )?);
            } else {
                return Err(Error::AnyhowError(anyhow!("Only handling BulletproofPlus")));
            }
        }
        Ok(rv)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MultiSigOut {
    c: Vec<RctKey>,
    mu_p: Vec<RctKey>,
    c0: Vec<RctKey>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct MultiSigkLRki {
    k: RctKey,
    L: RctKey,
    R: RctKey,
    ki: RctKey,
}

fn private_key_to_scalar(pk: &monero::PrivateKey) -> Scalar {
    Scalar::from_bytes_mod_order(pk.to_bytes())
}
