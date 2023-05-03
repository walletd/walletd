//! This module contains the implementation of the handling getting information to and from the bitcoin blockchain using the Blockstream Esplora JSON over HTTP API <https://github.com/Blockstream/esplora/blob/master/API.md>
//!
//!

use async_trait::async_trait;
use bitcoin::{Address, AddressType};
use bitcoin_hashes::{sha256d, Hash};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use walletd_coin_core::BlockchainConnector;

use time::format_description::well_known::Rfc2822;
use time::{Duration, OffsetDateTime};

use crate::BitcoinAmount;

pub use bitcoin::{
    sighash::EcdsaSighashType, Network, PrivateKey as BitcoinPrivateKey,
    PublicKey as BitcoinPublicKey, Script,
};

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

/// A blockchain connector for Bitcoin which follows [`the Blockstream API`](https://github.com/Blockstream/esplora/blob/master/API.md).
#[derive(Clone, Default, Debug)]
pub struct Blockstream {
    /// The client used to make requests to the API
    pub client: reqwest::Client,
    /// The url of the API
    pub url: String,
}

#[async_trait]
impl BlockchainConnector for Blockstream {
    type ErrorType = Error;

    fn new(url: &str) -> Result<Self, Error> {
        Ok(Self {
            client: reqwest::Client::new(),
            url: url.to_string(),
        })
    }

    fn url(&self) -> &str {
        &self.url
    }
}

/// FeeEstimates is a wrapper around the fee estimates returned by the Blockstream API
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FeeEstimates(pub serde_json::Map<String, Value>);

impl Blockstream {
    /// Checks if the given address has had an past transactions, returns true if it has and false if it has not
    /// Errors if the address is invalid or if the API returns an error
    pub async fn check_if_past_transactions_exist(
        &self,
        public_address: &str,
    ) -> Result<bool, Error> {
        let transactions = self.transactions(public_address).await?;
        if transactions.is_empty() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Fetch the block height
    pub fn block_count(&self) -> Result<u64, Error> {
        let body = reqwest::blocking::get(format!("{}/blocks/tip/height", self.url))
            .expect("Error getting block count")
            .text()?;
        let block_count = body
            .parse::<u64>()
            .map_err(|e| Error::FromStr(e.to_string()))?;
        Ok(block_count)
    }

    /// Fetch fee estimates from blockstream
    pub async fn fee_estimates(&self) -> Result<FeeEstimates, Error> {
        let body = reqwest::get(format!("{}/fee-estimates", self.url))
            .await?
            .text()
            .await?;
        let fee_estimates: FeeEstimates = serde_json::from_str(&body)?;
        Ok(fee_estimates)
    }

    /// Fetch transactions from blockstream
    pub async fn transactions(&self, address: &str) -> Result<Vec<BTransaction>, Error> {
        let body = reqwest::get(format!("{}/address/{}/txs", self.url, address))
            .await?
            .text()
            .await?;
        let transactions: Vec<BTransaction> = serde_json::from_str(&body)?;
        Ok(transactions)
    }

    /// Fetch mempool transactions from blockstream
    pub fn mempool_transactions(&self, address: &str) -> Result<Vec<BTransaction>, Error> {
        let body = reqwest::blocking::get(format!("{}/address/{}/txs/mempool", self.url, address))
            .expect("Error getting transactions")
            .text();
        let transactions: Vec<BTransaction> = serde_json::from_str(&body?)?;
        Ok(transactions)
    }

    /// Fetch UTXOs from blockstream
    pub async fn utxo(&self, address: &str) -> Result<Utxos, Error> {
        let body = reqwest::get(format!("{}/address/{}/utxo", self.url, address))
            .await?
            .text()
            .await?;

        let utxos: Utxos = serde_json::from_str(&body)?;
        Ok(utxos)
    }

    /// Fetch raw transaction hex from blockstream for a given txid
    pub async fn raw_transaction_hex(&self, txid: &str) -> Result<String, Error> {
        let body = reqwest::get(format!("{}/tx/{}/hex", self.url, txid))
            .await?
            .text()
            .await?;
        Ok(body)
    }

    /// Fetch transaction info
    pub async fn transaction(&self, txid: &str) -> Result<BTransaction, Error> {
        let body = reqwest::get(format!("{}/tx/{}", self.url, txid))
            .await?
            .text()
            .await?;

        let transaction: BTransaction = serde_json::from_str(&body)?;
        Ok(transaction)
    }

    /// Broadcast a raw transaction to the network
    pub async fn post_a_transaction(
        &self,
        raw_transaction_hex: &'static str,
    ) -> Result<String, Error> {
        let trans_resp = self
            .client
            .post(format!("{}/tx", self.url))
            .body(raw_transaction_hex)
            .send()
            .await?;

        let trans_status = trans_resp.status();
        let trans_content = trans_resp.text().await?;
        if !trans_status.is_client_error() && !trans_status.is_server_error() {
            Ok(trans_content)
        } else {
            log::info!(
                "trans_status.is_client_error(): {}",
                trans_status.is_client_error()
            );
            log::info!(
                "trans_status.is_server_error(): {}",
                trans_status.is_server_error()
            );
            log::info!("trans_content: {}", trans_content);
            Err(Error::BroadcastTransaction(trans_content))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::{json, Number, Value};

    #[test]
    fn test_block_count() {
        let mut server = Server::new();
        let expected_blockcount = 773876;
        server
            .mock("GET", "/blocks/tip/height")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(expected_blockcount.to_string())
            .create();

        let bs = Blockstream::new(&server.url()).unwrap();
        let check_blockcount = bs.block_count().unwrap();
        assert_eq!(expected_blockcount, check_blockcount);
    }

    #[tokio::test]
    async fn test_fee_estimates() {
        let mut server = Server::new();
        let mut expected_fee_map = serde_json::Map::new();
        expected_fee_map.insert(
            String::from("1"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("10"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("1008"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("11"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("12"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("13"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("14"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("144"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("15"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("16"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("17"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("18"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("19"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("2"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("20"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("21"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("22"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("23"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("24"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("25"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("3"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("4"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("5"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("504"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("6"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("7"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("8"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("9"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );

        server
            .mock("GET", "/fee-estimates")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&Value::Object(expected_fee_map.clone()).to_string())
            .create();

        let bs = Blockstream::new(&server.url()).unwrap();
        let fee_estimates = bs.fee_estimates().await.unwrap();
        assert_eq!(fee_estimates.0, expected_fee_map);
    }

    #[tokio::test]
    async fn test_transactions() {
        let mut server = Server::new();
        let mut expected_transactions_data: Vec<BTransaction> = Vec::new();
        let transactions1 = BTransaction {
        txid: "0de137bb9523540d1114986111e5e2d307473fa716b766be896055282c91e8fe".into(),
        version: 2,
        locktime: 0,
        vin: vec![
            Input {
                txid: "db9fc9977b23d8f3cb157baf6a9695a45bbffa792474b280111a89b7302be832".into(),
                vout: 0,
                prevout: Output {
                    scriptpubkey: "00140124f01c8a411b6f21704dc5f2cf496b3826f1dd".into(),
                    scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 0124f01c8a411b6f21704dc5f2cf496b3826f1dd".into(),
                    scriptpubkey_type: "v0_p2wpkh".into(),
                    scriptpubkey_address: "tb1qqyj0q8y2gydk7gtsfhzl9n6fdvuzduwaqa7jcn".into(),
                    pubkeyhash: "".into(),
                    value: 1000,
                },
                scriptsig: "".into(),
                scriptsig_asm: "".into(),
                witness: vec![
                    "30440220595a94acfcaf2a5ba06504b9ee04fce791049d10f9364e09c5fd1bdc7cbc977102201ceea8f3d8a983ee002734c92a1b0f2bd82d0c583e4d14160c31f3533aff550701".into(),
                    "02491a70fdf8e1ed02e53a59ea9063a089c41b5ac807f1aea5575575d8b9862199".into(),
                ],
                is_coinbase: false,
                sequence: 4294967293,
                inner_redeemscript_asm: "".into(),
            },
        ],
        vout: vec![
            Output {
                scriptpubkey: "00146b03a1d003f6dbc5391695403ba3b276f518197a".into(),
                scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 6b03a1d003f6dbc5391695403ba3b276f518197a".into(),
                scriptpubkey_type: "v0_p2wpkh".into(),
                scriptpubkey_address: "tb1qdvp6r5qr7mdu2wgkj4qrhgajwm63sxt6yekult".into(),
                pubkeyhash: "".into(),
                value: 887,
            },
        ],
        size: 191,
        weight: 437,
        fee: 113,
        status: Status {
            confirmed: true,
            block_height: 2425244,
            block_hash: "00000000747fd5d926d1b1f80f340c26b34fda84ae565728642968dfb5f8fe7b".into(),
            block_time: 1679364907,
        },
    };

        expected_transactions_data.push(transactions1);

        let transaction2 = BTransaction {
        txid: "db9fc9977b23d8f3cb157baf6a9695a45bbffa792474b280111a89b7302be832".into(),
        version: 2,
        locktime: 2425214,
        vin: vec![
            Input {
                txid: "389294def199edcac49c70300584c3c8dfba29176b4ffa937e10ca296a993d8e".into(),
                vout: 1,
                prevout: Output {
                    scriptpubkey: "0014964eb65874d710c3442bea7490e0950786f789e5".into(),
                    scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 964eb65874d710c3442bea7490e0950786f789e5".into(),
                    scriptpubkey_type: "v0_p2wpkh".into(),
                    scriptpubkey_address: "tb1qje8tvkr56ugvx3ptaf6fpcy4q7r00z0932kpdm".into(),
                    pubkeyhash: "".into(),
                    value: 11499616,
                },
                scriptsig: "".into(),
                scriptsig_asm: "".into(),
                witness: vec![
                    "3044022069a338775b081b88fd006e3ff11cc270b5278878b8ff803ff49f94d72d89f33a02205c95eda4dd5228442b957b866fa4039b03cab5aeac89df65ba748825dab9b4c301".into(),
                    "03164ce1ac7319b66f8815f07d5b7519e5cb8c43a4b3fbd78fdb6ab5022ffa17de".into(),
                ],
                is_coinbase: false,
                sequence: 4294967294,
                inner_redeemscript_asm: "".into(),
            },
        ],
        vout: vec![
            Output {
                scriptpubkey: "00140124f01c8a411b6f21704dc5f2cf496b3826f1dd".into(),
                scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 0124f01c8a411b6f21704dc5f2cf496b3826f1dd".into(),
                scriptpubkey_type: "v0_p2wpkh".into(),
                scriptpubkey_address: "tb1qqyj0q8y2gydk7gtsfhzl9n6fdvuzduwaqa7jcn".into(),
                pubkeyhash: "".into(),
                value: 1000,
            },
            Output {
                scriptpubkey: "0014e3ca5697c2d865b5db9fa9334fed52a6881d3ad3".into(),
                scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 e3ca5697c2d865b5db9fa9334fed52a6881d3ad3".into(),
                scriptpubkey_type: "v0_p2wpkh".into(),
                scriptpubkey_address: "tb1qu099d97zmpjmtkul4ye5lm2j56yp6wknfdf8pz".into(),
                pubkeyhash: "".into(),
                value: 11498475,
            },
        ],
        size: 222,
        weight: 561,
        fee: 141,
        status: Status {
            confirmed: true,
            block_height: 2425215,
            block_hash: "000000000000ad213a39c328183ed2803a23e5198a80a7aa2a8feb53c1ee67b8".into(),
            block_time: 1679342982,
        },
    };
        expected_transactions_data.push(transaction2);
        let for_address = "tb1qqyj0q8y2gydk7gtsfhzl9n6fdvuzduwaqa7jcn";

        let json_data_str = json!(expected_transactions_data).to_string();
        let get_path = format!("/address/{}/txs", for_address);
        server
            .mock("GET", get_path.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&json_data_str)
            .create();

        let bs = Blockstream::new(&server.url()).unwrap();
        let transactions_data = bs.transactions(for_address).await.unwrap();
        assert_eq!(transactions_data, expected_transactions_data);
    }

    #[test]
    fn test_mnempool_transactions() {
        let mut server = Server::new();
        let expected_mempool_transactions: Vec<BTransaction> = vec![
        BTransaction {
            txid: "816452a6a65d7533b54fbf1d03fe61ddaa9c856e5d0d1ae2a6b6bb152bd5adcd".into(),
            version: 1,
            locktime: 0,
            vin: vec![
                Input {
                    txid: "5b45205471cd757a328dfb836352a9b46e0a42c221f846d4b89782ec42312eb5".into(),
                    vout: 0,
                    prevout: Output {
                        scriptpubkey: "00148760df7716cc8d8e397ff2807428e6cad0f4af34".into(),
                        scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 8760df7716cc8d8e397ff2807428e6cad0f4af34".into(),
                        scriptpubkey_type: "v0_p2wpkh".into(),
                        scriptpubkey_address: "tb1qsasd7ackejxcuwtl72q8g28xetg0fte533qwgu".into(),
                        pubkeyhash: "".into(),
                        value: 1983973,
                    },
                    scriptsig: "".into(),
                    scriptsig_asm: "".into(),
                    witness: vec![
                        "304402203c6f05a7dad9555d09f9bc09a2365d6d8b9a2e93a1f851a8f56fd9c8ebba42c3022045757a946d0b1dce709b5a284f3c06f7428825a1eb2fb2765614599853cd1a4001".into(),
                        "03bb22bd262849923e41677615fd73401d4d72c57537994d84e62fff5f718199e0".into(),
                    ],
                    is_coinbase: false,
                    sequence: 4294967295,
                    inner_redeemscript_asm: "".into(),
                },
            ],
            vout: vec![
                Output {
                    scriptpubkey: "0014b835437e21844019b74a9b8d825624feb1b16099".into(),
                    scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 b835437e21844019b74a9b8d825624feb1b16099".into(),
                    scriptpubkey_type: "v0_p2wpkh".into(),
                    scriptpubkey_address: "tb1qhq65xl3ps3qpnd62nwxcy43yl6cmzcyekg965v".into(),
                    pubkeyhash: "".into(),
                    value: 200,
                },
                Output {
                    scriptpubkey: "001419f6ec5cca3c958777199fa674e49a89595e5535".into(),
                    scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 19f6ec5cca3c958777199fa674e49a89595e5535".into(),
                    scriptpubkey_type: "v0_p2wpkh".into(),
                    scriptpubkey_address: "tb1qr8mwchx28j2cwacen7n8fey639v4u4f4gvc8z3".into(),
                    pubkeyhash: "".into(),
                    value: 1983547,
                },
            ],
            size: 222,
            weight: 561,
            fee: 226,
            status: Status {
                confirmed: false,
                block_height: 0,
                block_hash: "".into(),
                block_time: 0,
            },
        },
    ];
        let json_data_str = json!(expected_mempool_transactions).to_string();

        let for_address = "tb1qhq65xl3ps3qpnd62nwxcy43yl6cmzcyekg965v";
        let get_path = format!("/address/{}/txs/mempool", for_address);
        server
            .mock("GET", get_path.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&json_data_str)
            .create();

        let bs = Blockstream::new(&server.url()).unwrap();
        let transactions_data = bs.mempool_transactions(for_address).unwrap();
        assert_eq!(transactions_data, expected_mempool_transactions);
    }

    #[tokio::test]
    async fn test_fetch_utxos() {
        let expected_utxos = Utxos(vec![Utxo {
            status: Status {
                confirmed: true,
                block_height: 2425463,
                block_hash: "00000000000000139a0ce10a0ec62ff754023f25b887157d6b422688d1784fd9"
                    .into(),
                block_time: 1679524580,
            },
            txid: "4497bea5ea7784b6f188256fb7ecfb6108a4b8060aa9ed87d1cea5732c3eedba".into(),
            value: 200,
            vout: 0,
        }]);
        let for_address = "tb1qjft2mkemu4jzy5epd45djr56eeej6c932rlt75";
        let json_data_str = json!(expected_utxos).to_string();
        let get_path = format!("/address/{}/utxo", for_address);
        let mut server = Server::new();
        server
            .mock("GET", get_path.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&json_data_str)
            .create();
        let bs = Blockstream::new(&server.url()).unwrap();
        let utxos = bs.utxo(for_address).await.unwrap();
        assert_eq!(utxos, expected_utxos);
    }

    #[tokio::test]
    async fn test_raw_transaction_hex() {
        let expected_tx_hex = "01000000000101d716b50967b96ac283a30b7a26408a5c95d7c08f05be660c1ce6cc0c576df4230100000000ffffffff02c8000000000000001600149256addb3be5642253216d68d90e9ace732d60b15c010000000000001600144074db37babb2ac2a6ad993219c09b2ffd4e39b002483045022100896671a10bbeab473d62afb52d287b7bf7509c88ad2fdccca9bc7299cbf678b3022041c489f0f4ff42e21bb5745f42fe257558533d944bf1e308071be557188697610121037ff20be5933c3093c6c57456c0fc829ef6101c960c59ee82d2d194bcd3883ee200000000";
        let for_txid = "4497bea5ea7784b6f188256fb7ecfb6108a4b8060aa9ed87d1cea5732c3eedba";
        let get_path = format!("/tx/{}/hex", for_txid);
        let mut server = Server::new();
        server
            .mock("GET", get_path.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&expected_tx_hex)
            .create();
        let bs = Blockstream::new(&server.url()).unwrap();
        let raw_tx_hex = bs.raw_transaction_hex(for_txid).await.unwrap();
        assert_eq!(raw_tx_hex, expected_tx_hex);
    }

    #[tokio::test]
    async fn test_transaction() {
        let expected_tx = BTransaction {
        txid: "4497bea5ea7784b6f188256fb7ecfb6108a4b8060aa9ed87d1cea5732c3eedba".into(),
        version: 1,
        locktime: 0,
        vin: vec![
            Input {
                txid: "23f46d570ccce61c0c66be058fc0d7955c8a40267a0ba383c26ab96709b516d7".into(),
                vout: 1,
                prevout: Output {
                    scriptpubkey: "00144074db37babb2ac2a6ad993219c09b2ffd4e39b0".into(),
                    scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 4074db37babb2ac2a6ad993219c09b2ffd4e39b0".into(),
                    scriptpubkey_type: "v0_p2wpkh".into(),
                    scriptpubkey_address: "tb1qgp6dkda6hv4v9f4dnyepnsym9l75uwds4fq3n8".into(),
                    pubkeyhash: "".into(),
                    value: 774,
                },
                scriptsig: "".into(),
                scriptsig_asm: "".into(),
                witness: vec![
                    "3045022100896671a10bbeab473d62afb52d287b7bf7509c88ad2fdccca9bc7299cbf678b3022041c489f0f4ff42e21bb5745f42fe257558533d944bf1e308071be5571886976101".into(),
                    "037ff20be5933c3093c6c57456c0fc829ef6101c960c59ee82d2d194bcd3883ee2".into(),
                ],
                is_coinbase: false,
                sequence: 4294967295,
                inner_redeemscript_asm: "".into(),
            },
        ],
        vout: vec![
            Output {
                scriptpubkey: "00149256addb3be5642253216d68d90e9ace732d60b1".into(),
                scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 9256addb3be5642253216d68d90e9ace732d60b1".into(),
                scriptpubkey_type: "v0_p2wpkh".into(),
                scriptpubkey_address: "tb1qjft2mkemu4jzy5epd45djr56eeej6c932rlt75".into(),
                pubkeyhash: "".into(),
                value: 200,
            },
            Output {
                scriptpubkey: "00144074db37babb2ac2a6ad993219c09b2ffd4e39b0".into(),
                scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 4074db37babb2ac2a6ad993219c09b2ffd4e39b0".into(),
                scriptpubkey_type: "v0_p2wpkh".into(),
                scriptpubkey_address: "tb1qgp6dkda6hv4v9f4dnyepnsym9l75uwds4fq3n8".into(),
                pubkeyhash: "".into(),
                value: 348,
            },
        ],
        size: 223,
        weight: 562,
        fee: 226,
        status: Status {
            confirmed: true,
            block_height: 2425463,
            block_hash: "00000000000000139a0ce10a0ec62ff754023f25b887157d6b422688d1784fd9".into(),
            block_time: 1679524580,
        },
    };

        let for_txid = "4497bea5ea7784b6f188256fb7ecfb6108a4b8060aa9ed87d1cea5732c3eedba";
        let get_path = format!("/tx/{}", for_txid);
        let json_data_str = json!(expected_tx).to_string();
        let mut server = Server::new();
        server
            .mock("GET", get_path.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&json_data_str)
            .create();
        let bs = Blockstream::new(&server.url()).unwrap();
        let tx = bs.transaction(for_txid).await.unwrap();
        assert_eq!(tx, expected_tx);
    }

    #[tokio::test]
    async fn test_post_a_transaction() {
        let raw_tx_data = "0100000000010141e5cc0928a3083bd6ea84b2955f1f5a01d6f7d5a0ff6dd797ba2d54f7fcd5bf0100000000ffffffff0201000000000000001600144074db37babb2ac2a6ad993219c09b2ffd4e39b09ae21d00000000001600143cb8f6ff881c210d051b562ab7cfff2ef53e2dda02473044022050a97fe6f89bcb995160507621a2a329f5d6de286758f63a57298ee562e0ec1e02206042ebc9e77dd7a38f197b85a8edc5018a0ce0422c4131b5d9ca5b0504de9e33012103d2f1f1b5b0915a302472d2a25a405641fbeca00ef7a5261252e28b7336bec61900000000";
        let mut server = Server::new();
        server
            .mock("POST", "/tx")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_tx_data)
            .with_body_from_request(|_request| {
                "c9ec56ecc714e2ec33d51519c647d6adb8469afcbd4b2a6a8052c7db29a00da2".into()
            })
            .create();

        let expected_txid = "c9ec56ecc714e2ec33d51519c647d6adb8469afcbd4b2a6a8052c7db29a00da2";
        // check that the txid is correct
        let bs = Blockstream::new(&server.url()).unwrap();
        let txid = bs.post_a_transaction(raw_tx_data).await.unwrap();
        assert_eq!(txid, expected_txid);
    }
}
