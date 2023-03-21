//! Monero LWS Client - Facilitates connection to a Monero Light Wallet Server
//!
//! This module allows for connecting to a Monero Light Wallet Server and
//! getting Monero blockchain information from it. MyMonero is an example Monero
//! LWS that can be connected to using this module

use reqwest;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use serde_json::Value;
use thiserror::Error;

use crate::rct_types::RctKey;
use crate::{
    mix_outs, public_key, KeyDerivation, KeyImage, MixAmountAndOuts, MoneroWallet, PublicKey,
};

/// The number of fake outputs to request from the LWS
pub const FAKE_OUTPUTS_COUNT: usize = 15;
/// The default dust threshold for Monero
pub const DEFAULT_DUST_THRESHOLD: u64 = 2000000000; // 2 * pow(10, 9)

#[derive(Error, Debug)]
pub enum Error {
    #[error("Status code error: client error, content: {0}")]
    ClientSideError(String),
    #[error("Status code error: server error, content: {0}")]
    ServerSideError(String),
    #[error("Status code error: client error and server error, content: {0}")]
    ErrorClientSideAndServerSide(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("mix_outs error: {0}")]
    MixOutsError(#[from] mix_outs::Error),
    #[error("hex error: {0}")]
    HexError(#[from] hex::FromHexError),
    #[error("Public key error: {0}")]
    PublicKeyError(#[from] public_key::Error),
    #[error("Invalid rct string length")]
    InvalidRctStringLength,
    #[error("Error from the key image module: {0}")]
    KeyImageError(#[from] crate::key_image::Error),
}

#[derive(Clone, Default, Debug)]
pub struct MoneroLWSConnection {
    pub client: reqwest::Client,
    pub url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UnspentOutput {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub amount: u64,
    pub index: u64,
    pub global_index: u64,
    pub public_key: String,
    pub rct: Option<String>,
    pub tx_pub_key: String,
}

impl Serialize for UnspentOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("UnspentOutput", 6)?;
        state.serialize_field("amount", &self.amount)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("global_index", &self.global_index)?;
        state.serialize_field("public_key", &self.public_key)?;
        state.serialize_field("rct", &self.rct)?;
        state.serialize_field("tx_pub_key", &self.tx_pub_key)?;
        state.end()
    }
}
impl UnspentOutput {
    /// Returns true if the UnspentOutput is a ring confidential transaction
    pub fn is_rct(&self) -> bool {
        match &self.rct {
            Some(rct) => !rct.is_empty(),
            None => false,
        }
    }

    /// Parses the rct string and returns the commit value as a Key
    pub fn parse_rct_commit(
        &self,
        _account: &MoneroWallet,
        _tx_pub_key: &PublicKey,
    ) -> Result<RctKey, Error> {
        match &self.rct {
            None => Ok(RctKey::zero_commit(self.amount)),
            Some(rct_string) => {
                if rct_string.is_empty() {
                    Ok(RctKey::zero_commit(self.amount))
                } else if rct_string.len() >= 64 {
                    let rct_commit_str = &rct_string[0..64];
                    let rct_commit = hex::decode(rct_commit_str)?;
                    Ok(RctKey::from_slice(&rct_commit))
                } else {
                    Err(Error::InvalidRctStringLength)
                }
            }
        }
    }

    /// Parses the rct string and returns the mask value as a Key
    pub fn parse_rct_mask(
        &self,
        account: &MoneroWallet,
        tx_pub_key: &PublicKey,
    ) -> Result<RctKey, Error> {
        match &self.rct {
            None => Ok(RctKey::identity()),
            Some(rct_string) => {
                if rct_string.is_empty() {
                    Ok(RctKey::identity())
                } else if rct_string.len() < 128 {
                    if rct_string == "coinbase" {
                        return Ok(RctKey::identity());
                    }
                    let key_deriv =
                        KeyDerivation::generate(tx_pub_key, &account.private_keys().view_key());
                    let derived_sec_key = key_deriv.derive_private_key(
                        self.index,
                        &account
                            .private_keys()
                            .spend_key()
                            .expect("expecting private spend key"),
                    )?;
                    return Ok(RctKey::gen_commitment_mask(&RctKey::from_slice(
                        &derived_sec_key.to_bytes(),
                    )));
                } else if rct_string.len() == 128 {
                    let encrypted_mask = RctKey::from_slice(&hex::decode(&rct_string[64..128])?);
                    if encrypted_mask == RctKey::identity() {
                        return Ok(encrypted_mask);
                    }
                    let key_deriv =
                        KeyDerivation::generate(tx_pub_key, &account.private_keys().view_key());
                    let decrypted_mask =
                        encrypted_mask.as_scalar() - key_deriv.hash_to_scalar(self.index);
                    return Ok(RctKey::from_slice(&decrypted_mask.to_bytes()));
                } else {
                    return Err(Error::InvalidRctStringLength);
                }
            }
        }
    }
}
impl MoneroLWSConnection {
    pub fn new(url: &str) -> Result<Self, Error> {
        Ok(Self {
            client: reqwest::Client::new(),
            url: url.to_string(),
        })
    }

    /// Internal function used to parse the response, check for error in
    /// response
    async fn parse_response_error(response: reqwest::Response) -> Result<Value, Error> {
        let status = response.status();
        let content = response.text().await?;
        if status.is_client_error() && status.is_server_error() {
            Err(Error::ErrorClientSideAndServerSide(content))
        } else if status.is_client_error() {
            return Err(Error::ClientSideError(content));
        } else if status.is_server_error() {
            return Err(Error::ServerSideError(content));
        } else {
            let content_json = serde_json::from_str(content.as_str())?;
            return Ok(content_json);
        }
    }

    /// Login endpoint, provide public address and private view key
    pub async fn login(
        &self,
        public_address: &str,
        private_view_key: &str,
        create_account: Option<bool>,
        generated_locally: Option<bool>,
    ) -> Result<Value, Error> {
        let mut body = serde_json::Map::new();
        body.insert("address".into(), serde_json::to_value(public_address)?);
        body.insert("view_key".into(), serde_json::to_value(private_view_key)?);
        if let Some(create_account_bool) = create_account {
            body.insert(
                "create_account".into(),
                serde_json::to_value(create_account_bool)?,
            );
        }
        if let Some(generated_locally_bool) = generated_locally {
            body.insert(
                "generated_locally".into(),
                serde_json::to_value(generated_locally_bool)?,
            );
        }
        let response = self
            .client
            .post(format!("{}/login", self.url))
            .json(&body)
            .send()
            .await?;
        Self::parse_response_error(response).await
    }

    /// Get address info endpoint
    pub async fn get_address_info(
        &self,
        public_address: &str,
        private_view_key: &str,
    ) -> Result<Value, Error> {
        let mut body = serde_json::Map::new();
        body.insert("address".into(), serde_json::to_value(public_address)?);
        body.insert("view_key".into(), serde_json::to_value(private_view_key)?);
        let response = self
            .client
            .post(format!("{}/get_address_info", self.url))
            .json(&body)
            .send()
            .await?;
        Self::parse_response_error(response).await
    }

    /// Get address txs endpoint
    pub async fn get_address_txs(
        &self,
        public_address: &str,
        private_view_key: &str,
    ) -> Result<Value, Error> {
        let mut body = serde_json::Map::new();
        body.insert("address".into(), serde_json::to_value(public_address)?);
        body.insert("view_key".into(), serde_json::to_value(private_view_key)?);

        let response = self
            .client
            .post(format!("{}/get_address_txs", self.url))
            .json(&body)
            .send()
            .await?;
        Self::parse_response_error(response).await
    }

    /// Get unspent outs
    pub async fn get_unspent_outs(
        &self,
        public_address: &str,
        private_view_key: &str,
        amount: u64,
        use_dust: bool,
        dust_threshold: u64,
    ) -> Result<Value, Error> {
        let mut body = serde_json::Map::new();
        body.insert("address".into(), serde_json::to_value(public_address)?);
        body.insert("view_key".into(), serde_json::to_value(private_view_key)?);
        body.insert("amount".into(), serde_json::to_value(amount.to_string())?);
        body.insert("use_dust".into(), serde_json::to_value(use_dust)?);
        if use_dust {
            body.insert(
                "dust_threshold".into(),
                serde_json::to_value(dust_threshold.to_string())?,
            );
        }

        let response = self
            .client
            .post(format!("{}/get_unspent_outs", self.url))
            .json(&body)
            .send()
            .await?;
        Self::parse_response_error(response).await
    }

    /// Get random outs
    pub async fn get_random_outs(&self, amounts: Vec<u64>) -> Result<Vec<MixAmountAndOuts>, Error> {
        let mut body = serde_json::Map::new();
        body.insert(
            "amounts".into(),
            serde_json::to_value(
                amounts
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            )?,
        );
        // hardcoding this to comply with current monero fork requirments
        body.insert(
            "count".into(),
            serde_json::to_value(FAKE_OUTPUTS_COUNT + 1)?,
        );
        let response = self
            .client
            .post(format!("{}/get_random_outs", self.url))
            .json(&body)
            .send()
            .await?;
        let value = Self::parse_response_error(response).await?;
        let amount_outs = value["amount_outs"].clone();
        let random_outs: Vec<MixAmountAndOuts> = MixAmountAndOuts::new_vec_from_value(amount_outs)?;
        Ok(random_outs)
    }

    /// Submits the raw transaction to the network
    pub async fn submit_raw_tx(
        &self,
        public_address: &str,
        private_view_key: &str,
        raw_tx: &str,
    ) -> Result<Value, Error> {
        let mut body = serde_json::Map::new();
        body.insert("address".into(), serde_json::to_value(public_address)?);
        body.insert("view_key".into(), serde_json::to_value(private_view_key)?);
        body.insert("tx".into(), serde_json::to_value(raw_tx)?);
        let response = self
            .client
            .post(format!("{}/submit_raw_tx", self.url))
            .json(&body)
            .send()
            .await?;
        Self::parse_response_error(response).await
    }

    /// Discerns the unspent outputs belonging to the account wallet
    pub fn to_unspent_outputs(
        account_wallet: &MoneroWallet,
        unspent_outs_response: &Value,
    ) -> Result<Vec<UnspentOutput>, anyhow::Error> {
        let account_private_keys = account_wallet.private_keys();

        let mut spendable_outputs: Vec<UnspentOutput> = Vec::new();
        let outputs = &unspent_outs_response["outputs"]
            .as_array()
            .expect("expected outputs to be an array");
        for output in outputs.iter() {
            let spend_key_images = &output["spend_key_images"]
                .as_array()
                .expect("expected a spend_key_images array");
            let mut is_output_unspent = true;
            let mut tx_public_key = [0u8; 32];
            tx_public_key.copy_from_slice(
                hex::decode(output["tx_pub_key"].as_str().expect("Expected tx_pub_key"))?
                    .as_slice(),
            );

            let tx_public_key = PublicKey::from_slice(
                hex::decode(output["tx_pub_key"].as_str().expect("Expected tx_pub_key"))?
                    .as_slice(),
            )
            .expect("Expected tx_pub_key to be a valid public key");
            let out_index = output["index"].as_u64().expect("Expected out_index as u64");

            let calculated_key_image =
                KeyImage::new(&account_private_keys, &tx_public_key, out_index)?
                    .key_image
                    .to_vec();

            // need to filter here for what is actually available to be spent (what is
            // unspent)
            for spend_key_image in spend_key_images.iter() {
                let key_image = hex::decode(
                    spend_key_image
                        .as_str()
                        .expect("Expected spend key image str"),
                )?;
                if key_image == calculated_key_image {
                    is_output_unspent = false;
                }
            }
            if is_output_unspent {
                let spendable_output: UnspentOutput = serde_json::from_value(output.clone())?;
                spendable_outputs.push(spendable_output);
            }
        }
        Ok(spendable_outputs)
    }
}
