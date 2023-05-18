//! This module contains the implementation of the handling getting information to and from the bitcoin blockchain using the mempool.space Esplora JSON over HTTP API <https://mempool.space/docs/api/rest>
//!

// signet - https://mempool.space/signet/api
// mainnet - https://mempool.space/api
use crate::{
    connectors::{BTransaction, FeeEstimates, Utxos},
    Error,
};
use async_trait::async_trait;
use walletd_coin_core::BlockchainConnector;

/// A blockchain connector for Bitcoin which follows [`the Mempool Space API`](https://mempool.space/docs/api/rest).
#[derive(Clone, Default, Debug)]
pub struct MempoolSpace {
    /// The client used to make requests to the API
    pub client: reqwest::Client,
    /// The url of the API
    pub url: String,
}

#[async_trait]
impl BlockchainConnector for MempoolSpace {
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

impl MempoolSpace {
    /// Fetch the block height
    pub async fn block_height(&self) -> Result<u64, Error> {
        let body = reqwest::get(format!("{}/blocks/tip/height", self.url))
            .await?
            .text()
            .await?;
        let block_count = body
            .parse::<u64>()
            .map_err(|e| Error::FromStr(e.to_string()))?;
        Ok(block_count)
    }

    /// Fetch fee estimates
    pub async fn fee_estimates(&self) -> Result<FeeEstimates, Error> {
        let body = reqwest::get(format!("{}/v1/fees/recommended", self.url))
            .await?
            .text()
            .await?;
        let fee_estimates: FeeEstimates = serde_json::from_str(&body)?;
        Ok(fee_estimates)
    }

    /// Fetch transactions
    pub async fn transactions(&self, address: &str) -> Result<Vec<BTransaction>, Error> {
        let body = reqwest::get(format!("{}/address/{}/txs", self.url, address))
            .await?
            .text()
            .await?;
        let transactions: Vec<BTransaction> = serde_json::from_str(&body)?;
        Ok(transactions)
    }

    /// Fetch mempool transactions
    pub fn mempool_transactions(&self, address: &str) -> Result<Vec<BTransaction>, Error> {
        let body = reqwest::blocking::get(format!("{}/address/{}/txs/mempool", self.url, address))
            .expect("Error getting transactions")
            .text();
        let transactions: Vec<BTransaction> = serde_json::from_str(&body?)?;
        Ok(transactions)
    }

    /// Fetch UTXOs
    pub async fn utxo(&self, address: &str) -> Result<Utxos, Error> {
        let body = reqwest::get(format!("{}/address/{}/utxo", self.url, address))
            .await?
            .text()
            .await?;

        let utxos: Utxos = serde_json::from_str(&body)?;
        Ok(utxos)
    }

    /// Fetch raw transaction hex for a given txid
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
    pub async fn broadcast_tx(&self, raw_transaction_hex: &'static str) -> Result<String, Error> {
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
    use crate::connectors::{Input, Output, Status, Utxo};

    use super::*;
    use mockito::Server;
    use serde_json::{json, Number, Value};

    #[tokio::test]
    async fn test_block_count() {
        let mut server = Server::new();
        let expected_blockcount = 773876;
        server
            .mock("GET", "/blocks/tip/height")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(expected_blockcount.to_string())
            .create();

        let bs = MempoolSpace::new(&server.url()).unwrap();
        let check_blockcount = bs.block_height().await.unwrap();
        assert_eq!(expected_blockcount, check_blockcount);
    }

    #[tokio::test]
    async fn test_fee_estimates() {
        let mut server = Server::new();
        let mut expected_fee_map = serde_json::Map::new();
        expected_fee_map.insert(
            String::from("economyFee"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("fastestFee"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("halfHourFee"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("hourFee"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );
        expected_fee_map.insert(
            String::from("minimumFee"),
            Value::Number(Number::from_f64(1.0).unwrap()),
        );

        server
            .mock("GET", "/v1/fees/recommended")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&Value::Object(expected_fee_map.clone()).to_string())
            .create();

        let bs = MempoolSpace::new(&server.url()).unwrap();
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

        let bs = MempoolSpace::new(&server.url()).unwrap();
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

        let bs = MempoolSpace::new(&server.url()).unwrap();
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
        let bs = MempoolSpace::new(&server.url()).unwrap();
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
        let bs = MempoolSpace::new(&server.url()).unwrap();
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
        let bs = MempoolSpace::new(&server.url()).unwrap();
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
        let bs = MempoolSpace::new(&server.url()).unwrap();
        let txid = bs.broadcast_tx(raw_tx_data).await.unwrap();
        assert_eq!(txid, expected_txid);
    }
}
