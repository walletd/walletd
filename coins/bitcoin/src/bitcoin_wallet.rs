use crate::blockstream::{BTransaction, Blockstream, FeeEstimates, Input, InputType, Output, Utxo};
use crate::BitcoinAddress;
use crate::BitcoinAmount;
use crate::Error;
use async_trait::async_trait;
use bitcoin::blockdata::script;
use std::cmp::Reverse;
use walletd_bip39::Seed;
use walletd_coin_core::CryptoAddress;
use walletd_coin_core::CryptoWalletBuilder;
use walletd_coin_core::{CryptoAmount, CryptoWallet};
use walletd_hd_key::slip44;
use walletd_hd_key::{HDKey, HDNetworkType, HDPath, HDPathBuilder, HDPathIndex, HDPurpose};

use bitcoin::script::PushBytes;

use ::secp256k1::{Message, Secp256k1, SecretKey};

pub use bitcoin::{
    sighash::EcdsaSighashType, Address, AddressType, Network, PrivateKey as BitcoinPrivateKey,
    PublicKey as BitcoinPublicKey, Script,
};

const DEFAULT_GAP_LIMIT: usize = 20;

/// Represents a Bitcoin wallet which can have mulitple BitcoinAddress structs associated with it which are derived from a single master HD key.
#[derive(Debug, Clone)]
pub struct BitcoinWallet {
    address_format: AddressType,
    associated: Vec<AssociatedAddress>,
    blockchain_client: Option<Blockstream>,
    master_hd_key: Option<HDKey>,
    gap_limit: usize,
    account_discovery: bool,
    hd_path_builder: Option<HDPathBuilder>,
}

impl Default for BitcoinWallet {
    fn default() -> Self {
        Self {
            associated: Vec::new(),
            blockchain_client: None,
            address_format: AddressType::P2wpkh,
            master_hd_key: None,
            gap_limit: DEFAULT_GAP_LIMIT,
            account_discovery: true,
            hd_path_builder: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssociatedAddress {
    pub address: BitcoinAddress,
    pub hd_key: HDKey,
}

impl AssociatedAddress {
    pub fn new(address: BitcoinAddress, hd_key: HDKey) -> Self {
        Self { address, hd_key }
    }

    pub fn address(&self) -> &BitcoinAddress {
        &self.address
    }

    pub fn hd_key(&self) -> &HDKey {
        &self.hd_key
    }
}

#[async_trait]
impl CryptoWallet for BitcoinWallet {
    type ErrorType = Error;
    type BlockchainClient = Blockstream;
    type CryptoAmount = BitcoinAmount;
    type NetworkType = Network;
    type WalletBuilder = BitcoinWalletBuilder;
    type AddressFormat = AddressType;

    async fn balance(&self) -> Result<BitcoinAmount, Error> {
        let client = self.blockchain_client()?;
        let mut total_balance = BitcoinAmount::new();
        for addr in self.addresses() {
            let balance = addr.balance(client).await?;
            total_balance = (total_balance + balance)?;
        }
        Ok(total_balance)
    }

    fn builder() -> Self::WalletBuilder {
        BitcoinWalletBuilder::new()
    }

    async fn transfer(
        &self,
        send_amount: &BitcoinAmount,
        to_public_address: &str,
    ) -> Result<String, Error> {
        let client = self.blockchain_client()?;
        let receiver_view_wallet =
            BitcoinAddress::from_public_address(to_public_address, self.network()?)?;

        // first checking existing endpoints with blockstream
        let fee_estimates: FeeEstimates = client.fee_estimates().await?;
        let confirmation_target: u32 = 6; // this variable specifies how many blocks need to include this transaction
                                          // before it's considered "confirmed"

        let fee_map = &fee_estimates.0;
        let fee_sat_per_byte = if !fee_map.is_empty() {
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
        // Look through all the associated owned addresses for available utxos
        let mut available_utxos = Vec::new();
        for addr in self.addresses() {
            let utxos = client.utxo(&addr.public_address()).await?;
            available_utxos.push(utxos);
        }

        // sum total value with confirmed status, also count number of utxos with
        // confirmed status
        let mut total_value_from_utxos = 0;
        let mut inputs_available: Vec<Utxo> = Vec::new();
        let mut inputs_available_tx_info: Vec<BTransaction> = Vec::new();
        let change_addr = self.next_change_address()?.address_info();

        let mut keys_per_input: Vec<(BitcoinPrivateKey, BitcoinPublicKey)> = Vec::new();
        let mut utxo_addr_index = Vec::new();
        for (i, utxos_i) in available_utxos.iter().enumerate() {
            for utxo in utxos_i.iter() {
                if utxo.status.confirmed {
                    total_value_from_utxos += &utxo.value;
                    let tx_info = client.transaction(utxo.txid.as_str()).await?;
                    inputs_available.push(utxo.clone());
                    inputs_available_tx_info.push(tx_info);
                    utxo_addr_index.push(i);
                }
            }
        }

        let available_input_max = BitcoinAmount {
            satoshi: total_value_from_utxos,
        };

        if available_input_max < *send_amount {
            return Err(Error::InsufficientFunds("Insufficent funds".into()));
        }

        let prepared = Self::prepare_transaction(
            fee_sat_per_byte,
            &inputs_available,
            &inputs_available_tx_info,
            send_amount,
            &receiver_view_wallet,
            change_addr,
        )?;

        let transaction = prepared.0;
        let chosen_indices = prepared.1;

        for ind in chosen_indices {
            let index = utxo_addr_index[ind];
            let private_key = self.associated[index].address().private_key()?;
            let public_key = self.associated[index].address().public_key()?;
            let key_pair = (private_key, public_key);
            keys_per_input.push(key_pair);
        }

        let signed_tx = Self::sign_tx(&transaction, keys_per_input)?;

        let transaction_hex = BTransaction::serialize(&signed_tx)?;
        let raw_transaction_hex: &'static str = Box::leak(transaction_hex.into_boxed_str());
        let tx_id = client.post_a_transaction(raw_transaction_hex).await?;
        Ok(tx_id)
    }

    fn set_blockchain_client(&mut self, client: Self::BlockchainClient) {
        self.blockchain_client = Some(client);
    }

    async fn sync(&mut self) -> Result<(), Error> {
        self.add_previously_used_addresses().await?;
        Ok(())
    }

    fn receive_address(&self) -> Result<String, Error> {
        let next_receive_address = self.next_address()?;
        Ok(next_receive_address.public_address())
    }

    fn blockchain_client(&self) -> Result<&Blockstream, Error> {
        match &self.blockchain_client {
            Some(client) => Ok(client),
            None => Err(Error::MissingBlockchainClient),
        }
    }
}

impl BitcoinWallet {
    /// Adds an address to the wallet if it is not already present
    pub fn add(&mut self, associated: &AssociatedAddress) {
        if self.addresses().contains(&associated.address) {
            return;
        }
        self.associated.push(associated.clone());
    }

    /// Returns the associated info
    pub fn associated_info(&self) -> &[AssociatedAddress] {
        &self.associated
    }

    /// Returns a vector of the BitcoinAddress objects associated with the wallet
    pub fn addresses(&self) -> Vec<BitcoinAddress> {
        self.associated.iter().map(|x| x.address.clone()).collect()
    }

    /// Returns the coin type id num based on the network
    /// # Errors
    /// Returns an error if the network is not supported
    pub fn coin_type_id(&self) -> Result<u32, Error> {
        match self.network()? {
            Network::Bitcoin => Ok(slip44::Coin::Bitcoin.id()),
            Network::Testnet | Network::Regtest => Ok(slip44::Coin::Testnet.id()),
            other => Err(Error::CurrentlyNotSupported(format!(
                "Network {} currently not supported",
                other
            ))),
        }
    }

    /// Returns the default HDPurpose based on the address format
    ///
    /// # Errors
    /// Returns an error if the address format is not currently supported
    pub fn default_hd_purpose(&self) -> Result<HDPurpose, Error> {
        match self.address_format() {
            AddressType::P2pkh => Ok(HDPurpose::BIP44),
            AddressType::P2sh => Ok(HDPurpose::BIP49),
            AddressType::P2wpkh => Ok(HDPurpose::BIP84),
            other => Err(Error::CurrentlyNotSupported(format!(
                "Address format {} currently not supported",
                other
            ))),
        }
    }

    /// Discovers previously used addresses by searching in sequential order based on master HDKey and a derivation type,
    /// stopping discovery when gap limit (n consecutive addresses without transaction history) has been met.
    /// Only considers change index = 0 (the receiving/external chain) when
    /// considering the gap limit but if there is transaction history with
    /// change index = 1 it is added as an associated address.
    /// If the account discovery setting is false, it will only search for addresses in the first account (account_index = 0).
    pub async fn add_previously_used_addresses(&mut self) -> Result<(), Error> {
        let master_hd_key = self.master_hd_key()?;
        let address_format = self.address_format();
        let blockchain_client = self.blockchain_client()?.clone();
        let gap_limit = self.gap_limit;
        let mut path_builder = match self.hd_path_builder.clone() {
            Some(deriv_type) => deriv_type,
            None => {
                let mut builder = HDPath::builder();
                builder
                    .purpose(self.default_hd_purpose()?.to_shortform_num())
                    .coin_type_index(self.coin_type_id()?)
                    .account_index(0)
                    .address_index(0);
                builder
            }
        };

        let mut current_gap = 0;
        let mut search_next_account = true;
        let mut account_index = 0;
        let mut address_index = 0;

        while search_next_account {
            search_next_account = false;
            while current_gap < gap_limit {
                for change_index in 0..2 {
                    let specify_deriv_path = &path_builder
                        .clone()
                        .change_index(change_index)
                        .build()
                        .to_string();
                    let derived = master_hd_key.derive(specify_deriv_path.clone())?;
                    let address = BitcoinAddress::from_hd_key(&derived, address_format)?;
                    let exists = blockchain_client
                        .check_if_past_transactions_exist(&address.public_address())
                        .await?;

                    log::info!(
                        "For deriv path: {}, address: {}, previous transaction history: {}",
                        &specify_deriv_path,
                        address.public_address(),
                        exists
                    );

                    if exists {
                        search_next_account = true;
                        let associated = AssociatedAddress::new(address, derived);
                        self.add(&associated);
                    } else if change_index == 0 {
                        current_gap += 1;
                    }
                }
                address_index += 1;
                path_builder.address_index(address_index);
            }
            if !self.account_discovery {
                break;
            }
            account_index += 1;
            path_builder.account_index(account_index);
            address_index = 0;
            current_gap = 0;
        }
        Ok(())
    }

    /// Returns the address format
    pub fn address_format(&self) -> AddressType {
        self.address_format
    }

    /// Returns the master HDKey, if it exists otherwise returns an error
    pub fn master_hd_key(&self) -> Result<HDKey, Error> {
        match &self.master_hd_key {
            Some(key) => Ok(key.clone()),
            None => Err(Error::MissingMasterHDKey),
        }
    }

    /// Returns the network based on the master HDKey
    pub fn network(&self) -> Result<Network, Error> {
        match self.master_hd_key()?.network() {
            HDNetworkType::MainNet => Ok(Network::Bitcoin),
            HDNetworkType::TestNet => Ok(Network::Testnet),
        }
    }

    /// Returns a BitcoinAddress object on the the next available address on the first account (account_index = 0)
    /// # Errors
    /// Returns an `Error` if it encounters a problem while deriving the next address
    pub fn next_address(&self) -> Result<BitcoinAddress, Error> {
        let purpose = self.default_hd_purpose()?.to_shortform_num();
        let coin_type = self.coin_type_id()?;
        let account = HDPathIndex::IndexHardened(0);
        let mut max_address = 0;
        let mut path_builder = HDPath::builder();
        path_builder
            .purpose(purpose)
            .coin_type_index(coin_type)
            .account_index(account.to_shortform_num())
            .hardened_account();

        for info in self.associated.iter() {
            let deriv_path = &info.hd_key().derivation_path();
            let account = deriv_path.account()?.to_shortform_num();
            let address_index = deriv_path.address()?.to_shortform_num();
            if account == 0 && address_index > max_address {
                max_address = address_index;
            }
        }
        let next_deriv_path = path_builder
            .address_index(max_address + 1)
            .build()
            .to_string();
        let next_hd_key = self.master_hd_key()?.derive(next_deriv_path)?;
        BitcoinAddress::from_hd_key(&next_hd_key, self.address_format)
    }

    /// Considering only account 0, returns the next change address corresponding to 1 + the max existing change address index
    /// Change addresses are used for sending change back to the wallet and have a value of 1 instead of 0 in the derivation path for the change index
    pub fn next_change_address(&self) -> Result<BitcoinAddress, Error> {
        let purpose = match &self.hd_path_builder {
            Some(builder) => match builder.purpose {
                Some(purpose) => purpose,
                None => self.default_hd_purpose()?.to_shortform_num(),
            },
            None => self.default_hd_purpose()?.to_shortform_num(),
        };

        let coin_type = self.coin_type_id()?;
        let account = HDPathIndex::IndexHardened(0);
        let mut max_address = 0;
        let mut path_builder = match self.hd_path_builder.clone() {
            Some(builder) => builder,
            None => {
                let mut builder = HDPath::builder();

                builder
                    .purpose(purpose)
                    .coin_type_index(coin_type)
                    .account_index(account.to_shortform_num())
                    .hardened_account();
                builder
            }
        };
        path_builder.change_index(1);

        for info in self.associated.iter() {
            let deriv_path = &info.hd_key().derivation_path();
            let change_index_derived = deriv_path.change()?.to_shortform_num();
            let address_index_derived = deriv_path.address()?.to_shortform_num();
            if (change_index_derived == 1) & (address_index_derived > max_address) {
                max_address = address_index_derived;
            }
        }

        let next_deriv_path = path_builder
            .address_index(max_address + 1)
            .build()
            .to_string();
        let next_hd_key = self.master_hd_key()?.derive(next_deriv_path)?;
        BitcoinAddress::from_hd_key(&next_hd_key, self.address_format)
    }

    /// Set the gap limit to use when searching for addresses, if not set, the default gap limit is used
    pub fn set_gap_limit(&mut self, gap_limit: usize) {
        self.gap_limit = gap_limit;
    }

    /// Set the account discovery flag, if set to true, the wallet will search for addresses on all accounts, if set to false, the wallet will only search for addresses on the first account
    /// If not set, the default value is true
    pub fn set_account_discovery(&mut self, account_discovery: bool) {
        self.account_discovery = account_discovery;
    }

    /// Set the HDPathBuilder to use when deriving addresses, if not set, the default HDPathBuilder is used
    pub fn set_hd_path_builder(&mut self, hd_path_builder: HDPathBuilder) {
        self.hd_path_builder = Some(hd_path_builder);
    }

    /// Returns the gap limit that is being used when searching for addresses with this wallet
    pub fn gap_limit(&self) -> usize {
        self.gap_limit
    }

    /// Returns the account discovery flag that is being used when searching for addresses with this wallet
    pub fn account_discovery(&self) -> bool {
        self.account_discovery
    }

    /// Returns the HDPathBuilder that is being used when deriving addresses with this wallet
    /// If no HDPathBuilder has been set, the default HDPathBuilder that is being used is returned
    pub fn hd_path_builder(&self) -> HDPathBuilder {
        match &self.hd_path_builder {
            Some(builder) => builder.clone(),
            None => {
                let mut builder = HDPath::builder();
                builder
                    .purpose(self.default_hd_purpose().unwrap().to_shortform_num())
                    .coin_type_index(self.coin_type_id().unwrap());
                builder
            }
        }
    }
    /// This function is used to calculate the signature as a hex encoded string with the option sighashall for a given transaction hash using a provided private key
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

    /// Signs a transaction with the provided private keys and returns the signed transaction
    pub fn sign_tx(
        tx: &BTransaction,
        keys_per_input: Vec<(BitcoinPrivateKey, BitcoinPublicKey)>,
    ) -> Result<BTransaction, Error> {
        let mut inputs = tx.vin.clone();
        // Signing and unlocking the inputs
        for (i, input) in inputs.iter_mut().enumerate() {
            // hardcoded default to SIGHASH_ALL
            let sighash_type = EcdsaSighashType::All;
            let transaction_hash_for_input_with_sighash =
                tx.transaction_hash_for_signing_segwit_input_index(i, sighash_type.to_u32())?;
            let private_key = &keys_per_input[i].0;
            let public_key = &keys_per_input[i].1;
            let sig_with_hashtype = BitcoinWallet::signature_sighashall_for_transaction_hash(
                &transaction_hash_for_input_with_sighash,
                private_key,
            )?;

            let sig_with_hashtype_vec = hex::decode(&sig_with_hashtype)?;
            let sig_with_hashtype_bytes: &PushBytes =
                sig_with_hashtype_vec.as_slice().try_into()?;

            // handle the different types of inputs based on previous locking script
            let prevout_lockingscript_type = &input.prevout.scriptpubkey_type;
            match prevout_lockingscript_type.as_str() {
                "p2pkh" => {
                    let script_sig = script::Builder::new()
                        .push_slice(sig_with_hashtype_bytes)
                        .push_key(public_key)
                        .into_script();
                    input.scriptsig_asm = script_sig.to_asm_string();
                    input.scriptsig = hex::encode(script_sig.as_bytes());
                }
                "p2sh" => {
                    // TODO(#83) need to handle redeem scripts
                    return Err(Error::CurrentlyNotSupported(
                        "Not currently handling P2SH".into(),
                    ));
                }
                "v0_p2wsh" => {
                    // TODO(#83) need to handle redeem scripts
                    return Err(Error::CurrentlyNotSupported(
                        "Not currently handling v0_p2wsh".into(),
                    ));
                }
                "v0_p2wpkh" => {
                    // Need to specify witness data to unlock
                    input.witness = vec![sig_with_hashtype, hex::encode(public_key.to_bytes())];
                }
                _ => {
                    return Err(Error::CurrentlyNotSupported(
                        "Unidentified locking script type from previous output".into(),
                    ))
                }
            }
        }
        let mut signed_tx = tx.clone();
        signed_tx.vin = inputs;
        Ok(signed_tx)
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
        let min_goal_target = (*send_amount * 1.5)?;
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
            obtained_amount = (obtained_amount + value)?;
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
            let change_and_fee_amount = (obtained_amount - *send_amount)?;
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
            let change_amount = (change_and_fee_amount - set_fee)?;
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
                    let wanted_extra = (min_change_amount - change_amount)?;
                    let min_goal_target = (obtained_amount + wanted_extra)?;
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
                        obtained_amount = (obtained_amount + value)?;
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
            if obtained_amount > (*send_amount + set_fee)? {
                Ok((inputs, set_fee, chosen_indices))
            } else {
                Err(Error::InsufficientFunds(
                    "Not enough funds to cover the send amount as well as the fee needed".into(),
                ))
            }
        }
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

    /// Prepares a transaction to be signed and broadcasted
    /// # Arguments
    /// * `fee_sat_per_byte` - the fee to be paid per byte of the transaction
    /// * `utxo_available` - the utxos available to be used in the transaction
    /// * `inputs_available_tx_info` - the transaction info for the utxos available
    /// * `send_amount` - the amount to be sent
    /// * `receiver_view_wallet` - the address to send the funds to
    /// * `change_addr` - the address to send the change to
    /// # Returns
    /// * `Result<(BTransaction, Vec<usize>), Error>` - the transaction and the indices of the utxos to use
    /// # Errors
    /// * Returns an Error if the transaction cannot be prepared
    pub fn prepare_transaction(
        fee_sat_per_byte: f64,
        utxo_available: &Vec<Utxo>,
        inputs_available_tx_info: &[BTransaction],
        send_amount: &BitcoinAmount,
        receiver_view_wallet: &BitcoinAddress,
        change_addr: Address,
    ) -> Result<(BTransaction, Vec<usize>), Error> {
        // TODO(AS): Add check here to limit the transaction to address types that are supported
        // choose inputs
        let (inputs, fee_amount, chosen_indices) = Self::choose_inputs_and_set_fee(
            utxo_available,
            send_amount,
            inputs_available_tx_info,
            fee_sat_per_byte,
        )?;
        let inputs_amount = BitcoinAmount {
            satoshi: inputs.iter().map(|x| x.prevout.value).sum(),
        };
        if inputs_amount < (*send_amount + fee_amount)? {
            return Err(Error::InsufficientFunds(
                "Insufficient funds to send amount and cover fees".into(),
            ));
        }

        let change_amount = ((inputs_amount - *send_amount)? - fee_amount)?;

        // Create two outputs, one for the send amount and another for the change amount
        // Hardcoding p2wpkh SegWit transaction option
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
}

/// Builder for [BitcoinWallet] that allows for the creation of a [BitcoinWallet] with a custom configuration
pub struct BitcoinWalletBuilder {
    /// The address format used to generate the wallet, if the address format is not provided, the default address format is P2wpkh
    address_format: AddressType,
    /// The HD purpose used to generate the wallet, if the HD purpose is not provided, the default HD purpose will be inferred from the address_format
    hd_purpose: Option<HDPurpose>,
    /// The master HD key used to import the wallet
    master_hd_key: Option<HDKey>,
    /// The gap limit used to determine when to stop searching for addresses with a previous transaction history, if the gap limit is not provided, the default gap limit is 20 which means the search will stop after 20 consecutive addresses with no previous transaction history
    gap_limit_specified: Option<usize>,
    /// The account discovery flag used to determine whether to search for consecutive accounts with a previous transaction history
    /// If the account discovery flag is set to false, then only the first account will be searched and the search will stop after the gap limit is reached
    /// If the account discovery is set to true, then the search will continue until the gap limit is reached for each account until a account is found with no previous transaction history when searched up to the gap limit
    /// The default value for the account discovery flag is true
    account_discovery: bool,
    /// The mnemonic seed used to import the wallet, if the mnemonic seed is not provided, the master_hd_key must be provided
    /// If the master_hd_key is provided, the mnemonic seed will be ignored
    mnemonic_seed: Option<Seed>,
    /// The specified network type to use, if the master_hd_key is provided, the network type will be inferred from the master_hd_key and this network_type will be ignored
    /// The default network type is Network::Bitcoin
    network_type: Network,
    /// Specifiyng a HDPathBuilder allows for customizing the derivation path used including which indices are hardened and will override the default
    /// The default HDPathBuilder uses hardened indices for the purpose, coin type, account ,and non-hardened indices for the change and address indices
    hd_path_builder: HDPathBuilder,
}

impl Default for BitcoinWalletBuilder {
    fn default() -> Self {
        let default_hd_purpose = HDPurpose::BIP84;

        let mut deriv_path_builder = HDPath::builder();
        deriv_path_builder
            .purpose(default_hd_purpose.to_shortform_num())
            .hardened_purpose()
            .coin_type_index(slip44::Coin::Bitcoin.id())
            .hardened_coin_type()
            .hardened_account()
            .non_hardened_change()
            .non_hardened_address();

        Self {
            address_format: AddressType::P2wpkh,
            hd_purpose: Some(HDPurpose::BIP84),
            master_hd_key: None,
            gap_limit_specified: Some(20),
            account_discovery: true,
            mnemonic_seed: None,
            network_type: Network::Bitcoin,
            hd_path_builder: deriv_path_builder,
        }
    }
}

impl CryptoWalletBuilder<BitcoinWallet> for BitcoinWalletBuilder {
    /// Generates a new BitcoinWalletBuilder with the default options
    fn new() -> Self {
        Self::default()
    }

    /// Allows specification of the master HD key for the wallet
    fn master_hd_key(&mut self, master_hd_key: HDKey) -> &mut Self {
        self.master_hd_key = Some(master_hd_key);
        self
    }

    /// Allows specification of the mnemonic seed for the wallet
    fn mnemonic_seed(&mut self, mnemonic_seed: Seed) -> &mut Self {
        self.mnemonic_seed = Some(mnemonic_seed);
        self
    }

    /// Allows specification of the address format to use for the wallet
    fn address_format(
        &mut self,
        address_format: <BitcoinWallet as CryptoWallet>::AddressFormat,
    ) -> &mut Self {
        self.address_format = address_format;
        self
    }

    /// Allows specification of the network type for the wallet, the default is Network::Bitcoin
    fn network_type(&mut self, network_type: Network) -> &mut Self {
        self.network_type = network_type;
        self
    }

    /// Allows specifiction of the hd path builder, will override the default
    fn hd_path_builder(&mut self, hd_path_builder: HDPathBuilder) -> &mut Self {
        self.hd_path_builder = hd_path_builder;
        self
    }

    /// Used to import an existing wallet from a master HD key or a mnemonic seed and specified network type
    /// # Errors
    /// Returns the error `Error::UnableToImportWallet` if the master HD key is not provided
    fn build(&self) -> Result<BitcoinWallet, Error> {
        let master_hd_key = match (&self.master_hd_key, &self.mnemonic_seed) {
            (None, None) => {
                return Err(Error::UnableToImportWallet(
                    "Neither the master HD key nor the mnemonic seed was provided".to_string(),
                ))
            }
            (Some(key), _) => key.clone(),
            (None, Some(seed)) => {
                let hd_network_type = match self.network_type {
                    Network::Bitcoin => HDNetworkType::MainNet,
                    _ => HDNetworkType::TestNet,
                };

                HDKey::new_master(seed.clone(), hd_network_type)?
            }
        };

        let hd_purpose = match self.hd_purpose {
            None => self.default_hd_purpose()?,
            Some(purpose) => purpose,
        };

        let coin_type_id = self.coin_type_id()?;

        let mut hd_path_builder = HDPath::builder();
        hd_path_builder
            .purpose(hd_purpose.to_shortform_num())
            .hardened_purpose()
            .coin_type_index(coin_type_id)
            .hardened_coin_type();

        let wallet = BitcoinWallet {
            address_format: self.address_format,
            associated: Vec::new(),
            blockchain_client: None,
            master_hd_key: Some(master_hd_key),
            account_discovery: self.account_discovery,
            gap_limit: self.gap_limit_specified.unwrap_or(DEFAULT_GAP_LIMIT),
            hd_path_builder: Some(hd_path_builder),
        };

        Ok(wallet)
    }
}

impl BitcoinWalletBuilder {
    /// Allows specification of the gap limit to use for the wallet
    pub fn gap_limit(&mut self, gap_limit: usize) -> &mut Self {
        self.gap_limit_specified = Some(gap_limit);
        self
    }

    /// Allows specification of the account discovery to use for the wallet
    /// If set to false, the wallet will not search for accounts used past the first account
    /// The default is true

    /// Enable account discovery, will search consecutive accounts until the gap limit is reached and an account is found with no transactions
    pub fn account_discovery(&mut self) -> &mut Self {
        self.account_discovery = true;
        self
    }

    /// Disable account discovery should not be used, will not search past the first account
    pub fn no_account_discovery(&mut self) -> &mut Self {
        self.account_discovery = false;
        self
    }

    /// Returns the default HDPurpose based on the address format
    ///
    /// # Errors
    /// Returns an error if the address format is not currently supported
    pub fn default_hd_purpose(&self) -> Result<HDPurpose, Error> {
        match self.address_format {
            AddressType::P2pkh => Ok(HDPurpose::BIP44),
            AddressType::P2sh => Ok(HDPurpose::BIP49),
            AddressType::P2wpkh => Ok(HDPurpose::BIP84),
            other => Err(Error::CurrentlyNotSupported(format!(
                "Address format {} currently not supported",
                other
            ))),
        }
    }

    /// Returns the coin type id num based on the network
    /// # Errors
    /// Returns an error if the network is not supported
    pub fn coin_type_id(&self) -> Result<u32, Error> {
        match &self.master_hd_key {
            Some(key) => {
                match key.network() {
                    HDNetworkType::MainNet => return Ok(slip44::Coin::Bitcoin.id()),
                    HDNetworkType::TestNet => return Ok(slip44::Coin::Testnet.id()),
                };
            }
            None => Err(Error::MissingMasterHDKey),
        }
    }
}

#[cfg(test)]
mod test_bitcoin_wallet_builder;
