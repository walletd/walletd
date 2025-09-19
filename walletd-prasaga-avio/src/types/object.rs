use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectClass(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectOperation {
    Create {
        class: ObjectClass,
        initial_state: serde_json::Value,
    },
    Update {
        id: ObjectId,
        state_changes: serde_json::Value,
    },
    Transfer {
        id: ObjectId,
        to: String,
    },
    InvokeMethod {
        id: ObjectId,
        method: String,
        params: Vec<serde_json::Value>,
    },
}
