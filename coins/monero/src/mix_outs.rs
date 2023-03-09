use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use thiserror::Error;

use crate::MoneroAmount;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MixAmountAndOuts {
    pub amount: MoneroAmount,
    pub outputs: Vec<MixOut>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MixOut {
    pub global_index: u64,
    pub public_key: String,
    pub rct: Option<String>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Don't have array info")]
    MissingArrayInfo,
    #[error("Mix amount and outs info not available")]
    MixAmountInfoMissing,
    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("hex error: {0}")]
    HexError(#[from] hex::FromHexError),
}

impl MixAmountAndOuts {
    pub fn new_vec_from_value(value: Value) -> Result<Vec<MixAmountAndOuts>, Error> {
        let mut vec_mix_outs: Vec<MixAmountAndOuts> = Vec::new();
        if value.is_array() {
            if let Value::Array(vec) = value {
                for item in vec.iter() {
                    let mix_outs = MixAmountAndOuts::from_value(item.clone())?;
                    vec_mix_outs.push(mix_outs);
                }
            }
            Ok(vec_mix_outs)
        } else {
            Err(Error::MissingArrayInfo)
        }
    }

    pub fn from_value(value: Value) -> Result<MixAmountAndOuts, Error> {
        let mut mix_outs = MixAmountAndOuts {
            ..Default::default()
        };
        println!("mix_outs from value: {:?}", value);

        if let Value::Object(object) = value {
            for obj_item in object {
                if obj_item.0 == "amount" {
                    mix_outs.amount =
                        MoneroAmount::from_piconero(serde_json::from_value(Value::from(
                            Number::from_str(obj_item.1.clone().as_str().expect("expected a str"))?,
                        ))?);
                } else if obj_item.0 == "outputs" {
                    mix_outs.outputs = MixOut::new_vec_from_value(obj_item.1.clone())?;
                }
            }
            Ok(mix_outs)
        } else {
            return Err(Error::MixAmountInfoMissing);
        }
    }
}

impl MixOut {
    pub fn new_vec_from_value(value: Value) -> Result<Vec<MixOut>, Error> {
        let mut vec_mix_outs: Vec<MixOut> = Vec::new();
        if value.is_array() {
            if let Value::Array(vec) = value {
                for item in vec.iter() {
                    let mut mix_out: MixOut = MixOut {
                        ..Default::default()
                    };
                    if let Value::Object(map) = item {
                        for map_item in map {
                            if map_item.0 == "global_index" {
                                mix_out.global_index =
                                    serde_json::from_value(Value::from(Number::from_str(
                                        map_item.1.clone().as_str().expect("expected a str"),
                                    )?))?;
                            } else if map_item.0 == "public_key" {
                                mix_out.public_key = serde_json::from_value(map_item.1.clone())?;
                            } else if map_item.0 == "rct" {
                                mix_out.rct = serde_json::from_value(map_item.1.clone())?;
                            }
                        }
                    }
                    vec_mix_outs.push(mix_out);
                }
            }
            Ok(vec_mix_outs)
        } else {
            Err(Error::MissingArrayInfo)
        }
    }
}
