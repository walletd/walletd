use std::cmp::Reverse;
use std::str::FromStr;
use std::fmt;

use bitcoin::script::PushBytes;
use ::secp256k1::{Message, Secp256k1, SecretKey};
use base58::ToBase58;
use bitcoin::blockdata::script::Builder;
pub use bitcoin::{
    Address as AddressInfo, AddressType, sighash::EcdsaSighashType, Network, PrivateKey as BitcoinPrivateKey,
    PublicKey as BitcoinPublicKey, Script,
};
use sha2::{Digest, Sha256};
use walletd_coin_model::{CryptoAddress};
use walletd_hd_key::{HDKey, HDNetworkType};
use crate::FeeEstimates;

use crate::blockstream::{BTransaction, Blockstream, Input, InputType, Output, Utxo};
use crate::BitcoinAmount;
use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitcoinAddress {
    address_info: AddressInfo,
    private_key: Option<BitcoinPrivateKey>,
    public_key: Option<BitcoinPublicKey>,
    network: Network,
}

impl BitcoinAddress {

    pub fn from_hd_key(hd_key: &HDKey, address_format: AddressType) -> Result<Self, Error> {
         // TODO(#82): consider handling the other Bitcoin network types
         let network: Network = match hd_key.network {
            HDNetworkType::MainNet => Network::Bitcoin,
            HDNetworkType::TestNet => Network::Testnet,
        };
        let public_key_bytes = &hd_key
            .extended_public_key
            .expect("Public key data missing")
            .to_bytes();
        
            let private_key_bytes = hd_key.extended_private_key().expect("Private key data missing").to_bytes();
        let public_key = BitcoinPublicKey::from_slice(public_key_bytes)?;
        let private_key = BitcoinPrivateKey::from_slice(&private_key_bytes, network)?;
       

        let address_info: AddressInfo = match address_format {
            AddressType::P2pkh => AddressInfo::p2pkh(&public_key, network),
            AddressType::P2sh => AddressInfo::p2sh(Script::empty(), network)?,
            AddressType::P2wpkh => AddressInfo::p2wpkh(&public_key, network)?,
            AddressType::P2wsh => AddressInfo::p2wsh(Script::empty(), network),
            // Currently not handling the AddressType::P2tr, fix if can understand how to create
            // this address properly
            _ => return Err(Error::CurrentlyNotSupported("Currently not handling this Bitcoin address type".into())),
        };
        
        Ok(Self {
            address_info,
            private_key: Some(private_key),
            public_key: Some(public_key),
            network,
        })
    }

    pub async fn balance(
        &self,
        blockchain_client: &Blockstream,
    ) -> Result<BitcoinAmount, Error> {
        let utxo_info = blockchain_client
            .utxo(&self.public_address())
            .await?;
        let amount = Self::confirmed_balance_from_utxo(utxo_info)?;

        Ok(amount)
    }

    pub async fn transfer(
        &self,
        client: &Blockstream,
        send_amount: &BitcoinAmount,
        public_address: &str,
    ) -> Result<String, Error> {
       
        let receiver_view_wallet = Self::from_public_address(public_address, self.network)?;

        // first checking existing endpoints with blockstream
        let fee_estimates: FeeEstimates = client.fee_estimates().await?;
        let confirmation_target: u32 = 6; // this variable specifies how many blocks need to include this transaction
                                          // before it's considered "confirmed"
        let fee_map = &fee_estimates.0;
        let fee_sat_per_byte: f64 = if !fee_map.is_empty() {
            fee_map
                .get(confirmation_target.to_string().as_str())
                .expect("fee_map missing key")
                .as_f64()
                .expect("Unable to convert to f64")
            
        } else {
            return Err(Error::MissingFeeMap);
        };

        // Build the transaction
        // Specify the inputs and outputs, the difference between the amount of the
        // inputs and the amount of the outputs is the transaction fee
        // Input(s) should cover the total amount
        // Inputs need to come from the utxo
        let available_utxos = client.utxo(self.public_address().as_str()).await?;

        // sum total value with confirmed status, also count number of utxos with
        // confirmed status
        let mut total_value_from_utxos = 0;
        let mut inputs_available: Vec<Utxo> = Vec::new();
        let mut inputs_available_tx_info: Vec<BTransaction> = Vec::new();
        for utxo in available_utxos {
            if utxo.status.confirmed {
                total_value_from_utxos += &utxo.value;
                let tx_info = client.transaction(utxo.txid.as_str()).await?;
                inputs_available.push(utxo);
                inputs_available_tx_info.push(tx_info);
            }
        }

        let available_input_max = BitcoinAmount {
            satoshi: total_value_from_utxos,
        };

        if available_input_max < *send_amount {
            return Err(Error::InsufficientFunds("Insufficient funds".into()));
        }
        let transaction = self.build_transaction(
            fee_sat_per_byte,
            &inputs_available,
            &inputs_available_tx_info,
            send_amount,
            &receiver_view_wallet,
        )?;

        let transaction_hex = BTransaction::serialize(&transaction)?;
        let raw_transaction_hex: &'static str = Box::leak(transaction_hex.into_boxed_str());
        let txid = client.post_a_transaction(raw_transaction_hex).await?;
        Ok(txid)
    }

}

impl CryptoAddress for BitcoinAddress {
    fn public_address(&self) -> String {
        self.address_info.to_string()
    }
}


impl BitcoinAddress {
    pub fn from_public_address(public_address: &str, network: Network) -> Result<Self, Error> {
        let address_info = AddressInfo::from_str(public_address)?.require_network(network)?;
        Ok(Self {
            address_info,
            private_key: None,
            public_key: None,
            network,
        })
    }

    pub fn public_key(&self) -> Result<BitcoinPublicKey, Error> {
        if let Some(key) = self.public_key {
            Ok(key)
        } else {
            Err(Error::MissingPublicKey)
        }
    }

    pub fn private_key(&self) -> Result<BitcoinPrivateKey, Error> {
        if let Some(key) = self.private_key {
            Ok(key)
        } else {
            Err(Error::MissingPrivateKey)
        }
    }

    /// This function is used to return the signature with the option sighashall for a given transaction hash using a private key
    // TODO(AS): Consider refactoring this when refactoring the walletd_bitcoin crate
    pub fn signature_sighashall_for_transaction_hash(
        transaction_hash: &str,
        private_key: &BitcoinPrivateKey,
    ) -> Result<String, Error> {
        // hardcoded default to SIGHASH_ALL
        let sighash_type = EcdsaSighashType::All;
        let secp = Secp256k1::new();
        let message = Message::from_slice(&hex::decode(transaction_hash)?).expect("32 bytes");
        let mut sig = secp.sign_ecdsa(&message, &SecretKey::from_slice(&private_key.to_bytes())?);
        sig.normalize_s();
        let mut sig_with_hashtype = sig.serialize_der().to_vec();
        sig_with_hashtype.push(sighash_type.to_u32().try_into()?);
        let content_len_index = 1;
        let mut len_content = sig_with_hashtype[content_len_index];
        let r_len_index = 3;
        let mut len_r = sig_with_hashtype[r_len_index];
        let r_first_byte = sig_with_hashtype[r_len_index + 1];
        if r_first_byte == 0 {
            let r_second_byte = sig_with_hashtype[r_len_index + 2];
            if r_second_byte < 0x80 {
                len_r -= 1;
                len_content -= 1;
                sig_with_hashtype.remove(r_len_index + 1); // removing first byte if not significant
                sig_with_hashtype[content_len_index] = len_content;
                sig_with_hashtype[r_len_index] = len_r;
            }
        }
        let s_len_index: usize = (3 + len_r + 1 + 1).into();
        let mut len_s = sig_with_hashtype[s_len_index];
        let s_first_byte = sig_with_hashtype[s_len_index + 1];
        if s_first_byte == 0 {
            let s_second_byte = sig_with_hashtype[s_len_index + 2];
            if s_second_byte < 0x80 {
                len_s -= 1;
                len_content -= 1;
                sig_with_hashtype.remove(s_len_index + 1);
                sig_with_hashtype[content_len_index] = len_content;
                sig_with_hashtype[s_len_index] = len_s;
            }
        }
        let signature = hex::encode(&sig_with_hashtype);
        Ok(signature)
    }

    /// Estimates the fee for a transaction with the given number of inputs and outputs given the fee per byte, makes use of default sizes to estimate the size of the tranasaction and the corresponding fee
    pub fn estimate_fee_with_default_sizes(
        is_segwit: bool,
        num_inputs: usize,
        num_outputs: usize,
        byte_fee: f64,
    ) -> Result<u64, Error> {
        const NONSEGWIT_DEFAULT_BYTES_PER_INPUT: usize = 148;
        const NONSEGWIT_DEFAULT_BYTES_PER_OUTPUT: usize = 34;
        const NONSEGWIT_DEFAULT_BYTES_BASE: usize = 10;
        const SEGWIT_DEFAULT_BYTES_PER_INPUT: usize = 102;
        const SEGWIT_DEFAULT_BYTES_PER_OUTPUT: usize = 31;
        const SEGWIT_DEFAULT_BYTES_BASE: usize = 10;

        if is_segwit {
            let tx_size = (num_inputs * NONSEGWIT_DEFAULT_BYTES_PER_INPUT)
                + (num_outputs * NONSEGWIT_DEFAULT_BYTES_PER_OUTPUT)
                + NONSEGWIT_DEFAULT_BYTES_BASE;
            let estimated_fee = f64::ceil(byte_fee * (tx_size as f64)) as u64;
            Ok(estimated_fee)
        } else {
            let tx_size = (num_inputs * SEGWIT_DEFAULT_BYTES_PER_INPUT)
                + (num_outputs * SEGWIT_DEFAULT_BYTES_PER_OUTPUT)
                + SEGWIT_DEFAULT_BYTES_BASE;
            let estimated_fee = f64::ceil(byte_fee * (tx_size as f64)) as u64;
            Ok(estimated_fee)
        }
    }

    /// Goal is to find a combination of the fewest inputs that is bigger than
    /// what we need - close to twice the send amount while not producing a
    /// change amount that is smaller than what the fee would be to spend that
    /// amount
    pub fn choose_inputs_and_set_fee(
        utxo_available: &Vec<Utxo>,
        send_amount: &BitcoinAmount,
        inputs_available_tx_info: &[BTransaction],
        byte_fee: f64,
    ) -> Result<(Vec<Input>, BitcoinAmount, Vec<usize>), Error> {
        // Sorting in reverse order of the value each UTXO (from highest UTXO value to
        // lowest), indices keeps track of the original indices after sort
        let mut indices = (0..utxo_available.len()).collect::<Vec<_>>();
        indices.sort_by_key(|&i| Reverse(&utxo_available[i].value));
        let mut chosen_indices = Vec::new();
        let mut inputs: Vec<Input> = Vec::new();
        let min_goal_target = *send_amount * 1.5;
        let mut obtained_amount = BitcoinAmount { satoshi: 0 };
        let mut met_goal = false;
        let mut segwit_transaction = false;

        for ind in &indices {
            let utxo = &utxo_available[*ind];
            let utxo_prevout = &inputs_available_tx_info[*ind].vout[utxo.vout as usize];
            if !segwit_transaction && InputType::new(utxo_prevout)?.is_segwit() {
                segwit_transaction = true;
            }
            let value = BitcoinAmount {
                satoshi: utxo.value,
            };
            obtained_amount += value;
            let mut input = Input {
                ..Default::default()
            };
            let input_tx_info = &inputs_available_tx_info[*ind];
            let input_utxo = &utxo_available[*ind];
            input.txid = input_tx_info.txid.to_owned();
            input.vout = input_utxo.vout;
            input.prevout = utxo_prevout.to_owned();

            // parsing and storing the hash of the pubkey value, useful later
            for command in input
                .prevout
                .scriptpubkey_asm
                .split_whitespace()
                .collect::<Vec<_>>()
                .iter()
            {
                let mut chars = command.chars();
                let first_char = chars.next();
                let second_char = chars.next();
                if let Some(first) = first_char {
                    if let Some(second) = second_char {
                        if first != 'O' && second != 'P' {
                            input.prevout.pubkeyhash = command.to_string();
                        }
                    }
                }
            }

            inputs.push(input);
            chosen_indices.push(*ind);

            if obtained_amount > min_goal_target {
                met_goal = true;
                break;
            }
        }

        if met_goal {
            let change_and_fee_amount = obtained_amount - *send_amount;
            // estimate fee
            let num_inputs = inputs.len();
            let num_outputs = 2; // one output to send, one output for change
            let set_fee = BitcoinAmount {
                satoshi: Self::estimate_fee_with_default_sizes(
                    segwit_transaction,
                    num_inputs,
                    num_outputs,
                    byte_fee,
                )?,
            };
            let change_amount = change_and_fee_amount - set_fee;
            let min_change_amount = BitcoinAmount {
                satoshi: Self::estimate_fee_with_default_sizes(segwit_transaction, 1, 0, byte_fee)?,
            };
            if change_amount > min_change_amount {
                // Met the goal, return the inputs collected
                Ok((inputs, set_fee, chosen_indices))
            }
            // initial change amount was not greater than the min_change_amount
            else {
                // Are any other utxos available?
                if inputs.len() < utxo_available.len() {
                    // Add more until change amount will be greater than min_change_amount
                    let wanted_extra = min_change_amount - change_amount;
                    let min_goal_target = obtained_amount + wanted_extra;
                    let start = inputs.len();
                    for ind in &indices[start..] {
                        let utxo = &utxo_available[*ind];
                        let utxo_prevout = &inputs_available_tx_info[*ind].vout[utxo.vout as usize];
                        if !segwit_transaction && InputType::new(utxo_prevout)?.is_segwit() {
                            segwit_transaction = true;
                        }
                        let value = BitcoinAmount {
                            satoshi: utxo.value,
                        };
                        obtained_amount += value;
                        let mut input = Input {
                            ..Default::default()
                        };
                        let input_tx_info = &inputs_available_tx_info[*ind];
                        let input_utxo = &utxo_available[*ind];
                        input.txid = input_tx_info.txid.clone();
                        input.vout = input_utxo.vout;
                        input.prevout = utxo_prevout.to_owned();

                        // parsing and storing the hash of the pubkey value, useful later
                        for command in input
                            .prevout
                            .scriptpubkey_asm
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .iter()
                        {
                            let mut chars = command.chars();
                            let first_char = chars.next();
                            let second_char = chars.next();
                            if let Some(first) = first_char {
                                if let Some(second) = second_char {
                                    if first != 'O' && second != 'P' {
                                        input.prevout.pubkeyhash = command.to_string();
                                    }
                                }
                            }
                        }
                        inputs.push(input);
                        chosen_indices.push(*ind);

                        if obtained_amount > min_goal_target {
                            return Ok((inputs, set_fee, chosen_indices));
                        }
                    }
                    // even if could not get the change amount to be greater than the min change
                    // amount, still proceed by including the added inputs
                    return Ok((inputs, set_fee, chosen_indices));
                }
                // even if could not get the change amount to be greater than the min change
                // amount, still proceed
                Ok((inputs, set_fee, chosen_indices))
            }
        } else {
            // did not meet goal (there are no more utxos to use to meet goal)
            // checked if obtained amount sufficient to pay fee
            // estimate fee
            let num_inputs = inputs.len();
            let num_outputs = 2; // one output to send, one output for change
            let set_fee = BitcoinAmount {
                satoshi: Self::estimate_fee_with_default_sizes(
                    segwit_transaction,
                    num_inputs,
                    num_outputs,
                    byte_fee,
                )?,
            };
            if obtained_amount > *send_amount + set_fee {
                Ok((inputs, set_fee, chosen_indices))
            } else {
                Err(Error::InsufficientFunds("Not enough funds to cover the send amount as well as the fee needed".into()))
            }
        }
    }

    /// Builds a transaction with inputs and outputs, taking into account a fee
    /// amount and a change amount.
    /// Uses the rust-bitcoin library and structs/functions
    pub fn build_transaction(
        &self,
        fee_sat_per_byte: f64,
        utxo_available: &Vec<Utxo>,
        inputs_available_tx_info: &[BTransaction],
        send_amount: &BitcoinAmount,
        receiver_view_wallet: &BitcoinAddress,
    ) -> Result<BTransaction, Error> {
        // choose inputs
        let (mut inputs, fee_amount, _chosen_inds) = Self::choose_inputs_and_set_fee(
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

        log::info!(
            "inputs_amount: {} BTC, send_amount: {} BTC, fee_amount {} BTC",
            &inputs_amount.btc(),
            &send_amount.btc(),
            &fee_amount.btc()
        );
        log::info!(
            "inputs_amount: {} sat, send_amount: {} sat, fee_amount {} sat",
            &inputs_amount.satoshi(),
            &send_amount.satoshi(),
            &fee_amount.satoshi()
        );
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
        output_change.set_scriptpubkey_info(self.address_info())?;
        outputs.push(output_change);

        let mut transaction = BTransaction {
            ..Default::default()
        };
        transaction.version = 1;
        transaction.locktime = 0;
        transaction.vin = inputs.clone();
        transaction.vout = outputs.clone();
        transaction.fee = fee_amount.satoshi();

        // Signing and unlocking the inputs
        for (i, input) in inputs.iter_mut().enumerate() {
            // hardcoded default to SIGHASH_ALL
            let sighash_type = EcdsaSighashType::All;
            let transaction_hash_for_input_with_sighash = transaction
                .transaction_hash_for_signing_segwit_input_index(i, sighash_type.to_u32())?;
            let private_key = self.private_key
                    .expect("Private key data missing");
            let sig_with_hashtype = Self::signature_sighashall_for_transaction_hash(
                &transaction_hash_for_input_with_sighash,
                &private_key
            )?;

            let sig_with_hashtype_vec = hex::decode(&sig_with_hashtype)?;
            let sig_with_hashtype_bytes: &PushBytes = sig_with_hashtype_vec.as_slice().try_into()?;

            // handle the different types of inputs based on previous locking script
            let prevout_lockingscript_type = &input.prevout.scriptpubkey_type;
            match prevout_lockingscript_type.as_str() {
                "p2pkh" => {
                
                    let script_sig = Builder::new()
                        .push_slice(sig_with_hashtype_bytes)
                        .push_key(&self.public_key()?)
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
                    input.witness = vec![sig_with_hashtype, format!("{:x}", self.public_key()?.inner)];
                }
                _ => {
                    return Err(Error::CurrentlyNotSupported("Unidentified locking script type from previous output".into()))
                }
            }
        }
        transaction.vin = inputs;

        Ok(transaction)
    }

    pub fn confirmed_balance_from_utxo(
        utxo_info: Vec<Utxo>,
    ) -> Result<BitcoinAmount, Error> {
        let mut satoshis: u64 = 0;
        for item in utxo_info {
            satoshis += item.value;
        }
        let confirmed_balance = BitcoinAmount { satoshi: satoshis };
        Ok(confirmed_balance)
    }

    pub fn address_info(&self) -> AddressInfo {
        self.address_info.clone()
    }

    pub fn public_address_p2pkh_from_public_key(public_key: &[u8]) -> String {
        // p2pkh format
        let mut address = [0u8; 25];

        address[0] = 0x00;
        address[1..21].copy_from_slice(&HDKey::hash160(public_key));

        let checksum = &(Sha256::digest(Sha256::digest(&address[0..21]).as_slice()).to_vec())[0..4];
        address[21..25].copy_from_slice(checksum);
        address.to_base58()
    }
}

impl fmt::Display for BitcoinAddress {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.public_address())?;   
            Ok(())
        }
    }


