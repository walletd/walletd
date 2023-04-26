use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::anyhow;
use async_trait::async_trait;
use curve25519_dalek::scalar::Scalar;
use hmac::{Hmac, Mac};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use sha2::Sha512;
use thiserror::Error;
use walletd_hd_key::SlipCoin;

use crate::monero_serialize::DoSerialize;
use crate::transaction::{TransactionPrefix, TxInToKey, TxSourceEntry};
use crate::{
    fee_utils, hash, key_image, monero_lws, payment_id, public_key, rct_types, transaction,
    KeyImage, PaymentId, PrivateKey, VarInt,
};
type HmacSha512 = Hmac<Sha512>;
use std::any::Any;

use crate::monero_lws::{UnspentOutput, DEFAULT_DUST_THRESHOLD, FAKE_OUTPUTS_COUNT};
use crate::private_key::KEY_LEN;
use crate::rct_types::RctKey;
use crate::transaction::{
    GetOutsEntry, PendingTransaction, Priority, SendTransaction, TxDestinationEntry,
};
use crate::{
    Address, AddressType, CryptoWallet, HDKey, MoneroAmount,
    MoneroLWSConnection, MoneroPrivateKeys, MoneroPublicKeys, Network, HDNetworkType, PublicKey,
    Seed, SerializedArchive,
};

const TX_EXTRA_TAG_PUBKEY: u8 = 0x01;
const HF_VERSION_VIEW_TAGS: u8 = 15;
const EXPECTED_MINIMUM_HF_VERSION: u8 = 15;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MoneroWallet {
    crypto_type: SlipCoin,
    address_format: AddressType,
    network: Network,
    public_address: Address,
    private_keys: MoneroPrivateKeys,
    public_keys: MoneroPublicKeys,
}

#[derive(Debug, Error)]
pub enum Error {
    /// Error of fake output having the same global index as the real output
    /// Can be handled by retrying getting the fake outputs from the server
    #[error("Fake output has the same global index as the real output")]
    FakeOutputHasSameGlobalIndex,
    /// Incorrect number of outputs for amount
    #[error("Not correct number of outputs for amount: expected {expected}, found {found}")]
    IncorrectNumberOfOutputs { expected: usize, found: usize },
    #[error("Monero::LWS Error: {0}")]
    MoneroLWS(#[from] monero_lws::Error),
    // Error from public_key module
    #[error("public_key Error: {0}")]
    PublicKey(#[from] public_key::Error),
    /// Error because sending zero amount
    #[error("Error because sending zero amount")]
    SendingZeroAmount,
    /// Unable to meet the hard fork version expectation
    #[error("Unable to meet the hard fork version expectated minimum fork version: {expected:?}, found {found:?}")]
    InvalidHardForkVersionAssumption { found: u8, expected: u8 },
    /// Only one payment id allowed per transaction
    #[error("Only one payment id allowed per transaction")]
    OnlyOnePaymentIdAllowed,
    /// Error from handling payment id
    #[error("Error from handling payment id")]
    PaymentId(#[from] payment_id::Error),
    /// Error stemming from insufficient funds for transfer
    #[error("Insufficient funds, unable to complete transfer, needed {needed:?}, found {found:?}")]
    InsufficientFunds { needed: u64, found: u64 },
    /// Error stemming from insufficient funds to cover fees and send amount
    #[error("Insufficient funds to cover fees and send amount, unable to complete transfer, needed {needed:?}, found {found:?}")]
    InsufficientFundsForFee { needed: u64, found: u64 },
    /// Real output index is out of bounds
    #[error("Real output index is out of bounds")]
    RealOutputIndexOutOfBounds { index: usize, size: usize },
    /// Did not find real output index
    #[error("Did not find real output index")]
    DidNotFindRealOutputIndex,
    /// Error with the check derived key not equaling the real key
    #[error("Derived not equal real: index {index:?}, real_out {real_out:?}, derived_key {derived_key:?}, real_key {real_key:?}")]
    DerivedNotEqualReal {
        index: usize,
        real_out: u64,
        derived_key: String,
        real_key: String,
    },
    /// Not all TxOutTargetVariant types are currently supported
    #[error("Not all TxOutTargetVariant types are currently supported")]
    UnsupportedTxOutTargetVariant,
    /// Error converted from an anyhow::Error
    #[error("Error converted from an anyhow::Error: {0}")]
    FromAnyhow(#[from] anyhow::Error),
    /// Error converted from the key_image module
    #[error("Error converted from the key_image module: {0}")]
    KeyImage(#[from] key_image::Error),
    /// Error from vectors having different lengths
    #[error("Expected vectors to the same length: vector 1 length {0:?} != vector 2 length {1:?}")]
    DifferentLengths(usize, usize),
    #[error("Transaction error, outputs value greater than inputs value: inputs {inputs:?}, outputs {outputs:?}")]
    TransactionValue { inputs: u64, outputs: u64 },
}

#[async_trait]
impl CryptoWallet for MoneroWallet {
    type AddressFormat = AddressType;
    type BlockchainClient = MoneroLWSConnection;
    type CryptoAmount = MoneroAmount;
    type HDKeyInfo = HDKey;
    type MnemonicSeed = Seed;
    type NetworkType = Network;

    /// Returns the CryptoCoin type, for Monero, returns SlipCoin::XMR
    fn crypto_type(&self) -> SlipCoin {
        SlipCoin::XMR
    }

    /// Constructs a MoneroWallet given a hd key and address format
    fn from_hd_key(
        hd_keys: &HDKey,
        address_format: Self::AddressFormat,
    ) -> Result<Self, anyhow::Error> {
        // uses BIP85 specification, https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki
        let mut entropy = HmacSha512::new_from_slice(b"bip-entropy-from-k")?;
        entropy.update(
            &hd_keys
                .extended_private_key
                .expect("extended private key data missing"),
        );

        // Monero uses 256 bits for the seed, 32 bytes (KEY_LEN)
        let entropy_bytes = &entropy.finalize().into_bytes()[..KEY_LEN];

        let mut seed = [0u8; KEY_LEN];
        seed.copy_from_slice(entropy_bytes);
        let private_keys = MoneroPrivateKeys::from_seed(&seed)?;
        let public_keys = MoneroPublicKeys::from_private_keys(&private_keys);

        let network = match hd_keys.network {
            HDNetworkType::MainNet => Network::Mainnet,
            HDNetworkType::TestNet => Network::Stagenet,
        };

        let public_address = Address::new(&network, &public_keys, &address_format)?;

        Ok(Self {
            crypto_type: SlipCoin::XMR,
            address_format,
            private_keys,
            public_keys,
            public_address,
            network,
        })
    }

    fn from_mnemonic(
        mnemonic_seed: &Seed,
        network: Network,
        address_format: AddressType,
    ) -> Result<Self, anyhow::Error> {
        let seed = mnemonic_seed.as_bytes();
        let private_keys = MoneroPrivateKeys::from_seed(seed)?;
        let public_keys = MoneroPublicKeys::from_private_keys(&private_keys);
        let public_address = Address::new(&network, &public_keys, &address_format)?;

        Ok(Self {
            crypto_type: SlipCoin::XMR,
            address_format,
            private_keys,
            public_keys,
            public_address,
            network,
        })
    }

    fn public_address_string(&self) -> String {
        self.public_address.to_string()
    }

    /// TODO(#68) add more nuanced balance reports for Monero (pending,unlocked,
    /// locked)
    async fn balance(
        &self,
        blockchain_client: &Self::BlockchainClient,
    ) -> Result<Self::CryptoAmount, anyhow::Error> {
        blockchain_client
            .login(
                &self.public_address_string(),
                &self.private_keys.view_key().to_string(),
                Some(true),
                None,
            )
            .await?;
        let unspent_outs_response = blockchain_client
            .get_unspent_outs(
                &self.public_address_string(),
                &self.private_keys.view_key().to_string(),
                0,
                true,
                0,
            )
            .await?;
        let unspent_outs = MoneroLWSConnection::to_unspent_outputs(self, &unspent_outs_response)?;
        println!("unspent outs: {:?}", unspent_outs);
        let mut balance = MoneroAmount::from_piconero(0);
        for unspent_out in unspent_outs {
            balance += MoneroAmount::from_piconero(unspent_out.amount);
        }
        return Ok(balance);
    }

    /// TODO(#81) add transfer functionality for Monero
    /// to send to multiple destinations
    /// TODO(#68): fix this transfer function so that it is able to send a valid
    /// transaction, currently transaction created is not valid and does not
    /// pass the validate check
    async fn transfer(
        &self,
        blockchain_client: &Self::BlockchainClient,
        send_amount: &Self::CryptoAmount,
        public_address: &str,
    ) -> Result<String, anyhow::Error> {
        let receiver_address = Address::from_str(public_address)?;
        let send_amount_dest = TxDestinationEntry {
            amount: send_amount.as_piconero(),
            addr: receiver_address,
        };

        let unspent_outs_response = blockchain_client
            .get_unspent_outs(
                &self.public_address_string(),
                &self.private_keys.view_key().to_string(),
                0,
                true,
                0,
            )
            .await?;

        let per_byte_fee: u64 = unspent_outs_response["per_byte_fee"]
            .as_u64()
            .expect("per_byte_fee should be present");
        let fee_mask: u64 = unspent_outs_response["fee_mask"]
            .as_u64()
            .expect("fee_mask should be present");

        let fork_version: u8 = unspent_outs_response["fork_version"]
            .as_u64()
            .unwrap()
            .try_into()
            .expect("fork_version should be present");

        // hardcoding to low priority for now
        let priority = Priority::PriorityLow;
        let send_transfer = SendTransaction {
            destinations: vec![send_amount_dest],
            priority,
            sweep_all: false,
            payment_id: None,
            from_addr: self.public_address(),
            fork_version,
            per_byte_fee,
            fee_mask,
        };

        let mut unspent_outs =
            MoneroLWSConnection::to_unspent_outputs(self, &unspent_outs_response)?;

        let using_mix_outs =
            Self::light_wallet_get_outs(self, blockchain_client, &unspent_outs).await?;

        let mut pending_transaction =
            self.create_transaction(&send_transfer, &mut unspent_outs, &using_mix_outs)?;

        let signed_pending_tx = self.sign_tx(&mut pending_transaction)?;
        let mut serialized_tx = SerializedArchive::new();
        signed_pending_tx
            .pending_tx
            .tx
            .do_serialize(&mut serialized_tx)?;
        let raw_tx_hex = hex::encode(serialized_tx.data);

        let validated = signed_pending_tx.pending_tx.tx.validate()?;
        if validated {
            match blockchain_client
                .submit_raw_tx(
                    public_address,
                    &self.private_keys().view_key().to_string(),
                    &raw_tx_hex,
                )
                .await
            {
                Ok(response) => {
                    let tx_hash = response
                        .as_str().expect("should be a string");
                    Ok(tx_hash.to_string())
                }
                Err(e) => Err(anyhow!("Error: {}", e)),
            }
        } else {
            Err(anyhow!("Transaction is not valid"))
        }
    }
}

impl MoneroWallet {
    /// Creates an unsigned pending transaction given info about the transaction
    /// to be made, available unspent outs, and mix outs to use
    pub fn create_transaction(
        &self,
        send_transfer: &SendTransaction,
        unspent_outs: &mut Vec<UnspentOutput>,
        using_mix_outs: &[Vec<GetOutsEntry>],
    ) -> Result<PendingTransaction, Error> {
        if !send_transfer.sweep_all {
            for send_amount_to_dest in &send_transfer.destinations {
                let sending_amount = send_amount_to_dest.amount;
                if sending_amount == 0 {
                    return Err(Error::SendingZeroAmount);
                }
            }
        }

        // For light wallet assume we are on the latest hard fork
        // This checks our assumption that our minimum hard fork version
        if send_transfer.fork_version < EXPECTED_MINIMUM_HF_VERSION {
            return Err(Error::InvalidHardForkVersionAssumption {
                found: send_transfer.fork_version,
                expected: EXPECTED_MINIMUM_HF_VERSION,
            });
        }

        // Gather the amounts we are sending to each destination
        let mut sending_amounts: Vec<u64> = Vec::new();
        for send_amount_to_dest in &send_transfer.destinations {
            let sending_amount = send_amount_to_dest.amount;
            sending_amounts.push(sending_amount);
        }

        // Setting these booleans based on the expectation of the hard fork version
        let use_per_byte_fee = true;
        let use_rct = true;
        let bulletproof = true;
        let bulletproof_plus = true;
        let clsag = true;
        let use_view_tags = send_transfer.fork_version >= HF_VERSION_VIEW_TAGS;
        let base_fee = send_transfer.per_byte_fee;
        let fee_quantization_mask = send_transfer.fee_mask;
        let mixin = FAKE_OUTPUTS_COUNT;

        // tx_key: generate a random private key, tx_key_pub a public key derived from
        // the randomly generated private key
        let tx_key = PrivateKey::from_scalar(&Scalar::from_bytes_mod_order(thread_rng().gen()));
        let tx_key_pub = PublicKey::from_private_key(&tx_key);

        let mut extra_nonce: Vec<u8> = vec![];
        let mut tx_extra: Vec<u8> = vec![];

        // check for a long payment id provided in the transaction parameters
        // if a payment_id is provided in send_transfer, add the info to tx_extra
        if let Some(pid) = &send_transfer.payment_id {
            if !extra_nonce.is_empty() {
                return Err(Error::OnlyOnePaymentIdAllowed);
            }
            extra_nonce = pid.extra_nonce()?;
            pid.add_pid_to_tx_extra(&mut tx_extra)?;
        }

        // check for short payment id incorporated in a destination address
        // The short payment id is encrypted and added to tx_extra.
        for destination in &send_transfer.destinations {
            let dst_addr = &destination.addr;
            if let AddressType::Integrated(pid) = &dst_addr.format {
                if !extra_nonce.is_empty() {
                    return Err(Error::OnlyOnePaymentIdAllowed);
                }
                extra_nonce = pid.extra_nonce()?;
                // encrypt the pid
                let encrypted_pid = pid.encrypt_payment_id(&tx_key_pub, &tx_key)?;
                encrypted_pid.add_pid_to_tx_extra(&mut tx_extra)?;
            }
        }

        // add dummy payment id if payment id was not already added, unless we have more
        // than the usual 1 destination + change
        if extra_nonce.is_empty() && send_transfer.destinations.len() > 1 {
            let dummy_pid = PaymentId::from_slice(&[0u8; 8])?;
            let encrypted_pid = dummy_pid.encrypt_payment_id(&tx_key_pub, &tx_key)?;
            encrypted_pid.add_pid_to_tx_extra(&mut extra_nonce)?;
        }

        // TODO(#85): further handling for subaddresses

        // add the tx_key_pub to the tx_extra
        tx_extra.push(TX_EXTRA_TAG_PUBKEY);
        tx_extra.extend(tx_key_pub.as_slice());

        // Initial estimate of the fee, upper bound is to use all of the inputs
        let max_estimated_fee = fee_utils::estimate_fee(
            use_per_byte_fee,
            use_rct,
            unspent_outs.len(),
            FAKE_OUTPUTS_COUNT,
            &sending_amounts.len() + 1,
            tx_extra.len(),
            bulletproof,
            clsag,
            bulletproof_plus,
            use_view_tags,
            base_fee,
            fee_quantization_mask,
        );

        // Add up the amount we need to send, does not include fee or change
        let mut final_total_wo_fee = MoneroAmount::from_piconero(0);
        for sending_amount in &sending_amounts {
            final_total_wo_fee += MoneroAmount::from_piconero(*sending_amount);
        }

        // sort unspent_outs to be in reverse order by amount (large amount to smaller
        // amount), TODO(#68): look into whether we are following best practices here
        unspent_outs.sort_by(|a, b| b.amount.cmp(&a.amount));

        // This will keep track of which of our unspent outs we will actully use and
        // inputs to the tx
        let mut using_outs: Vec<UnspentOutput> = Vec::new();
        let mut using_inds: Vec<usize> = Vec::new();

        // We need to gather enough funds to try to cover the potential total
        let potential_total = if send_transfer.sweep_all {
            MoneroAmount::from_piconero(u64::MAX)
        } else {
            final_total_wo_fee + max_estimated_fee
        };

        // using_outs_amount keeps track of the amount of unspent outs we have selected
        // so far to use
        let mut using_outs_amount = MoneroAmount::from_piconero(0);
        for (i, unspent_out) in unspent_outs.iter().enumerate() {
            if using_outs_amount >= potential_total {
                // breaks for loop, search is over if enough to cover potential_total
                break;
            }

            // handles the case of unspent out has rct
            // we have set use_rct to true earlier in this function, so we should only be
            // using rct outputs
            if let Some(rct) = &unspent_out.rct {
                if !rct.is_empty() {
                    // confirming that the rct string is not empty, before gathering the unspent out
                    using_outs_amount += MoneroAmount::from_piconero(unspent_out.amount);
                    using_outs.push(unspent_out.clone());
                    using_inds.push(i);
                }
            }
        }

        // Returns an error if the amount we gathered is not even enough to cover the
        // amounts we want to send .
        if using_outs_amount < final_total_wo_fee {
            return Err(Error::InsufficientFunds {
                needed: final_total_wo_fee.as_piconero(),
                found: using_outs_amount.as_piconero(),
            });
        }

        // this estimate of the fee uses the actual number of inputs we will be using
        let using_fee = fee_utils::estimate_fee(
            use_per_byte_fee,
            use_rct,
            using_outs.len(),
            FAKE_OUTPUTS_COUNT,
            &sending_amounts.len() + 1,
            tx_extra.len(),
            bulletproof,
            clsag,
            bulletproof_plus,
            use_view_tags,
            base_fee,
            fee_quantization_mask,
        );

        // The required balance is the amount we need to send + the fee,
        // Previously we had calculated the potential total using a higher bound on the
        // fee and we had tried to gather enough funds to cover that potential total.
        let required_balance = final_total_wo_fee + using_fee;
        if using_outs_amount < required_balance {
            return Err(Error::InsufficientFundsForFee {
                needed: required_balance.as_piconero(),
                found: using_outs_amount.as_piconero(),
            });
        }

        // The change amount will be sent back the the sender of the transaction.
        // The change amount is what is in excess of the required balance.
        let change_amount = using_outs_amount - required_balance;

        // Change destination address is the from address of the transaction
        let change_dst = TxDestinationEntry {
            amount: change_amount.as_piconero(),
            addr: send_transfer.from_addr.clone(),
        };

        let mut needed_money = MoneroAmount::from_piconero(0);
        needed_money += final_total_wo_fee;
        needed_money += using_fee;
        // TODO(#68): check on overflow
        // TODO(#68) check subaddress index, all inputs should be from the same account
        // (subaddress major index)
        // Currently assuming all inputs are from the same account and index

        let mut all_rct = true;

        // splitted_dts includes the change destination
        let mut splitted_dsts = send_transfer.destinations.clone();
        splitted_dsts.push(change_dst.clone());

        let mut found_money = MoneroAmount::from_piconero(0);

        // Populate the input sources info
        let mut sources: Vec<TxSourceEntry> = Vec::new();
        let out_index = 0;
        let mut vin: Vec<TxInToKey> = Vec::new();

        let mut in_contexts: Vec<transaction::InputGenerationContext> = Vec::new();
        let mut summary_inputs_money: u64 = 0;

        for (idx, unspent_out) in using_outs.iter().enumerate() {
            all_rct &= unspent_out.is_rct();
            found_money += MoneroAmount::from_piconero(unspent_out.amount);

            let mut src = TxSourceEntry {
                ..Default::default()
            };
            src.amount = unspent_out.amount;
            src.rct = unspent_out.is_rct();

            for n in 0..mixin + 1 {
                let outs_entry = &using_mix_outs[out_index][n];
                let ctkey = rct_types::CtKey {
                    dest: RctKey::from_slice(&outs_entry.1.to_bytes()),
                    mask: outs_entry.2,
                };
                let oe = transaction::OutputEntry(outs_entry.0, ctkey);
                src.outputs.push(oe);
            }

            // Figure out the index of the real output in the outputs vector
            let real_ind = src
                .outputs
                .iter()
                .position(|oe| oe.0 == unspent_out.global_index)
                .ok_or(Error::DidNotFindRealOutputIndex)?;
            let rct_tx_pub_key = PublicKey::from_str(&unspent_out.tx_pub_key.clone())?;
            let _rct_commit = &unspent_out.parse_rct_commit(self, &rct_tx_pub_key)?;
            let rct_mask = &unspent_out.parse_rct_mask(self, &rct_tx_pub_key)?;
            let rct_dest_public_key = PublicKey::from_str(&unspent_out.public_key)?;
            let _real_oe = transaction::OutputEntry(
                unspent_out.global_index,
                rct_types::CtKey {
                    dest: RctKey::from_slice(&rct_dest_public_key.to_bytes()),
                    mask: rct_types::RctKey::commit(src.amount, rct_mask),
                },
            );

            src.real_out_tx_key = PublicKey::from_str(&unspent_out.tx_pub_key.clone())?;
            // src.real_out_additional_tx_keys = // TODO(#68): handle additional tx keys as
            // needed
            src.real_output = real_ind as u64;
            src.real_output_in_tx_index = unspent_out.index;
            src.mask = *rct_mask;

            // TODO(#86): handle multisig src.multisig_kLRki

            if src.real_output >= src.outputs.len() as u64 {
                return Err(Error::RealOutputIndexOutOfBounds {
                    index: src.real_output as usize,
                    size: src.outputs.len(),
                });
            }
            summary_inputs_money += src.amount;
            let k_image = KeyImage::new(
                &self.private_keys(),
                &src.real_out_tx_key,
                src.real_output_in_tx_index,
            )?;

            let out_key =
                PublicKey::from_slice(&src.outputs[src.real_output as usize].1.dest.to_bytes())?;

            // check that derived ephemeral public key is equal to the real output key
            if k_image.ephemeral_public_key != out_key {
                return Err(Error::DerivedNotEqualReal {
                    index: idx,
                    real_out: src.real_output,
                    derived_key: k_image.ephemeral_public_key.to_string(),
                    real_key: hex::encode(src.outputs[src.real_output as usize].1.dest.as_bytes()),
                });
            }

            in_contexts.push(transaction::InputGenerationContext {
                private_key: k_image.ephemeral_private_key,
                public_key: k_image.ephemeral_public_key,
            });

            // handle vin
            let mut key_offsets: Vec<u64> = src.outputs.iter().map(|oe| oe.0).collect();
            TxInToKey::absolute_output_offsets_to_relative(&mut key_offsets);
            let input_to_key = TxInToKey {
                amount: src.amount,
                key_offsets: key_offsets.iter().map(|v| VarInt::<u64>(*v)).collect(),
                k_image, // for double spending protection
            };

            sources.push(src);
            vin.push(input_to_key);
        }
        // sort vins by their key image
        vin.sort_by(|a, b| a.k_image.key_image.cmp(&b.k_image.key_image));

        // TODO(#68): handle additional_tx_pub_keys
        // (we don't need to include additional tx keys if:
        // - all the destinations are standard addresses
        // - there's only one destination which is a subaddress)

        let mut amount_keys: Vec<RctKey> = Vec::new();
        let mut vout: Vec<transaction::TxOut> = Vec::new();
        // hardcoding shuffle_outs to true
        let shuffle_outs = true;
        if shuffle_outs {
            let mut rng = thread_rng();
            splitted_dsts.shuffle(&mut rng);
        }

        let use_view_tags = send_transfer.fork_version >= HF_VERSION_VIEW_TAGS;

        let mut summary_outs_money = 0;
        // handle vout
        for (output_index, dst_entr) in splitted_dsts.iter().enumerate() {
            // output ephemeral keys
            let k_image = KeyImage::new(&self.private_keys(), &tx_key_pub, output_index as u64)?;
            let output_ephemeral_pub_key = k_image.ephemeral_public_key;
            let hash_scalar = k_image.key_derivation.hash_to_scalar(output_index as u64);

            amount_keys.push(RctKey::from_slice(hash_scalar.as_bytes()));

            summary_outs_money += dst_entr.amount;

            if !use_view_tags {
                vout.push(transaction::TxOut {
                    amount: dst_entr.amount,
                    target: transaction::TxOutTargetVariant::ToKey(transaction::TxOutToKey {
                        key: output_ephemeral_pub_key,
                    }),
                });
            } else {
                // derive the view tag
                let view_tag =
                    transaction::ViewTag::derive(&k_image.key_derivation, output_index as u64);
                vout.push(transaction::TxOut {
                    amount: dst_entr.amount,
                    target: transaction::TxOutTargetVariant::ToTaggedKey(
                        transaction::TxOutToTaggedKey {
                            key: output_ephemeral_pub_key,
                            view_tag,
                        },
                    ),
                });
            }
        }

        // TODO(#68): see if need to sort tx_extra

        let extra = transaction::RawExtraField(tx_extra);

        let mut tx_prefix = TransactionPrefix {
            version: 2,                              // because use_rct is true
            unlock_time: transaction::UnlockTime(0), // default unlock_time to 0
            vin,
            vout,
            extra,
        };

        // check mony
        if summary_outs_money > summary_inputs_money {
            return Err(Error::TransactionValue {
                inputs: summary_inputs_money,
                outputs: summary_outs_money,
            });
        }

        // TODO(#68): check for watch only wallet

        // continuing the assumption of the minimum hard fork version checked in the
        // prepare_transaction function

        // hard coding tx.version to 2,
        // hard coding use_simple_rct to true for now, based on rct_config how it is
        // hardcoded here
        let rct_config = rct_types::RctConfig {
            range_proof_type: rct_types::RangeProofType::RangeProofPaddedBulletproof,
            bp_version: 4,
        };

        assert!(all_rct);

        let use_simple_rct = true;

        let mut amount_in: u64 = 0;
        let mut amount_out: u64 = 0;
        let mut in_sk: Vec<rct_types::CtKey> = Vec::new();
        in_sk.reserve(sources.len());
        let mut destinations: Vec<RctKey> = Vec::new();
        let mut in_amounts: Vec<u64> = Vec::new();
        let mut out_amounts: Vec<u64> = Vec::new();
        let mut index: Vec<u64> = Vec::new();
        for i in 0..sources.len() {
            let amount = sources[i].amount;
            amount_in += amount;
            in_amounts.push(amount);
            index.push(sources[i].real_output);
            in_sk.push(rct_types::CtKey {
                dest: RctKey::from_slice(in_contexts[i].private_key.as_slice()),
                mask: sources[i].mask,
            })
        }

        for tx_vout in tx_prefix.vout.iter() {
            out_amounts.push(tx_vout.amount);
            amount_out += tx_vout.amount;
            match &tx_vout.target {
                transaction::TxOutTargetVariant::ToKey(target) => {
                    let output_public_key = RctKey::from_slice(target.key.as_slice());
                    destinations.push(output_public_key);
                }
                transaction::TxOutTargetVariant::ToTaggedKey(target) => {
                    let output_public_key = RctKey::from_slice(target.key.as_slice());
                    destinations.push(output_public_key);
                }
                _ => return Err(Error::UnsupportedTxOutTargetVariant),
            }
        }

        let mut mix_ring: Vec<Vec<rct_types::CtKey>> = Vec::new();
        mix_ring.resize(sources.len(), Vec::new());
        // Currently hardcoding use_simple_rct to true
        if use_simple_rct {
            for i in 0..sources.len() {
                mix_ring[i].resize(sources[i].outputs.len(), rct_types::CtKey::default());
                for n in 0..sources[i].outputs.len() {
                    mix_ring[i][n] = sources[i].outputs[n].1.clone();
                }
            }
        }

        // zero out all amounts to mask rct outputs, real amounts are now encrypted
        for (i, tx_vin) in tx_prefix.vin.iter_mut().enumerate() {
            if sources[i].rct {
                tx_vin.amount = 0;
            }
        }

        for tx_vout in tx_prefix.vout.iter_mut() {
            tx_vout.amount = 0;
        }

        // tx_prefix_hash
        let mut tx_prefix_serialized: SerializedArchive = SerializedArchive::new();
        tx_prefix.do_serialize(&mut tx_prefix_serialized)?;

        let tx_prefix_hash = hash::keccak256(&tx_prefix_serialized.to_bytes());
        let message = RctKey::from_slice(&tx_prefix_hash);
        let mut out_sk: Vec<rct_types::CtKey> = Vec::new();
        let rct_signatures = rct_types::RctSig::generate_rct_simple(
            &message,
            &in_sk,
            &destinations,
            &in_amounts,
            &out_amounts,
            amount_in - amount_out,
            &mix_ring,
            &amount_keys,
            &index,
            &mut out_sk,
            rct_config,
        )?;

        // TODO(#68): add zeroize for in_sk

        let tx = transaction::Transaction {
            prefix: tx_prefix.clone(),
            prunable_hash_valid: false,
            hash_valid: false,
            blob_size_valid: false,
            signatures: Vec::new(),
            rct_signatures,
        };

        if tx.prefix.vout.len() != out_sk.len() {
            return Err(Error::DifferentLengths(tx.prefix.vout.len(), out_sk.len()));
        }

        // TODO(#68): check on if the invalidate_hashes function needs to be implemented

        let tx_construction_data = transaction::TxConstructionData {
            unlock_time: tx.prefix.unlock_time,
            use_rct: true,
            rct_config,
            sources,
            change_dts: change_dst,
            splitted_dsts: splitted_dsts.clone(),
            selected_transfers: vec![0], // Only one transaction handled for now
            extra: tx_prefix.extra.clone(),
            use_view_tags,
            dests: send_transfer.destinations.clone(), /* doesn't include the change
                                                        * destination */
            subaddr_indices: HashSet::new(),
            subadr_account: 0, // TODO(#85) hardcoded to main account for now
        };

        let pending_tx = transaction::PendingTx {
            fee: using_fee.as_piconero(),
            dust_added_to_fee: false, // TODO(#68): check on this
            change_dts: tx_construction_data.change_dts.clone(),
            selected_transfers: tx_construction_data.selected_transfers.clone(),
            key_images: String::new(),
            dests: tx_construction_data.dests.clone(),
            tx_key,
            additional_tx_keys: Vec::new(), /* TODO(#68): add support for additional tx keys as
                                             * needed */
            tx,
            dust: DEFAULT_DUST_THRESHOLD, // TODO(#68): check on this
            multisig_sigs: Vec::new(),    /* TODO(#68): need to add multisig support for monero
                                           * transactions */
            multisig_tx_key_entropy: None,
            construction_data: tx_construction_data,
        };

        Ok(PendingTransaction {
            status: transaction::Status::StatusOk,
            priority: send_transfer.priority,
            pending_tx,
            signers: HashSet::new(),
            key_images: HashSet::new(),
        })
    }

    /// Sign a transaction
    pub fn sign_tx(
        &self,
        unsigned_pending_tx: &mut PendingTransaction,
    ) -> Result<PendingTransaction, anyhow::Error> {
        // tx_sign
        if !unsigned_pending_tx.signers.is_empty() {
            return Err(anyhow!("Transaction already has signers"));
        }

        // Implementing process to fill in the key_images field, assuming do not have to
        // fill in signers field as multisig is not implemented currently
        // TODO(#68): handle multisig case
        let mut key_images: HashSet<KeyImage> = HashSet::new();

        let tx_pub_key = PublicKey::from_private_key(&unsigned_pending_tx.pending_tx.tx_key);

        // TODO(#68): Handle case when additional_tx_pub_keys is not empty
        // for now assuming additional_tx_pub_keys is empty
        assert!(unsigned_pending_tx.pending_tx.additional_tx_keys.is_empty());

        // TODO(#68): need to be able to handle txout_to_tagged_key case as well as the
        // txout_to_key case
        for (i, _vout) in unsigned_pending_tx
            .pending_tx
            .tx
            .prefix
            .vout
            .iter()
            .enumerate()
        {
            let key_image_info = KeyImage::new(&self.private_keys, &tx_pub_key, i as u64)?;
            key_images.insert(key_image_info);
        }
        let mut signed_pending_tx = unsigned_pending_tx.clone();
        signed_pending_tx.key_images = key_images;
        Ok(signed_pending_tx)
    }

    /// Returns the public keys for the wallet
    pub fn public_keys(&self) -> MoneroPublicKeys {
        self.public_keys
    }

    /// Returns the private keys for the wallet
    pub fn private_keys(&self) -> MoneroPrivateKeys {
        self.private_keys
    }

    /// Returns the public address for the wallet
    pub fn public_address(&self) -> Address {
        self.public_address.clone()
    }

    /// Returns the network type for the wallet
    pub fn network(&self) -> Network {
        self.network
    }

    /// Handles getting the relevant outputs from the server and creating the
    /// outs including decoys needed for a transaction
    /// Implemented based on the `light_wallet_get_outs` function from the
    /// Monero codebase while implementing some updates
    /// **Source** <`monero/src/wallet/wallet2.cpp`>(https://github.com/monero-project/monero/blob/75d80d431a9586996c559cb39f3eabebad3da60a/src/wallet/wallet2.cpp#L8030-L8135)
    pub async fn light_wallet_get_outs(
        &self,
        blockchain_client: &MoneroLWSConnection,
        using_outs: &Vec<UnspentOutput>,
    ) -> Result<Vec<Vec<GetOutsEntry>>, Error> {
        println!("In light_wallet_get_outs");
        let mut vec_needed: Vec<u64> = Vec::new();

        for using_out in using_outs {
            if using_out.is_rct() {
                // amount is hidden, always set to 0 when using rct
                vec_needed.push(0);
            } else {
                vec_needed.push(using_out.amount);
            }
        }

        let random_outs_server = blockchain_client.get_random_outs(vec_needed).await.unwrap();

        // Check the number of outputs for each amount, checks if it meets the
        // expectation
        for mix_out in &random_outs_server {
            if mix_out.outputs.len() != FAKE_OUTPUTS_COUNT + 1 {
                return Err(Error::IncorrectNumberOfOutputs {
                    expected: FAKE_OUTPUTS_COUNT,
                    found: mix_out.outputs.len(),
                });
            }
        }

        // TODO(#68): see if can use some chache like this for more efficiency?
        // let mut valid_public_keys_cache: HashSet<PublicKey> = HashSet::new();

        let mut outs: Vec<Vec<GetOutsEntry>> = Vec::new();
        for (idx, using_out) in using_outs.iter().enumerate() {
            // add real output first
            let tx_pub_key = PublicKey::from_str(&using_out.public_key)?;
            let _rct_commit = using_out.parse_rct_commit(self, &tx_pub_key)?;
            let rct_mask = using_out.parse_rct_mask(self, &tx_pub_key)?;
            let real_output = GetOutsEntry(
                using_out.global_index,
                PublicKey::from_str(&using_out.public_key)?,
                RctKey::commit(using_out.amount, &rct_mask),
            );
            let mut idx_out = vec![real_output];

            // Add the fake outputs (decoys)
            // Pick from the results from the lighwallet server in a random order
            let mut rng = rand::thread_rng();
            let mut random_order: Vec<usize> = (0..FAKE_OUTPUTS_COUNT).collect();
            random_order.shuffle(&mut rng);
            for i in random_order {
                let amount_key = idx;
                let global_index = &random_outs_server[amount_key].outputs[i].global_index;
                let real_index = using_out.global_index;
                if *global_index == real_index {
                    // Fake output has the same global index as the real output
                    // This should be very rare, if it happens the error can be handled by the
                    // wallet and the wallet can try again to call the light_w
                    return Err(Error::FakeOutputHasSameGlobalIndex);
                }

                let output_public_key =
                    PublicKey::from_str(&random_outs_server[amount_key].outputs[i].public_key)?;
                let rct_commit = using_out.parse_rct_commit(self, &output_public_key)?;
                let _mask = using_out.parse_rct_mask(self, &output_public_key)?;

                // TODO(#68): not sure if using this cache correctly for anything useful
                // See if these commented lines can be removed
                // valid_public_keys_cache.insert(output_public_key);
                // valid_public_keys_cache.insert(PublicKey::from_slice(mask.bytes.as_slice())?
                // );
                idx_out.push(GetOutsEntry(*global_index, output_public_key, rct_commit));
            }
            // Sort by global index
            idx_out.sort_by(|a, b| a.0.cmp(&b.0));
            outs.push(idx_out);
        }
        Ok(outs)
    }
}

// TODO(#61): Remove this display trait implementation as it is overly specified
// and does not belong in this library
impl Display for MoneroWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Monero Wallet")?;
        writeln!(f, " Network: {:?}", self.network)?;
        if let Some(private_spend_key) = self.private_keys.spend_key() {
            writeln!(f, " Private Spend Key: {}", private_spend_key)?;
        }
        writeln!(f, " Private View Key: {}", self.private_keys.view_key())?;
        if let Some(public_spend_key) = self.public_keys.spend_key() {
            writeln!(f, " Public Spend Key: {}", public_spend_key)?;
        }
        if let Some(public_view_key) = self.public_keys.view_key() {
            writeln!(f, " Public View Key: {}", public_view_key)?;
        }
        writeln!(f, " Public Address: {}", self.public_address)?;
        Ok(())
    }
}

impl CryptoAddressGeneral for MoneroWallet {
    fn crypto_type(&self) -> SlipCoin {
        SlipCoin::XMR
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> Box<dyn CryptoAddressGeneral> {
        Box::new(self.clone())
    }
}
