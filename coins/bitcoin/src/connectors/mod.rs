//! # WalletD Connectors
//!
//! Provides interfaces for common connectors to blockchain data.
//!

use crate::bitcoin::{Address, AddressType};
use bitcoin_hashes::{sha256d, Hash};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use time::format_description::well_known::Rfc2822;
use time::{Duration, OffsetDateTime};

use crate::BitcoinAmount;

use crate::Error;
/// Represents a Bitcoin transaction in the format with the data fields returned by Blockstream
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct BTransaction {
    #[serde(default)]
    /// Txid
    pub txid: String,
    #[serde(default)]
    /// Version
    pub version: i32,
    #[serde(default)]
    /// Locktime
    pub locktime: u32,
    #[serde(default)]
    /// Vector of Inputs
    pub vin: Vec<Input>,
    #[serde(default)]
    /// Vector of Outputs
    pub vout: Vec<Output>,
    #[serde(default)]
    /// Size
    pub size: u64,
    #[serde(default)]
    /// Weight
    pub weight: u64,
    #[serde(default)]
    /// Fee
    pub fee: u64,
    #[serde(default)]
    /// Status
    pub status: Status,
}

/// Represents a Bitcoin transaction out in the format with the data fields returned by Blockstream
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Output {
    #[serde(default)]
    /// ScriptPubKey
    pub scriptpubkey: String,
    #[serde(default)]
    /// ScriptPubKey ASM
    pub scriptpubkey_asm: String,
    #[serde(default)]
    /// ScriptPubKey Type
    pub scriptpubkey_type: String,
    #[serde(default)]
    /// ScriptPubKey Address
    pub scriptpubkey_address: String,
    #[serde(default)]
    /// PubKeyHash
    pub pubkeyhash: String,
    #[serde(default)]
    /// Value in Satoshis
    pub value: u64,
}

/// Represents a Bitcoin transaction input in the format with the data fields returned by Blockstream
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Input {
    #[serde(default)]
    /// Tx ID
    pub txid: String,
    #[serde(default)]
    /// Index of the output that this input represents from the previous transaction
    pub vout: u32,
    #[serde(default)]
    /// Previous output
    pub prevout: Output,
    #[serde(default)]
    /// ScriptSig
    pub scriptsig: String,
    #[serde(default)]
    /// ScriptSig ASM
    pub scriptsig_asm: String,
    #[serde(default)]
    /// Witness
    pub witness: Vec<String>,
    #[serde(default)]
    /// Is coinbase
    pub is_coinbase: bool,
    #[serde(default)]
    /// Sequence
    pub sequence: u32,
    #[serde(default)]
    /// Inner RedeemScript
    pub inner_redeemscript_asm: String,
}

/// Represents the Status of a Bitcoin transaction in the format with the data fields returned by Blockstream
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct Status {
    #[serde(default)]
    /// Confirmed
    pub confirmed: bool,
    #[serde(default)]
    /// Block Height
    pub block_height: u32,
    #[serde(default)]
    /// Block Hash
    pub block_hash: String,
    #[serde(default)]
    /// Block Time
    pub block_time: u32,
}

impl Status {
    /// Returns the timestamp based on the block_time data as a string formatted as RFC2822
    pub fn timestamp(&self) -> Result<String, Error> {
        if self.confirmed {
            // Creates a timestamp from the specified number of whole seconds which have passed since the UNIX_EPOCH
            match OffsetDateTime::UNIX_EPOCH.checked_add(Duration::new(self.block_time.into(), 0)) {
                // Formats the combined date and time with the specified format string.
                Some(timestamp) => {
                    let formatted_timestamp = timestamp.format(&Rfc2822)?;
                    Ok(formatted_timestamp)
                }
                None => Err(Error::Timestamp(
                    "Overflow error when converting timestamp".into(),
                )),
            }
        } else {
            Ok("".to_string())
        }
    }
}

/// Represents a Bitcoin UTXO (Unspent Transaction Output) in the format with the data fields returned by Blockstream
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Utxo {
    #[serde(default)]
    /// Status of the UTXO
    pub status: Status,
    #[serde(default)]
    /// Txid associated with the UTXO
    pub txid: String,
    #[serde(default)]
    /// Value in satoshis
    pub value: u64,
    #[serde(default)]
    /// The index of the output in the associated transaction
    pub vout: u32,
}

/// A wrapper around a vector of Utxo objects.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Utxos(pub Vec<Utxo>);

impl Utxos {
    /// Creates a new Utxos empty vector.
    pub fn new() -> Self {
        Utxos(Vec::new())
    }

    /// Returns whether the Uxtos vector is empty or not.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator to the underlying vector.
    pub fn iter(&self) -> std::slice::Iter<Utxo> {
        self.0.iter()
    }

    /// Returns sum of all Utxos in the vector as a BitcoinAmount.
    pub fn sum(&self) -> Result<BitcoinAmount, Error> {
        let mut satoshis: u64 = 0;
        for item in self.iter() {
            satoshis += item.value;
        }
        let confirmed_balance = BitcoinAmount { satoshi: satoshis };
        Ok(confirmed_balance)
    }

    /// Pushes a Utxo to the Utxos vector.
    pub fn push(&mut self, utxo: Utxo) {
        self.0.push(utxo);
    }
}

/// Enum of possible input types.
pub enum InputType {
    /// Pay to public key hash.
    P2pkh,
    /// Pay to script hash.
    P2sh,
    /// Pay to witness script hash.
    P2wsh,
    /// Pay to witness public key hash.
    P2wpkh,
    /// Pay to script hash nested in witness script hash.
    P2sh2Wpkh,
    /// Pay to script hash nested in witness public key hash.
    P2sh2Wsh,
}

impl InputType {
    /// Returns the input type of the given UTXO.
    pub fn new(utxo_prevout: &Output) -> Result<Self, Error> {
        match utxo_prevout.scriptpubkey_type.as_str() {
            "p2pkh" => Ok(InputType::P2pkh),
            "p2sh" => {
                let scriptpubkey_asm = &utxo_prevout
                    .scriptpubkey_asm
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                let op_pushbytes = scriptpubkey_asm.get(1);
                if let Some(op) = op_pushbytes {
                    match op.as_str() {
                        "OP_PUSHBYTES_22" => return Ok(InputType::P2sh2Wpkh),
                        "OP_PUSHBYTES_34" => return Ok(InputType::P2sh2Wsh),
                        _ => return Ok(InputType::P2sh),
                    }
                }
                Ok(InputType::P2sh)
            }
            "v0_p2wsh" => Ok(InputType::P2wsh),
            "v0_p2wpkh" => Ok(InputType::P2wpkh),
            _ => Err(Error::CurrentlyNotSupported(
                "Unknown scriptpubkey_type, not currently handled".into(),
            )),
        }
    }

    /// Returns whether the input type is segwit or not.
    pub fn is_segwit(&self) -> bool {
        match self {
            InputType::P2pkh | InputType::P2sh => false,
            InputType::P2sh2Wpkh | InputType::P2sh2Wsh | InputType::P2wsh | InputType::P2wpkh => {
                true
            }
        }
    }
}

impl BTransaction {
    /// Calculates a transaction hash for signing a segwit input with a given index
    pub fn transaction_hash_for_signing_segwit_input_index(
        &self,
        index: usize,
        sighash_num: u32,
    ) -> Result<String, Error> {
        let serialized = self.serialize_for_segwit_input_index_with_sighash(index, sighash_num)?;
        let hash = sha256d::Hash::hash(&hex::decode(serialized)?);
        Ok(hex::encode(hash))
    }

    /// Serializes the transaction for a given input index
    pub fn serialize_for_segwit_input_index_with_sighash(
        &self,
        index: usize,
        sighash_num: u32,
    ) -> Result<String, Error> {
        let input = self.vin.get(index).expect("index not present");
        let mut serialization = String::new();

        // nVersion of the transaction (4-byte little endian)
        let version_encoded = self.version.to_le_bytes();
        serialization.push_str(&hex::encode(version_encoded));

        // hashPrevouts, double sha256 hash of the all of the previous outpoints (32
        // byte hash) Ignoring case of ANYONECANPAY
        let mut prevouts_serialized = String::new();
        for input_here in &self.vin {
            let prev_txid = &input_here.txid;
            if prev_txid.len() != 64 {
                return Err(Error::TxId(
                    "The references txid in hex format should be 64 characters long".into(),
                ));
            }
            let prev_txid_encoded = Self::hex_reverse_byte_order(prev_txid)?;
            prevouts_serialized.push_str(prev_txid_encoded.as_str());
            let prev_vout: u32 = input_here.vout;
            let prev_vout_encoded = &prev_vout.to_le_bytes();
            prevouts_serialized.push_str(&hex::encode(prev_vout_encoded));
        }

        let hash_prevouts = hex::encode(sha256d::Hash::hash(&hex::decode(prevouts_serialized)?));

        serialization.push_str(hash_prevouts.as_str());

        // hashSequence (using the sequence from each input) (32 byte hash)
        // this is hardcoded right now ignoring case of sighash ANYONECANPAY, SINGLE,
        // NONE
        let mut sequence_serialized = String::new();
        for input_here in &self.vin {
            let sequence_here = input_here.sequence.to_le_bytes();
            sequence_serialized.push_str(hex::encode(sequence_here).as_str());
        }
        let hash_sequence = hex::encode(sha256d::Hash::hash(&hex::decode(sequence_serialized)?));

        serialization.push_str(hash_sequence.as_str());

        // outpoint (32-byte hash + 4-byte little endian)
        let prev_txid = &input.txid;
        if prev_txid.len() != 64 {
            return Err(Error::TxId(
                "The references txid in hex format should be 64 characters long".into(),
            ));
        }
        let prev_txid_encoded = Self::hex_reverse_byte_order(prev_txid)?;
        serialization.push_str(prev_txid_encoded.as_str());
        let prev_vout: u32 = input.vout;
        let prev_vout_encoded = &prev_vout.to_le_bytes();
        serialization.push_str(&hex::encode(prev_vout_encoded));

        // scriptCode of the input, hardcoded to p2wpkh
        let pubkeyhash = input.prevout.pubkeyhash.as_str();

        let script_code = "1976a914".to_string() + pubkeyhash + "88ac";
        serialization.push_str(script_code.as_str());

        // value of output spent by this input (8 byte little endian)
        serialization.push_str(&hex::encode(input.prevout.value.to_le_bytes()));

        // nSequence of the input (4 byte little endian)
        serialization.push_str(&hex::encode(input.sequence.to_le_bytes()));

        // hashOutputs (32 byte hash) hardcoding for sighash ALL
        let mut outputs_serialization = String::new();
        for output in &self.vout {
            let value: u64 = output.value;
            let value_encoded = value.to_le_bytes();
            outputs_serialization.push_str(&hex::encode(value_encoded));
            let len_scriptpubkey = output.scriptpubkey.len();
            if len_scriptpubkey % 2 != 0 {
                return Err(Error::ScriptInvalid(
                    "Length of scriptpubkey should be a multiple of 2".into(),
                ));
            }
            let len_scriptpubkey_encoded =
                Self::variable_length_integer_encoding(len_scriptpubkey / 2)?;
            outputs_serialization.push_str(&hex::encode(len_scriptpubkey_encoded));
            // scriptpubkey is already encoded for the serialization
            outputs_serialization.push_str(output.scriptpubkey.as_str());
        }
        let hash_outputs = hex::encode(sha256d::Hash::hash(&hex::decode(outputs_serialization)?));
        serialization.push_str(hash_outputs.as_str());
        // Lock Time
        serialization.push_str(&hex::encode(self.locktime.to_le_bytes()));
        // Sighash
        serialization.push_str(&hex::encode(sighash_num.to_le_bytes()));

        Ok(serialization)
    }

    /// Serializes the transaction data (makes a hex string) considering the
    /// data from all of the fields
    pub fn serialize(transaction: &Self) -> Result<String, Error> {
        let mut serialization = String::new();
        // version
        let version_encoded = transaction.version.to_le_bytes();
        serialization.push_str(&hex::encode(version_encoded));

        // Handling the segwit marker and flag
        let mut segwit_transaction = false;
        for input in transaction.vin.iter() {
            if !input.witness.is_empty() {
                segwit_transaction = true;
            }
        }

        if segwit_transaction {
            let marker_encoded = "00";
            serialization.push_str(marker_encoded);
            let flag_encoded = "01";
            serialization.push_str(flag_encoded);
        }

        // Inputs
        let num_inputs = transaction.vin.len();
        let num_inputs_encoded = Self::variable_length_integer_encoding(num_inputs)?;
        serialization.push_str(&hex::encode(num_inputs_encoded));
        for input in &transaction.vin {
            let prev_txid = &input.txid;
            if prev_txid.len() != 64 {
                return Err(Error::TxId(
                    "The reference txid in hex format should be 64 characters long".into(),
                ));
            }
            let prev_txid_encoded = Self::hex_reverse_byte_order(prev_txid)?;
            serialization.push_str(prev_txid_encoded.as_str());
            let prev_vout: u32 = input.vout;
            let prev_vout_encoded = &prev_vout.to_le_bytes();
            serialization.push_str(&hex::encode(prev_vout_encoded));
            let len_signature_script = input.scriptsig.len();
            if len_signature_script % 2 != 0 {
                return Err(Error::ScriptInvalid(
                    "Length of script_sig should be a multiple of 2".into(),
                ));
            }
            let len_signature_script_encoded =
                Self::variable_length_integer_encoding(len_signature_script / 2)?;
            serialization.push_str(&hex::encode(len_signature_script_encoded));
            // script_sig is already encoded for the serialization
            serialization.push_str(&input.scriptsig);
            // sequence
            serialization.push_str(&hex::encode(input.sequence.to_le_bytes()));
        }

        // Outputs
        let num_outputs = transaction.vout.len();
        let num_outputs_encoded = Self::variable_length_integer_encoding(num_outputs)?;
        serialization.push_str(&hex::encode(num_outputs_encoded));
        for output in &transaction.vout {
            let value: u64 = output.value;
            let value_encoded = value.to_le_bytes();
            serialization.push_str(&hex::encode(value_encoded));
            let len_scriptpubkey = output.scriptpubkey.len();
            if len_scriptpubkey % 2 != 0 {
                return Err(Error::ScriptInvalid(
                    "Length of scriptpubkey should be a multiple of 2".into(),
                ));
            }
            let len_scriptpubkey_encoded =
                Self::variable_length_integer_encoding(len_scriptpubkey / 2)?;
            serialization.push_str(&hex::encode(len_scriptpubkey_encoded));
            // scriptpubkey is already encoded for the serialization
            serialization.push_str(output.scriptpubkey.as_str());
        }

        // Witness data
        if segwit_transaction {
            let mut witness_counts: Vec<usize> = Vec::new();
            let mut witness_lens: Vec<u8> = Vec::new();
            let mut witness_data: Vec<String> = Vec::new();

            for (i, input) in transaction.vin.iter().enumerate() {
                witness_counts.push(0);
                for data in &input.witness {
                    witness_counts[i] += 1;
                    if data.len() % 2 != 0 {
                        return Err(Error::ScriptInvalid(
                            "Witness data length in hex should be a multiple of 2".into(),
                        ));
                    }
                    witness_lens.push((data.len() / 2).try_into()?);
                    witness_data.push(data.to_string());
                }
            }
            let mut witness_counter = 0;
            for witness_count in witness_counts {
                serialization.push_str(&hex::encode(Self::variable_length_integer_encoding(
                    witness_count,
                )?));
                for _j in 0..witness_count {
                    serialization
                        .push_str(&hex::encode(witness_lens[witness_counter].to_le_bytes()));
                    serialization.push_str(witness_data[witness_counter].as_str());
                    witness_counter += 1;
                }
            }
        }

        // Lock Time
        serialization.push_str(&hex::encode(transaction.locktime.to_le_bytes()));
        Ok(serialization)
    }

    /// Displays the transaction id in the form used in the blockchain which is
    /// reverse byte of txid()
    pub fn txid_blockchain(&self) -> Result<String, Error> {
        let txid = self.txid()?;
        Self::hex_reverse_byte_order(&txid)
    }

    /// Hashes the transaction without including the segwit data
    pub fn txid(&self) -> Result<String, Error> {
        let mut transaction = self.clone();
        for input in &mut transaction.vin {
            input.witness = Vec::new();
        }
        let serialization = Self::serialize(&transaction)?;
        let txid = sha256d::Hash::hash(&hex::decode(serialization)?);
        Ok(hex::encode(txid))
    }

    /// Hashes the transaction including all data (including the segwit witness
    /// data)
    pub fn wtxid(&self) -> Result<String, Error> {
        let transaction = self.clone();
        let serialization = Self::serialize(&transaction)?;
        let txid = sha256d::Hash::hash(&hex::decode(serialization)?);
        Ok(hex::encode(txid))
    }

    /// Returns the "normalized txid" - sha256 double hash of the serialized
    /// transaction data without including any inputs unlocking data
    /// (witness data and signature, public key data is not included)
    pub fn ntxid(&self) -> Result<String, Error> {
        let mut transaction = self.clone();
        for input in &mut transaction.vin {
            input.witness = Vec::new();
            input.scriptsig = String::new();
            input.scriptsig_asm = String::new();
        }
        let serialization = Self::serialize(&transaction)?;
        let ntxid = sha256d::Hash::hash(&hex::decode(serialization)?);
        Ok(hex::encode(ntxid))
    }

    /// Returns a string that is the reverse byte order string representation of the input hex string
    pub fn hex_reverse_byte_order(hex_string: &String) -> Result<String, Error> {
        let len = hex_string.len();
        if len % 2 != 0 {
            return Err(Error::ScriptInvalid(
                "The hex string should have a length that is a multiple of 2".into(),
            ));
        }
        let mut encoded = String::new();
        for i in 0..len / 2 {
            let reverse_ind = len - i * 2 - 2;
            encoded.push_str(&hex_string[reverse_ind..reverse_ind + 2]);
        }
        Ok(encoded)
    }

    /// Returns the variable length integer encoding of the input number
    pub fn variable_length_integer_encoding(num: usize) -> Result<Vec<u8>, Error> {
        if num < 0xFD {
            Ok(vec![num as u8])
        } else if num <= 0xFFFF {
            let num_as_bytes = (num as u16).to_le_bytes().to_vec();
            Ok([vec![0xFD], num_as_bytes].concat())
        } else if num <= 0xFFFFFFFF {
            let num_as_bytes = (num as u32).to_le_bytes().to_vec();
            Ok([vec![0xFE], num_as_bytes].concat())
        } else {
            let num_as_bytes = (num as u64).to_le_bytes().to_vec();
            Ok([vec![0xFF], num_as_bytes].concat())
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            txid: String::new(),
            vout: 0,
            prevout: Output {
                ..Default::default()
            },
            scriptsig: String::new(),
            scriptsig_asm: String::new(),
            witness: Vec::new(),
            is_coinbase: false,
            sequence: 0xFFFFFFFF,
            inner_redeemscript_asm: String::new(),
        }
    }
}

impl Output {
    /// Sets the scriptpubkey info for the output based on the address
    pub fn set_scriptpubkey_info(&mut self, address_info: Address) -> Result<(), Error> {
        self.scriptpubkey_address = address_info.to_string();
        let address_type = address_info.address_type().expect("address type missing");
        match address_type {
            AddressType::P2pkh => self.scriptpubkey_type = "p2pkh".to_string(),
            AddressType::P2sh => self.scriptpubkey_type = "p2sh".to_string(),
            AddressType::P2wpkh => self.scriptpubkey_type = "v0_p2wpkh".to_string(),
            AddressType::P2wsh => self.scriptpubkey_type = "v0_p2wsh".to_string(),
            _ => {
                return Err(Error::CurrentlyNotSupported(
                    "Currently not implemented setting scriptpubkey for this address type".into(),
                ))
            }
        }
        let script_pubkey = address_info.script_pubkey();
        self.scriptpubkey_asm = script_pubkey.to_asm_string();
        self.scriptpubkey = hex::encode(script_pubkey.as_bytes());
        Ok(())
    }
}

/// FeeEstimates is a wrapper around the fee estimates returned by the Blockstream API
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FeeEstimates(pub serde_json::Map<String, Value>);
