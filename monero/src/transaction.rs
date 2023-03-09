use std::collections::HashSet;
use std::fmt::Debug;
use std::str::Utf8Error;

use curve25519_dalek::scalar::Scalar;
use thiserror::Error;

use crate::key_image::KeyDerivation;
use crate::monero_lws::{UnspentOutput, FAKE_OUTPUTS_COUNT};
use crate::rct_types::{
    CtKey, MultiSigOut, MultiSigkLRki, RCTType, RangeProofType, RctConfig, RctKey, RctSig,
};
use crate::varint::VarIntEncoding;
use crate::{
    fee_utils, keccak256, key_image, payment_id, public_key, Address, AddressType, DoSerialize,
    KeyImage, MoneroAmount, MoneroPrivateKeys, PaymentId, PrivateKey, PublicKey, SerializedArchive,
    SubaddressIndex, VarInt,
};

const TX_EXTRA_TAG_PUBKEY: u8 = 0x01;
const HF_VERSION_VIEW_TAGS: u8 = 15;
const HF_VERSION_PER_BYTE_FEE: u8 = 8;
const EXPECTED_MINIMUM_HF_VERSION: u8 = 15;

/// TODO(#68): Figure out what this CryptoHash is and implement it
#[derive(Debug, Clone)]
pub struct CryptoHash;

/// A transaction that is pending to be sent to the network.
/// Implemented in Rust based on Monero's pending_tx struct
/// **Source** <`monero/src/wallet/wallet2.h`>(https://github.com/monero-project/monero/blob/9f5c7209a0a9c4ed3b8a0c00dda9dc885f400fed/src/wallet/wallet2.h#L630-L670)
#[derive(Debug, Clone)]
pub struct PendingTx {
    pub tx: Transaction,
    pub dust: u64,
    pub fee: u64,
    pub dust_added_to_fee: bool,
    pub change_dts: TxDestinationEntry,
    pub selected_transfers: Vec<usize>,
    pub key_images: String,
    pub tx_key: PrivateKey,
    pub additional_tx_keys: Vec<PrivateKey>,
    pub dests: Vec<TxDestinationEntry>,
    pub multisig_sigs: Vec<MultiSigSignature>,
    pub multisig_tx_key_entropy: Option<PrivateKey>,
    pub construction_data: TxConstructionData,
}

/// Implemented in Rust based on Monero's tx_construction_data struct
/// **Source** <`monero/src/wallet/wallet2.h`>(https://github.com/monero-project/monero/blob/9f5c7209a0a9c4ed3b8a0c00dda9dc885f400fed/src/wallet/wallet2.h#L539-L552)
#[derive(Debug, Clone)]
pub struct TxConstructionData {
    pub sources: Vec<TxSourceEntry>,
    pub change_dts: TxDestinationEntry,
    pub splitted_dsts: Vec<TxDestinationEntry>,
    pub selected_transfers: Vec<usize>,
    pub extra: RawExtraField,
    pub unlock_time: UnlockTime,
    pub use_rct: bool,
    pub rct_config: RctConfig,
    pub use_view_tags: bool,
    pub dests: Vec<TxDestinationEntry>,
    pub subadr_account: u32,
    pub subaddr_indices: HashSet<u32>,
}

/// Struct to represent the Unlock Time of a transaction, associated with a u64
/// data type value
#[derive(Clone, Debug, Default, Copy)]
pub struct UnlockTime(pub u64);

/// Implemented in Rust based on Monero's output_entry type
/// **Source** <`monero/src/cryptonote_core/cryptonote_tx_utils.h`>(https://github.com/monero-project/monero/blob/451ff7bd91c68cc9861711fbd45587a388df77dc/src/cryptonote_core/cryptonote_tx_utils.h#L44)
#[derive(Clone, Default, Debug)]
pub struct OutputEntry(pub u64, pub CtKey);

/// Implemented in Rust based on Monero's cryptonote::tx_source_entry structT
/// **Source** <`monero/src/cryptonote_core/cryptonote_tx_utils.h`>(https://github.com/monero-project/monero/blob/451ff7bd91c68cc9861711fbd45587a388df77dc/src/cryptonote_core/cryptonote_tx_utils.h#L42-L54)
#[allow(non_snake_case)]
#[derive(Clone, Default, Debug)]
pub struct TxSourceEntry {
    pub outputs: Vec<OutputEntry>,
    pub real_output: u64,
    pub real_out_tx_key: PublicKey,
    pub real_out_additional_tx_keys: Vec<PublicKey>,
    pub real_output_in_tx_index: u64,
    pub amount: u64,
    pub rct: bool,
    pub mask: RctKey,
    pub multisig_kLRki: MultiSigkLRki,
}

/// Implemented in Rust based on Monero's cryptonote::tx_destination_entry
/// struct
/// **Source** <`monero/src/cryptonote_core/cryptonote_tx_utils.h`>(https://github.com/monero-project/monero/blob/451ff7bd91c68cc9861711fbd45587a388df77dc/src/cryptonote_core/cryptonote_tx_utils.h#L74-L108)
#[derive(Debug, Clone)]
pub struct TxDestinationEntry {
    pub amount: u64,
    pub addr: Address,
}

/// Implemented in Rust based on Monero's transfer_details struct
/// **Source** <`monero/src/wallet/wallet2.h`>(https://github.com/monero-project/monero/blob/9f5c7209a0a9c4ed3b8a0c00dda9dc885f400fed/src/wallet/wallet2.h#L326-L347)
/// Is this actually needed? TODO(#68)
pub struct TransferDetails {
    pub block_height: u64,
    pub tx_prefix: TransactionPrefix,
    pub txid: CryptoHash,
    pub internal_output_index: u64,
    pub global_output_index: u64,
    pub spent: bool,
    pub frozen: bool,
    pub spent_height: u64,
    pub key_image: KeyImage,
    pub mask: RctKey,
    pub amount: MoneroAmount,
    pub key_image_known: bool,
    pub key_image_request: bool,
    pub pk_index: u64,
    pub subaddr_index: SubaddressIndex,
    pub key_image_partial: bool,
    pub multi_sig_k: Vec<RctKey>,
    // pub multi_sig_info: Vec<MultiSigInfo>, //TODO(#86) Implement multisig for Monero
    pub uses: Vec<(u64, CryptoHash)>,
}

/// Implemented in Rust based on Monero's signature class
/// **Source** <`monero/src/crypto/crypto.h`>(https://github.com/monero-project/monero/blob/ad80aa0f656fd8a9d168b0af4fc6dcf927da5d4b/src/crypto/crypto.h#L98-L101)
#[derive(Clone, Debug)]
pub struct Signature {
    pub c: Scalar,
    pub r: Scalar,
}

/// Implemented in Rust based on Monero's cryptonote::multisig_sig struct
/// **Source** <`monero/src/wallet/wallet2.h`> https://github.com/monero-project/monero/blob/9f5c7209a0a9c4ed3b8a0c00dda9dc885f400fed/src/wallet/wallet2.h#L601-L628)
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct MultiSigSignature {
    pub sigs: RctSig,
    pub ignore: HashSet<PublicKey>,
    pub used_L: HashSet<RctKey>,
    pub signing_keys: HashSet<PublicKey>,
    pub msout: MultiSigOut,
    pub total_alpha_G: Vec<Vec<RctKey>>,
    pub total_alpha_H: Vec<Vec<RctKey>>,
    pub c_0: Vec<RctKey>,
    pub s: Vec<RctKey>,
}

/// Struct representing a Monero transaction
/// Implemented in Rust based on Monero's cryptonote::transaction struct
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L204-L450)
#[derive(Clone, Debug)]
pub struct Transaction {
    pub prefix: TransactionPrefix,
    pub hash_valid: bool,
    pub prunable_hash_valid: bool,
    pub blob_size_valid: bool,
    pub signatures: Vec<Signature>, /* length of signatures should be the same as length of vin
                                     * in the prefix */
    pub rct_signatures: RctSig,
}

impl Transaction {
    pub fn validate(&self) -> Result<bool, anyhow::Error> {
        self.rct_signatures.verify_rct_simple()
    }
}

impl DoSerialize for Transaction {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        SerializedArchive::begin_object(serialized);
        self.prefix.do_serialize(serialized)?;
        // Currently feature hardcoded for only handling v2 transactions
        assert!(self.prefix.version == 2);
        serialized.add_tag("rct_signatures");
        if !self.prefix.vin.is_empty() {
            serialized.begin_object();
            self.rct_signatures.base.do_serialize(serialized)?;
            serialized.end_object();
        }
        serialized.add_tag("rctsig_prunable");
        serialized.begin_object();
        self.rct_signatures.p.do_serialize(serialized)?;
        serialized.end_object();
        serialized.end_object();
        Ok(())
    }
}

/// Struct representing the extra field of a Monero transaction as raw bytes
#[derive(Debug, Clone, Default)]
pub struct RawExtraField(pub Vec<u8>);

impl DoSerialize for RawExtraField {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        let n = self.0.len();
        serialized.begin_array_with_size(n)?;
        for (i, byte) in self.0.iter().enumerate() {
            VarInt(*byte).do_serialize(serialized)?;
            if i < n - 1 {
                serialized.delimit_array();
            }
        }
        serialized.end_array();
        Ok(())
    }
}

/// Struct containing transaction prefix info including version, unlock time,
/// inputs and outuputs and extra Implemented in Rust based on Monero's
/// cryptonote::transaction_prefix struct
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L170-L202)
#[derive(Debug, Clone)]
pub struct TransactionPrefix {
    pub version: usize,
    pub unlock_time: UnlockTime,
    pub vin: Vec<TxInToKey>,
    pub vout: Vec<TxOut>,
    pub extra: RawExtraField,
}

impl DoSerialize for TransactionPrefix {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        if self.version == 0 || self.version > 2 {
            return Err(anyhow::Error::new(Error::InvalidVersion(self.version)));
        }
        // version varint_field
        let version = VarInt(self.version as u64);
        serialized.add_tag("version");
        version.do_serialize(serialized)?;
        // unlock_time varint_field
        serialized.add_tag("unlock_time");
        let unlock_time = VarInt(self.unlock_time.0);
        unlock_time.do_serialize(serialized)?;
        // vin, field
        serialized.serialize_vector_variant("vin", &self.vin, TxInVariant::TxInToKey)?;
        // vout, field
        serialized.serialize_vector("vout", &self.vout)?;
        // extra, field
        serialized.serialize_field("extra", &self.extra)?;
        Ok(())
    }
}

/// Implemented in Rust based on Monero's txin_v
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L152)
pub enum TxInVariant {
    TxInGen,
    TxInToScript,
    TxInToScriptHash,
    TxInToKey,
}

pub trait Variant {
    fn variant_tag(&self) -> u8;
}

impl Variant for TxInVariant {
    fn variant_tag(&self) -> u8 {
        match self {
            TxInVariant::TxInGen => 0xff,
            TxInVariant::TxInToScript => 0x0,
            TxInVariant::TxInToScriptHash => 0x01,
            TxInVariant::TxInToKey => 0x02,
        }
    }
}

/// Implemented in Rust based on Monero's txin_gen
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L101-L108)
pub struct TxInGen {
    pub height: usize,
}

/// Implemented in Rust based on Monero's txin_to_script
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L110-L121)
pub struct TxInToScript {
    pub prev: CryptoHash,
    pub prev_out: usize,
    pub sigset: Vec<u8>,
}

/// Implemented in Rust based on Monero's txin_to_script_hash
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L123-L136)
pub struct TxInToScriptHash {
    pub prev: CryptoHash,
    pub prevout: usize,
    pub script: TxOutToScript,
    pub sigset: Vec<u8>,
}

/// Implemented in Rust based on Monero's txin_to_key
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L138-L149)
#[derive(Debug, Clone)]
pub struct TxInToKey {
    pub amount: u64,
    pub key_offsets: Vec<VarInt<u64>>,
    pub k_image: KeyImage, // for double spending protection
}

impl TxInToKey {
    pub fn absolute_output_offsets_to_relative(key_offsets: &mut Vec<u64>) {
        if key_offsets.is_empty() {
            return;
        }

        // should actually already be sorted but just in case
        key_offsets.sort();

        for i in (1..key_offsets.len()).rev() {
            key_offsets[i] -= key_offsets[i - 1];
        }
    }
}

impl DoSerialize for TxInToKey {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.begin_object();
        serialized.add_tag("key");
        serialized.begin_object();
        serialized.serialize_field("amount", &VarInt(self.amount))?;
        serialized.serialize_vector("key_offsets", &self.key_offsets)?;
        serialized.serialize_field("k_image", &self.k_image)?;
        serialized.end_object();
        serialized.end_object();
        Ok(())
    }
}

/// Implemented in Rust based on Monero's tx_out struct
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L157-168)
#[derive(Debug, Clone)]
pub struct TxOut {
    pub amount: u64,
    // Right now hardcoding to using TxOutToKey
    pub target: TxOutTargetVariant,
}

impl DoSerialize for TxOut {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.begin_object();
        serialized.serialize_field("amount", &VarInt(self.amount))?;
        serialized.serialize_field("target", &self.target)?;
        serialized.end_object();
        Ok(())
    }
}
/// Implemented in Rust based on Monero's txout_target_v
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L154)
#[derive(Debug, Clone)]
pub enum TxOutTargetVariant {
    ToScript(TxOutToScript),
    ToScriptHash(TxOutToScriptHash),
    ToKey(TxOutToKey),
    ToTaggedKey(TxOutToTaggedKey),
}

impl Variant for TxOutTargetVariant {
    fn variant_tag(&self) -> u8 {
        use TxOutTargetVariant::*;
        match self {
            ToScript(_) => 0x0,
            ToScriptHash(_) => 0x01,
            ToKey(_) => 0x02,
            ToTaggedKey(_) => 0x03,
        }
    }
}

impl DoSerialize for TxOutTargetVariant {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        // Tag the variant type
        let variant_tag = self.variant_tag();
        serialized.data.push(variant_tag);
        serialized.begin_object();
        use TxOutTargetVariant::*;
        match self {
            ToKey(v) => v.do_serialize(serialized)?,
            ToTaggedKey(v) => {
                serialized.add_tag("tagged_key");
                serialized.begin_object();
                v.do_serialize(serialized)?;
                serialized.end_object();
            }
            _ => return Err(anyhow::Error::new(Error::UnsupportedTxOutTargetVariant)),
        }
        serialized.end_object();
        Ok(())
    }
}

/// Implemented in Rust based on Monero's txout_to_script
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L61-70)
#[derive(Debug, Clone)]
pub struct TxOutToScript {
    pub keys: Vec<PublicKey>,
    pub script: Vec<u8>,
}

/// Implemented in Rust based on Monero's txout_to_script_hash
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L72-75)
#[derive(Debug, Clone)]
pub struct TxOutToScriptHash {
    pub hash: CryptoHash,
}

/// Implemented in Rust based on Monero's txout_to_key
/// Used if outputs <= HF_VERSION_VIEW_TAGS
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#L77-L83)
#[derive(Debug, Clone)]
pub struct TxOutToKey {
    pub key: PublicKey,
}

impl DoSerialize for TxOutToKey {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.serialize_field("key", &self.key)?;
        Ok(())
    }
}

/// input context
pub struct InputGenerationContext {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
}

#[derive(Debug, Clone)]
pub struct ViewTag(pub u8);

#[derive(Debug, Clone, Default)]
struct ViewTagBuf {
    salt: [u8; 8],
    derivation: KeyDerivation,
    output_index: Vec<u8>,
}

impl ViewTagBuf {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.salt);
        buf.extend_from_slice(&self.derivation.as_slice());
        buf.extend_from_slice(&self.output_index);
        buf
    }
}

impl ViewTag {
    pub fn size_of() -> usize {
        1
    }

    pub fn derive(derivation: &KeyDerivation, output_index: u64) -> Self {
        let mut buf = ViewTagBuf::default();
        buf.salt.copy_from_slice("view_tag".as_bytes());
        buf.derivation = derivation.clone();
        buf.output_index = VarInt(output_index).encode_to_bytes();

        // view_tag_ful = H[salt|derivation|output_index]
        let view_tag_full = keccak256(&buf.to_bytes());
        assert!(ViewTag::size_of() < view_tag_full.len());
        let view_tag = ViewTag(view_tag_full[0]);
        view_tag
    }
}

/// Implemented in Rust based on Monero's txout_to_key
/// Used if outputs > HF_VERSION_VIEW_TAGS
/// **Source** <`monero/src/cryptonote_basic/cryptonote_basic.h`>(https://github.com/monero-project/monero/blob/ea87b30f8907ee11252433811e7a7d0c46758cca/src/cryptonote_basic/cryptonote_basic.h#85-97)
#[derive(Debug, Clone)]
pub struct TxOutToTaggedKey {
    pub key: PublicKey,
    pub view_tag: ViewTag,
}

impl DoSerialize for TxOutToTaggedKey {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized.serialize_field("key", &self.key)?;
        serialized.add_tag("view_tag");
        serialized.serialize_directly(&[self.view_tag.0])?;
        Ok(())
    }
}
/// Represents the possible statuses associated with a pending transaction
#[derive(Debug, Clone, Copy)]
pub enum Status {
    StatusOk,
    StatusError,
    StatusCritical,
}

/// Represents the options for the priority of a pending transaction
#[derive(Debug, Clone, Copy)]
pub enum Priority {
    PriorityDefault = 0,
    PriorityLow = 1,
    PriorityMedium = 2,
    PriorityHigh = 3,
    PriorityLast,
}

/// Contains info about the status and priority of a transaction and interfaces
/// with the details of the pending transaction (PendingTx struct) Implemented
/// in Rust based on aspects of both Monero's PendingTransaction struct as well
/// as Monero's PendingTransactionImpl class
/// **Source for PendingTransactionImpl class** <`monero/src/wallet/api/pending_transaction.h`>(https://github.com/monero-project/monero/blob/9a124f681119855949f6406ecd69c2ad91da9770/src/wallet/api/pending_transaction.h#L41-L72)
#[derive(Debug, Clone)]
pub struct PendingTransaction {
    pub status: Status,
    pub priority: Priority,
    pub pending_tx: PendingTx,
    pub signers: HashSet<PublicKey>,
    pub key_images: HashSet<KeyImage>,
}

/// Struct used to contain the specifications of a transaction to be sent
pub struct SendTransaction {
    pub destinations: Vec<TxDestinationEntry>,
    pub priority: Priority,
    pub sweep_all: bool,
    pub payment_id: Option<PaymentId>,
    pub from_addr: Address,
    pub fork_version: u8,
    pub fee_mask: u64,
    pub per_byte_fee: u64,
}

/// Based on Monero's get_outs_entry typedef
/// **Source** <`monero/src/wallet/wallet2.h`>(https://github.com/monero-project/monero/blob/75d80d431a9586996c559cb39f3eabebad3da60a/src/wallet/wallet2.h#L792)
pub struct GetOutsEntry(pub u64, pub PublicKey, pub RctKey);

/// Error type for the transaction module
#[derive(Debug, Error)]
pub enum Error {
    /// Error because sending zero amount
    #[error("Error because sending zero amount")]
    SendingZeroAmount,
    /// Error from handling payment id
    #[error("Error from handling payment id")]
    PaymentIdError(#[from] payment_id::Error),
    /// Error stemming from insufficient funds for transfer
    #[error("Insufficient funds, unable to complete transfer, needed {needed:?}, found {found:?}")]
    InsufficientFunds { needed: u64, found: u64 },
    /// Error stemming from insufficient funds to cover fees and send amount
    #[error("Insufficient funds to cover fees and send amount, unable to complete transfer, needed {needed:?}, found {found:?}")]
    InsufficientFundsForFee { needed: u64, found: u64 },
    /// Error from the public_key module
    #[error("Error from the public_key module: {0}")]
    PublicKeyError(#[from] public_key::Error),
    /// Missing an expected rct string
    #[error("Missing an expected rct string")]
    MissingRctString,
    /// Did not find real output index
    #[error("Did not find real output index")]
    DidNotFindRealOutputIndex,
    /// Only one payment id allowed per transaction
    #[error("Only one payment id allowed per transaction")]
    OnlyOnePaymentIdAllowed,
    /// Error from key_image module
    #[error("Error from key_image module: {0}")]
    KeyImageError(#[from] key_image::Error),
    /// Real output index is out of bounds
    #[error("Real output index is out of bounds")]
    RealOutputIndexOutOfBounds { index: usize, size: usize },
    /// Error with the check derived key not equaling the real key
    #[error("Derived not equal real: index {index:?}, real_out {real_out:?}, derived_key {derived_key:?}, real_key {real_key:?}")]
    DerivedNotEqualReal {
        index: usize,
        real_out: u64,
        derived_key: String,
        real_key: String,
    },
    /// Invalid version
    #[error("Invalid version: {0:?}")]
    InvalidVersion(usize),
    /// Error from UTF8 conversion
    #[error("Error from UTF8 conversion: {0}")]
    Utf8Error(#[from] Utf8Error),
    /// Error from the rct_types module
    #[error("Error from the anyhow::Error: {0}")]
    RctTypesError(#[from] anyhow::Error),
    /// Error from vectors having different lengths
    #[error("Expected vectors to the same length: vector 1 length {0:?} != vector 2 length {1:?}")]
    DifferentLengths(usize, usize),
    /// Not all TxOutTargetVariant types are currently supported
    #[error("Not all TxOutTargetVariant types are currently supported")]
    UnsupportedTxOutTargetVariant,
    /// Unable to meet the hard fork version expectation
    #[error("Unable to meet the hard fork version expectated minimum fork version: {expected:?}, found {found:?}")]
    InvalidHardForkVersionAssumption { found: u8, expected: u8 },
}

/// Stores transaction parameters that were calculated in the prepare step
pub struct TransactionParameters {
    pub priority: Priority,
    pub change_amount: MoneroAmount,
    pub final_total_wo_fee: MoneroAmount,
    pub mixin: usize,
    pub using_fee: MoneroAmount,
    pub using_outs: Vec<UnspentOutput>,
    pub destinations: Vec<TxDestinationEntry>, // does not include the change destination
    pub change_dst: TxDestinationEntry,        // identifies the change destination
    pub splitted_dsts: Vec<TxDestinationEntry>, /* includes all the destinations including the
                                                * change destination */
    pub fork_version: u8,
    pub payment_id: Option<PaymentId>,
}

#[cfg(test)]

mod tests {

    use hex_literal;

    use super::*;

    #[test]
    fn test_derive_view_tag() {
        // One example test case taken from the monero codebase
        // 0fc47054f355ced4d67de73bfa12e4c78ff19089548fffa7d07a674741860f97 0 76
        let derivation_bytes =
            hex_literal::hex!("0fc47054f355ced4d67de73bfa12e4c78ff19089548fffa7d07a674741860f97");
        let deriv_result = KeyDerivation::from_slice(&derivation_bytes);
        assert!(deriv_result.is_ok());
        let derivation = deriv_result.unwrap();
        let output_index = 0;
        let expected_view_tag = hex_literal::hex!("76");
        let calculated_view_tag = ViewTag::derive(&derivation, output_index);
        assert_eq!(calculated_view_tag.0, expected_view_tag[0]);
    }
}
