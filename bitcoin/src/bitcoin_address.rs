use core::fmt;
use core::fmt::Display;
use std::any::Any;
use std::cmp::Reverse;
use std::str::FromStr;

use ::secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use anyhow::anyhow;
use async_trait::async_trait;
use base58::ToBase58;
use bitcoin::blockdata::script::Builder;
pub use bitcoin::{
    Address, AddressType, EcdsaSighashType, Network, PrivateKey as BitcoinPrivateKey,
    PublicKey as BitcoinPublicKey, Script,
};
use sha2::{Digest, Sha256};
use walletd_bip39::Seed;
use walletd_coin_model::{CryptoWallet, CryptoAddressGeneral};
use walletd_hd_key::{HDKey, HDNetworkType, SlipCoin};
use crate::FeeEstimates;

use crate::blockstream::{BTransaction, Blockstream, Input, InputType, Output, Utxo};
use crate::BitcoinAmount;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitcoinAddress {
    pub crypto_type: SlipCoin,
    pub address_info: Address,
    pub public_address: String,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
    pub network: Network,
}

#[async_trait]
impl CryptoWallet for BitcoinAddress {
    type AddressFormat = AddressType;
    type BlockchainClient = Blockstream;
    type CryptoAmount = BitcoinAmount;
    type HDKeyInfo = HDKey;
    type MnemonicSeed = Seed;
    type NetworkType = Network;

    fn crypto_type(&self) -> SlipCoin {
        SlipCoin::BTC
    }

    fn from_hd_key(hd_keys: &HDKey, address_type: AddressType) -> Result<Self, anyhow::Error> {
        let public_key_bytes = &hd_keys
            .extended_public_key
            .expect("Public key data missing")
            .to_vec();
        let public_key = BitcoinPublicKey {
            inner: bitcoin::secp256k1::PublicKey::from_slice(public_key_bytes)?,
            compressed: true,
        };

        // TODO(#82): consider handling the other Bitcoin network types
        let network: Network = match hd_keys.network {
            HDNetworkType::MainNet => Network::Bitcoin,
            HDNetworkType::TestNet => Network::Testnet,
        };

        let address_info: Address = match address_type {
            AddressType::P2pkh => Address::p2pkh(&public_key, network),
            // TODO(#83): Not sure about initializing this with an empty script, double check and
            // fix as necessary
            AddressType::P2sh => Address::p2sh(&Script::new(), network)?,
            AddressType::P2wpkh => Address::p2wpkh(&public_key, network)?,
            // TODO(#83): Again check the script::new() here and fix if needed
            AddressType::P2wsh => Address::p2wsh(&Script::new(), network),
            // Currently not handling the AddressType::P2tr, fix if can understand how to create
            // this address properly
            _ => return Err(anyhow!("Currently not handling this Bitcoin address type")),
        };
        let public_address = address_info.to_string();

        Ok(Self {
            crypto_type: SlipCoin::BTC,
            address_info,
            public_address,
            private_key: Some(hd_keys.to_wif()?),
            public_key: Some(hd_keys.public_key()?),
            network,
        })
    }

    fn from_mnemonic(
        mnemonic_seed: &Seed,
        network: Network,
        address_type: AddressType,
    ) -> Result<Self, anyhow::Error> {
        let seed_bytes = mnemonic_seed.as_bytes();
        let mut private_key_bytes = [0u8; 32];
        private_key_bytes.copy_from_slice(&seed_bytes[0..32]);
        let public_key_bytes = PublicKey::from_secret_key(
            &Secp256k1::new(),
            &SecretKey::from_slice(&private_key_bytes)?,
        )
        .serialize();

        let public_key = BitcoinPublicKey {
            inner: bitcoin::secp256k1::PublicKey::from_slice(&public_key_bytes)?,
            compressed: true,
        };

        let address_info: Address = match address_type {
            AddressType::P2pkh => Address::p2pkh(&public_key, network),
            // TODO(#83): Not sure about initializing this with an empty script, double check and
            // fix as necessary
            AddressType::P2sh => Address::p2sh(&Script::new(), network)?,
            AddressType::P2wpkh => Address::p2wpkh(&public_key, network)?,
            // TODO(#83): Again check the script::new() here and fix if needed
            AddressType::P2wsh => Address::p2wsh(&Script::new(), network),
            // Currently not handling the AddressType::P2tr, fix if can understand how to create
            // this address properly
            _ => return Err(anyhow!("Currently not handling this Bitcoin address type")),
        };

        let network_prefix: u8 = match network {
            Network::Bitcoin => 0x80,
            Network::Testnet => 0xef,
            _ => return Err(anyhow!("Currently not handling network {}", network)),
        };

        let public_address = address_info.to_string();

        Ok(Self {
            crypto_type: SlipCoin::BTC,
            address_info,
            public_address,
            private_key: Some(Self::to_private_key_wif(
                &private_key_bytes,
                network_prefix,
            )?),
            public_key: Some(Self::to_public_key_hex(&public_key_bytes)?),
            network,
        })
    }

    fn public_address_string(&self) -> String {
        self.public_address.clone()
    }

    async fn balance(
        &self,
        blockchain_client: &Blockstream,
    ) -> Result<BitcoinAmount, anyhow::Error> {
        let utxo_info = blockchain_client
            .utxo(self.public_address_string().as_str())
            .await?;
        let amount = Self::confirmed_balance_from_utxo(utxo_info)?;

        return Ok(amount);
    }

    async fn transfer(
        &self,
        client: &Blockstream,
        send_amount: &BitcoinAmount,
        public_address: &str,
    ) -> Result<String, anyhow::Error> {
       
        let receiver_view_wallet = Self::new_view_only(public_address)?;

        // first checking existing endpoints with blockstream
        let fee_estimates: FeeEstimates = client.fee_estimates().await?;
        let confirmation_target: u32 = 6; // this variable specifies how many blocks need to include this transaction
                                          // before it's considered "confirmed"
        let fee_sat_per_byte: f64;
        let fee_map = &fee_estimates.0;
        if !fee_map.is_empty() {
            fee_sat_per_byte = fee_map
                .get(confirmation_target.to_string().as_str())
                .expect("fee_map missing key")
                .as_f64()
                .expect("Unable to convert to f64");
            println!(
                "fee_sat_per_vB for confirmation_target {} is {}",
                confirmation_target, fee_sat_per_byte
            );
            
        } else {
            return Err(anyhow!("Did not get fee map"));
        }

        // Build the transaction
        // Specify the inputs and outputs, the difference between the amount of the
        // inputs and the amount of the outputs is the transaction fee
        // Input(s) should cover the total amount
        // Inputs need to come from the utxo
        let available_utxos = client.utxo(self.public_address_string().as_str()).await?;

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
            return Err(anyhow!("Insufficent funds"));
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

    fn address_by_index(
        &self,
        bip32_master: &HDKey,
        index: usize,
    ) -> Result<Box<dyn CryptoAddressGeneral>, anyhow::Error> {
        let derived_key = HDKey::derive(
            bip32_master,
            format!("m/84'/0'/0'/0/{}", index),
        )?;
        Ok(Box::new(BitcoinAddress::from_hd_key(
            &derived_key,
            AddressType::P2wpkh,
        )?))
    }
}

impl BitcoinAddress {
    pub fn new_view_only(public_address: &str) -> Result<Self, anyhow::Error> {
        let address_info = Address::from_str(public_address)?;
        let public_address = address_info.to_string();
        // Currently hardcoding to Mainnet, TODO(#82) handle other Bitcoin network
        // options
        let network = Network::Bitcoin;

        Ok(Self {
            crypto_type: SlipCoin::BTC,
            address_info,
            public_address,
            private_key: None,
            public_key: None,
            network,
        })
    }

    pub fn public_key(&self) -> Result<Vec<u8>, anyhow::Error> {
        if let Some(key) = self.public_key.clone() {
            Ok(hex::decode(key)?)
        } else {
            Err(anyhow!("Public Key not included"))
        }
    }

    pub fn signature_sighashall_for_trasaction_hash(
        transaction_hash: String,
        secret_key: SecretKey,
    ) -> Result<String, anyhow::Error> {
        // hardcoded default to SIGHASH_ALL
        let sighash_type = EcdsaSighashType::All;
        let secp = Secp256k1::new();
        let message = Message::from_slice(&hex::decode(transaction_hash)?).expect("32 bytes");
        let mut sig = secp.sign_ecdsa(&message, &secret_key);
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

    pub fn estimate_fee_with_default_sizes(
        is_segwit: bool,
        num_inputs: usize,
        num_outputs: usize,
        byte_fee: f64,
    ) -> Result<u64, anyhow::Error> {
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
    ) -> Result<(Vec<Input>, BitcoinAmount, Vec<usize>), anyhow::Error> {
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
                Err(anyhow!(
                    "Not enough funds to cover the send amount as well as the fee needed"
                ))
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
    ) -> Result<BTransaction, anyhow::Error> {
        println!("Building a Transaction");
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
            return Err(anyhow!("Insufficient funds to send amount and cover fees"));
        }

        println!(
            "inputs_amount: {} BTC, send_amount: {} BTC, fee_amount {} BTC",
            &inputs_amount.btc(),
            &send_amount.btc(),
            &fee_amount.btc()
        );
        println!(
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
            let private_key = BitcoinPrivateKey::from_wif(
                self.private_key
                    .as_ref()
                    .expect("Private key data missing")
                    .as_str(),
            )?;
            let secret_key = SecretKey::from_slice(private_key.to_bytes().as_slice())
                .expect("32 bytes, within curve order");
            let sig_with_hashtype = Self::signature_sighashall_for_trasaction_hash(
                transaction_hash_for_input_with_sighash.to_string(),
                secret_key,
            )?;

            // handle the different types of inputs based on previous locking script
            let prevout_lockingscript_type = &input.prevout.scriptpubkey_type;
            match prevout_lockingscript_type.as_str() {
                "p2pkh" => {
                    let script_sig = Builder::new()
                        .push_slice(&hex::decode(sig_with_hashtype)?)
                        .push_key(&BitcoinPublicKey::from_slice(&self.public_key()?)?)
                        .into_script();
                    input.scriptsig_asm = script_sig.asm();
                    input.scriptsig = hex::encode(script_sig.as_bytes());
                }
                "p2sh" => {
                    // TODO(#83) need to handle redeem scripts
                    return Err(anyhow!("Not currently handling P2SH"));
                }
                "v0_p2wsh" => {
                    // TODO(#83) need to handle redeem scripts
                    return Err(anyhow!("Not currently handling v0_p2wsh"));
                }
                "v0_p2wpkh" => {
                    // Need to specify witness data to unlock
                    input.witness = vec![sig_with_hashtype, hex::encode(self.public_key()?)];
                }
                _ => {
                    return Err(anyhow!(
                        "Unidentified locking script type from previous output"
                    ))
                }
            }
        }
        transaction.vin = inputs;

        Ok(transaction)
    }

    pub fn confirmed_balance_from_utxo(
        utxo_info: Vec<Utxo>,
    ) -> Result<BitcoinAmount, anyhow::Error> {
        let mut satoshis: u64 = 0;
        for item in utxo_info {
            satoshis += item.value;
        }
        let confirmed_balance = BitcoinAmount { satoshi: satoshis };
        Ok(confirmed_balance)
    }

    pub fn address_info(&self) -> Address {
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

impl Display for BitcoinAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Bitcoin Wallet")?;
        writeln!(f, " Network: {}", self.network)?;
        writeln!(
            f,
            " Private Key: {}",
            self.private_key.clone().unwrap_or_default()
        )?;
        writeln!(
            f,
            " Public Key: {}",
            self.public_key.clone().unwrap_or_default()
        )?;
        writeln!(
            f,
            " Address Type: {}",
            self.address_info
                .address_type()
                .expect("Expecting address type datas")
        )?;
        writeln!(f, " Public Address: {}", self.public_address)?;
        Ok(())
    }
}

impl CryptoAddressGeneral for BitcoinAddress {
    fn crypto_type(&self) -> SlipCoin {
        self.crypto_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> Box<dyn CryptoAddressGeneral> {
        Box::new(self.clone())
    }
}
