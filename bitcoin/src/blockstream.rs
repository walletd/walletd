use std::any::Any;

use async_trait::async_trait;
use bitcoin::script::PushBytes;
use bitcoin::{Address, AddressType};
use bitcoin_hashes::{sha256d, Hash};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use walletd_coin_model::BlockchainConnectorGeneral;
use walletd_coin_model::{BlockchainConnector, CryptoWallet};
use crate::BitcoinWallet;

use time::{OffsetDateTime, Duration};
use time::format_description::well_known::Rfc2822;

use prettytable::Table;
use prettytable::row;
use std::fmt;

use walletd_coin_model::CryptoAddress;

use crate::BitcoinAmount;

use crate::BitcoinAddress;
pub use bitcoin::{
     sighash::EcdsaSighashType, Network, PrivateKey as BitcoinPrivateKey,
    PublicKey as BitcoinPublicKey, Script,
};

use bitcoin::blockdata::script::Builder;

use crate::Error;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BTransaction {
    #[serde(default)]
    pub txid: String,
    #[serde(default)]
    pub version: i32,
    #[serde(default)]
    pub locktime: u32,
    #[serde(default)]
    pub vin: Vec<Input>,
    #[serde(default)]
    pub vout: Vec<Output>,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub weight: u64,
    #[serde(default)]
    pub fee: u64,
    #[serde(default)]
    pub status: Status,
}


impl BTransaction {
    pub async fn overview(btc_wallet: BitcoinWallet) -> Result<String, Error> {
        
        // We need to know which addresses belong to our wallet
        let our_addresses = btc_wallet.addresses().iter().map(|address| address.public_address()).collect::<Vec<String>>();
        let blockchain_client = btc_wallet.blockchain_client()?;
         let mut transactions: Vec<BTransaction> = Vec::new();
         let mut owners_addresses = Vec::new();
        for address in &our_addresses {
            let txs = blockchain_client.transactions(address).await?;
           
            for tx in txs {
                if transactions.iter().any(|x| x.txid == tx.txid) {
                    continue
                }
                transactions.push(tx);
                owners_addresses.push(address.clone());
            }
        }

        // sort the transactions by the block_time
        transactions.sort_by(|a, b| a.status.block_time.cmp(&b.status.block_time));
        let mut table = Table::new();
        // Amount to display is the change in the running balance
        table.add_row(row!["Transaction ID", "Amount (BTC)", "To/From Address", "Status", "Timestamp"]);
        for i in 0..transactions.len() {
            let our_inputs: Vec<Output> = transactions[i].vin.iter().filter(|input| owners_addresses.contains(&input.prevout.scriptpubkey_address)).map(|x| x.prevout.clone()).collect();
            let received_outputs: Vec<Output> = transactions[i].vout.iter().filter(|output| owners_addresses.contains(&output.scriptpubkey_address)).cloned().collect();
            let received_amount = BitcoinAmount::new_from_satoshi(received_outputs.iter().fold(0, |acc, output| acc + output.value));
            let sent_amount = BitcoinAmount::new_from_satoshi(our_inputs.iter().fold(0, |acc, output| acc + output.value));
          
            let amount_balance = if received_amount > sent_amount {
                // this is situation when we are receiving money
                (received_amount - sent_amount).btc()
            }
            else {
                // this is the situation where we are sending money
                (sent_amount - received_amount).btc() * -1.0
            };

            let status_string = if transactions[i].status.confirmed {
                "Confirmed".to_string()
            } else {
                "Pending Confirmation".to_string()
            };
            let timestamp = transactions[i].status.timestamp()?;
            
            
            table.add_row(row![transactions[i].txid, amount_balance, owners_addresses[i], status_string, timestamp]);

        }
        Ok(table.to_string())
    }

    pub fn details(&self, owners_address: String) -> Result<String, Error> {
        let mut table_string = String::new();
        let mut table = Table::new();
        table.add_row(row!["Tx ID", self.txid]);
        table_string.push_str(&table.to_string());
        table = Table::new();
        table.add_row(row!["Status", self.status]);
        table_string.push_str(&table.to_string());
        let input_amount = BitcoinAmount::new_from_satoshi(self.vin.iter().fold(0, |acc, input| acc + input.prevout.value)).btc();
        let output_amount = BitcoinAmount::new_from_satoshi(self.vout.iter().fold(0, |acc, output| acc + output.value)).btc();
        let change_amount = BitcoinAmount::new_from_satoshi(self.vout.iter().filter(|output| output.scriptpubkey_address == owners_address).fold(0, |acc, output| acc + output.value)).btc();
        let amount = input_amount - output_amount + change_amount;
        let fee_amount = BitcoinAmount::new_from_satoshi(self.fee).btc();
        table = Table::new();
        table.add_row(row!["Amount: ", amount.to_string()]);
        table.add_row(row!["Fee Amount: ", fee_amount.to_string()]);
        table.add_row(row!["To/From Address: ", owners_address]);
        table.add_row(row!["Version", self.version]);
        table.add_row(row!["Locktime", self.locktime]);
        table.add_row(row!["Size", self.size]);
        table.add_row(row!["Weight", self.weight]);
        table_string.push_str(&table.to_string());
        
        table = Table::new();
        table.add_row(row!["Inputs"]);
        for input in &self.vin {
            table.add_row(row![input.to_string()]);
        }
        table_string.push_str(&table.to_string());
        table = Table::new();
        table.add_row(row!["Outputs"]);
        for output in &self.vout {
            table.add_row(row![output.to_string()]);
        }
        table_string.push_str(&table.to_string());
        Ok(table_string)
    }


     // TODO(AS): add doc here
     pub fn prepare_transaction(
        fee_sat_per_byte: f64,
        utxo_available: &Vec<Utxo>,
        inputs_available_tx_info: &[BTransaction],
        send_amount: &BitcoinAmount,
        receiver_view_wallet: &BitcoinAddress,
        change_addr: Address
    ) -> Result<(BTransaction, Vec<usize>), Error> {
        // choose inputs
        let (inputs, fee_amount, chosen_indices)= BitcoinAddress::choose_inputs_and_set_fee(
            utxo_available,
            send_amount,
            inputs_available_tx_info,
            fee_sat_per_byte,
        )?;
        let inputs_amount = BitcoinAmount {
            satoshi: inputs.iter().map(|x| x.prevout.value).sum(),
        };
        if inputs_amount < (*send_amount + fee_amount) {
            return Err(Error::InsufficientFunds("Insufficient funds to send amount and cover fees".into()));
        }

      
        let change_amount = inputs_amount - *send_amount - fee_amount;

        // Create two outputs, one for the send amount and another for the change amount
        // Hardcoding p2wpkh SegWit transaction option
        // TODO(#83) right away need to add the scriptpubkey info
        let mut outputs: Vec<Output> = Vec::new();
        let mut output_send = Output {
            ..Default::default()
        };
        output_send.value = send_amount.satoshi();
        output_send.set_scriptpubkey_info(receiver_view_wallet.address_info())?;
        outputs.push(output_send);
        let mut output_change = Output {
            ..Default::default()
        };
        output_change.value = change_amount.satoshi();
        output_change.set_scriptpubkey_info(change_addr)?;
        outputs.push(output_change);

        let mut transaction = BTransaction {
            ..Default::default()
        };
        transaction.version = 1;
        transaction.locktime = 0;
        transaction.vin = inputs;
        transaction.vout = outputs.clone();
        transaction.fee = fee_amount.satoshi();

        Ok((transaction, chosen_indices))    
    }


    pub fn sign_tx(&self, keys_per_input: Vec<(BitcoinPrivateKey, BitcoinPublicKey)>) -> Result<Self, Error> {
        let mut inputs = self.vin.clone();
        // Signing and unlocking the inputs
        for (i, input) in inputs.iter_mut().enumerate() {
            // hardcoded default to SIGHASH_ALL
            let sighash_type = EcdsaSighashType::All;
            let transaction_hash_for_input_with_sighash = self
                .transaction_hash_for_signing_segwit_input_index(i, sighash_type.to_u32())?;
            let private_key = &keys_per_input[i].0;
            let public_key= &keys_per_input[i].1;
            let sig_with_hashtype = BitcoinAddress::signature_sighashall_for_transaction_hash(
                &transaction_hash_for_input_with_sighash,
                private_key
            )?;

            let sig_with_hashtype_vec = hex::decode(&sig_with_hashtype)?;
            let sig_with_hashtype_bytes: &PushBytes = sig_with_hashtype_vec.as_slice().try_into()?;
            
            // handle the different types of inputs based on previous locking script
            let prevout_lockingscript_type = &input.prevout.scriptpubkey_type;
            match prevout_lockingscript_type.as_str() {
                "p2pkh" => {
                    let script_sig = Builder::new()
                        .push_slice(sig_with_hashtype_bytes)
                        .push_key(public_key)
                        .into_script();
                    input.scriptsig_asm = script_sig.to_asm_string();
                    input.scriptsig = hex::encode(script_sig.as_bytes());
                }
                "p2sh" => {
                    // TODO(#83) need to handle redeem scripts
                    return Err(Error::CurrentlyNotSupported("Not currently handling P2SH".into()));
                }
                "v0_p2wsh" => {
                    // TODO(#83) need to handle redeem scripts
                    return Err(Error::CurrentlyNotSupported("Not currently handling v0_p2wsh".into()));
                }
                "v0_p2wpkh" => {
                    // Need to specify witness data to unlock
                    input.witness = vec![sig_with_hashtype, hex::encode(public_key.to_bytes())];
                }
                _ => {
                    return Err(Error::CurrentlyNotSupported("Unidentified locking script type from previous output".into()))
                }
            }
        }
        let mut signed_tx = self.clone();
        signed_tx.vin = inputs;
        Ok(signed_tx)
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Output {
    #[serde(default)]
    pub scriptpubkey: String,
    #[serde(default)]
    pub scriptpubkey_asm: String,
    #[serde(default)]
    pub scriptpubkey_type: String,
    #[serde(default)]
    pub scriptpubkey_address: String,
    #[serde(default)]
    pub pubkeyhash: String,
    #[serde(default)]
    pub value: u64,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        if !self.scriptpubkey.is_empty() {
            table.add_row(row!["ScriptPubKey", self.scriptpubkey]);
        }
        if !self.scriptpubkey_asm.is_empty() {
            table.add_row(row!["ScriptPubKey ASM", self.scriptpubkey_asm]);
        }
        if !self.scriptpubkey_type.is_empty() {
            table.add_row(row!["ScriptPubKey Type", self.scriptpubkey_type]);
        }
        if !self.scriptpubkey_address.is_empty() {
            table.add_row(row!["ScriptPubKey Address", self.scriptpubkey_address]);
        }
        if !self.pubkeyhash.is_empty() {
            table.add_row(row!["PubKeyHash", self.pubkeyhash]);
        }
        table.add_row(row!["Value (BTC)", BitcoinAmount::new_from_satoshi(self.value).btc()]);
        write!(f, "{}", table)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    #[serde(default)]
    pub txid: String,
    #[serde(default)]
    pub vout: u32,
    #[serde(default)]
    pub prevout: Output,
    #[serde(default)]
    pub scriptsig: String,
    #[serde(default)]
    pub scriptsig_asm: String,
    #[serde(default)]
    pub witness: Vec<String>,
    #[serde(default)]
    pub is_coinbase: bool,
    #[serde(default)]
    pub sequence: u32,
    #[serde(default)]
    pub inner_redeemscript_asm: String,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["Input Tx ID", self.txid]);
        table.add_row(row!["Amount (BTC)", BitcoinAmount::new_from_satoshi(self.vout as u64).btc()]);
        if !self.scriptsig.is_empty() {
            table.add_row(row!["ScriptSig", self.scriptsig]);
        }
        if !self.scriptsig_asm.is_empty() {
            table.add_row(row!["ScriptSig ASM", self.scriptsig_asm]);
        }
        if !self.witness.is_empty() {
            table.add_row(row!["Witness", self.witness.join(" ")]);
        }
        if !self.inner_redeemscript_asm.is_empty() {
            table.add_row(row!["Inner Redeemscript ASM", self.inner_redeemscript_asm]);
        }
        table.add_row(row!["Is Coinbase", self.is_coinbase]);
        table.add_row(row!["Sequence", self.sequence]);
        write!(f, "{}", table)
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Status {
    #[serde(default)]
    pub confirmed: bool,
    #[serde(default)]
    pub block_height: u32,
    #[serde(default)]
    pub block_hash: String,
    #[serde(default)]
    pub block_time: u32,
}

impl Status {

    pub fn timestamp(&self) -> Result<String, Error> {
        if self.confirmed {
        // Creates a timestamp from the specified number of whole seconds which have passed since the UNIX_EPOCH
       match OffsetDateTime::UNIX_EPOCH.checked_add(Duration::new(self.block_time.into(), 0)) {
        // Formats the combined date and time with the specified format string.
        Some(timestamp) => {let formatted_timestamp = timestamp.format(&Rfc2822)?;
        Ok(formatted_timestamp)}
        None => Err(Error::Timestamp("Overflow error when converting timestamp".into()))
       }
              
        }
        else {
            Ok("".to_string())
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       let mut table = Table::new();
       table.add_row(row!["Confirmed: ", self.confirmed]);
         table.add_row(row!["Block Height: ", self.block_height]);
            table.add_row(row!["Block Hash: ", self.block_hash]);
                table.add_row(row!["Timestamp ", self.timestamp().unwrap()]);
                write!(f, "{}", table)
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Utxo {
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub txid: String,
    #[serde(default)]
    pub value: u64,
    #[serde(default)]
    pub vout: u32,
}

pub enum InputType {
    P2pkh,
    P2sh,
    P2wsh,
    P2wpkh,
    P2sh2Wpkh,
    P2sh2Wsh,
}

impl InputType {
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
            _ => Err(Error::CurrentlyNotSupported("Unknown scriptpubkey_type, not currently handled".into())),
        }
    }

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
                    "The references txid in hex format should be 64 characters long".into()
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
                "The references txid in hex format should be 64 characters long".into()
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
                return Err(Error::ScriptInvalid("Length of scriptpubkey should be a multiple of 2".into()));
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
                    "The reference txid in hex format should be 64 characters long".into()
                ));
            }
            let prev_txid_encoded = Self::hex_reverse_byte_order(prev_txid)?;
            serialization.push_str(prev_txid_encoded.as_str());
            let prev_vout: u32 = input.vout;
            let prev_vout_encoded = &prev_vout.to_le_bytes();
            serialization.push_str(&hex::encode(prev_vout_encoded));
            let len_signature_script = input.scriptsig.len();
            if len_signature_script % 2 != 0 {
                return Err(Error::ScriptInvalid("Length of script_sig should be a multiple of 2".into()));
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
                
                return Err(Error::ScriptInvalid("Length of scriptpubkey should be a multiple of 2".into()));
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
                            "Witness data length in hex should be a multiple of 2".into()
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

    pub fn hex_reverse_byte_order(hex_string: &String) -> Result<String, Error> {
        let len = hex_string.len();
        if len % 2 != 0 {
            return Err(Error::ScriptInvalid(
                "The hex string should have a length that is a multiple of 2".into()
            ));
        }
        let mut encoded = String::new();
        for i in 0..len / 2 {
            let reverse_ind = len - i * 2 - 2;
            encoded.push_str(&hex_string[reverse_ind..reverse_ind + 2]);
        }
        Ok(encoded)
    }

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
                    "Currently not implemented setting scriptpubkey for this address type".into()
                ))
            }
        }
        let script_pubkey = address_info.script_pubkey();
        self.scriptpubkey_asm = script_pubkey.to_asm_string();
        self.scriptpubkey = hex::encode(script_pubkey.as_bytes());
        Ok(())
    }
}

#[derive(Clone, Default, Debug)]
pub struct Blockstream {
    pub client: reqwest::Client,
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

    async fn display_fee_estimates(&self) -> Result<String, Error> {
        let fee_estimates: FeeEstimates = self.fee_estimates().await?;
        let fee_string = format!("{}", fee_estimates);
        Ok(fee_string)
    }
}

impl BlockchainConnectorGeneral for Blockstream {
    fn as_any(&self) -> &dyn Any {
        self
    }


    fn box_clone(&self) -> Box<dyn BlockchainConnectorGeneral> {
        Box::new(self.clone())
    }

}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct FeeEstimates(pub serde_json::Map<String, Value>);


impl fmt::Display for FeeEstimates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        writeln!(f, "Fee Estimates")?;
        table.add_row(row!["Confirmation Target (Blocks)", "Fee (sat/vB)"]);
        let mut keys = self.0.iter().map(|(a, _b)| a.parse::<u32>().expect("expecting that key should be able to be parsed as u32")).collect::<Vec<_>>();
        keys.sort();
        for key in keys {
           table.add_row(row![key, self.0[&key.to_string()]]);
        }
        write!(f, "{}", table)?;
        Ok(())
    }
}

impl TryFrom <Box<dyn BlockchainConnectorGeneral>> for Blockstream {
    type Error = Error;

    fn try_from(blockchain_connector: Box<dyn BlockchainConnectorGeneral>) -> Result<Self, Self::Error> {
        match blockchain_connector.as_any().downcast_ref::<Blockstream>() {
            Some(blockstream) => Ok(blockstream.clone()),
            None =>  Err(Error::UnableToDowncastBlockchainConnector("Could not convert BlockchainConnector to Blockstream".into())),
        }
    }
}

impl TryFrom <&Box<dyn BlockchainConnectorGeneral>> for Blockstream {
    type Error = Error;

    fn try_from(blockchain_connector: &Box<dyn BlockchainConnectorGeneral>) -> Result<Self, Self::Error> {
        match blockchain_connector.as_any().downcast_ref::<Blockstream>() {
            Some(blockstream) => Ok(blockstream.clone()),
            None =>  Err(Error::UnableToDowncastBlockchainConnector("Could not convert BlockchainConnector to Blockstream".into())),
        }
    }
}


impl Blockstream {
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
        let block_count = body.parse::<u64>().map_err(|e| Error::FromStr(e.to_string()))?;
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
        let transactions: Vec<BTransaction>= serde_json::from_str(&body)?;
        Ok(transactions)
    }

    /// Fetch mempool transactions from blockstream
    pub fn mempool_transactions(&self, address: &str) -> Result<Value, Error> {
        let body = reqwest::blocking::get(format!("{}/address/{}/txs/mempool", self.url, address))
            .expect("Error getting transactions")
            .text();
        let transactions = json!(&body?);
        Ok(transactions)
    }

    /// Fetch UTXOs from blockstream
    pub async fn utxo(&self, address: &str) -> Result<Vec<Utxo>, Error> {
        let body = reqwest::get(format!("{}/address/{}/utxo", self.url, address))
            .await?
            .text()
            .await?;

        let utxos: Vec<Utxo> = serde_json::from_str(&body)?;
        Ok(utxos)
    }

    pub async fn get_raw_transaction_hex(&self, txid: &str) -> Result<String, Error> {
        let body = reqwest::get(format!("{}/tx/{}/raw", self.url, txid))
            .await?
            .text()
            .await?;
        let raw_transaction_hex = json!(&body);
        Ok(raw_transaction_hex.to_string())
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
            .await
            .expect("Transaction failed to be posted");

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
    use mockito::mock;

    use super::*;

    #[test]
    fn test_block_count() {
        let _m = mock("GET", "/blocks/tip/height")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("773876")
            .create();

        let _url: &String = &mockito::server_url();
        let bs = Blockstream::new(&mockito::server_url()).unwrap();
        let check = bs.block_count().unwrap();
        assert_eq!(773876, check);
    }
}
