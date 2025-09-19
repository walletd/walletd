//! eXtensible Blockchain Object Model (XBOM) implementation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbomObject {
    pub id: String,
    pub class: String,
    pub owner: String,
    pub state: serde_json::Value,
    pub methods: Vec<String>,
}

impl XbomObject {
    pub fn new(class: String, owner: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            class,
            owner,
            state: serde_json::Value::Object(Default::default()),
            methods: Vec::new(),
        }
    }
}
pub mod serializer;
pub use serializer::*;
