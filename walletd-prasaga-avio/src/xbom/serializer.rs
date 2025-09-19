use crate::types::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbomObject {
    pub id: String,
    pub class: String,
    pub owner: String,
    pub state: serde_json::Value,
    pub methods: Vec<XbomMethod>,
    pub permissions: XbomPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbomMethod {
    pub name: String,
    pub params: Vec<XbomParam>,
    pub visibility: MethodVisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbomParam {
    pub name: String,
    pub param_type: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MethodVisibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbomPermissions {
    pub transferable: bool,
    pub mutable: bool,
    pub executable: bool,
}

pub struct XbomSerializer;

impl XbomSerializer {
    /// Serialize an XBOM object to bytes (using JSON for compatibility)
    pub fn serialize(object: &XbomObject) -> Result<Vec<u8>> {
        // Use JSON serialization instead of bincode for serde_json::Value compatibility
        serde_json::to_vec(object).map_err(Error::Serialization)
    }

    /// Deserialize bytes to an XBOM object
    pub fn deserialize(data: &[u8]) -> Result<XbomObject> {
        serde_json::from_slice(data).map_err(Error::Serialization)
    }

    /// Serialize to compact binary format (without arbitrary JSON values)
    pub fn serialize_binary(object: &XbomObject) -> Result<Vec<u8>> {
        // Convert to a binary-safe format first
        let binary_safe = BinaryXbomObject {
            id: object.id.clone(),
            class: object.class.clone(),
            owner: object.owner.clone(),
            state_json: object.state.to_string(),
            methods: object.methods.clone(),
            permissions: object.permissions.clone(),
        };

        bincode::serialize(&binary_safe)
            .map_err(|e| Error::Unknown(format!("Binary serialization failed: {e}")))
    }

    /// Create a method call payload
    pub fn create_method_call(
        object_id: &str,
        method_name: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<Vec<u8>> {
        let call = serde_json::json!({
            "object_id": object_id,
            "method": method_name,
            "params": params,
        });

        serde_json::to_vec(&call).map_err(Error::Serialization)
    }
}

// Binary-safe version for bincode serialization
#[derive(Debug, Serialize, Deserialize)]
struct BinaryXbomObject {
    id: String,
    class: String,
    owner: String,
    state_json: String, // JSON as string for binary serialization
    methods: Vec<XbomMethod>,
    permissions: XbomPermissions,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xbom_serialization() {
        let object = XbomObject {
            id: "obj123".to_string(),
            class: "Token".to_string(),
            owner: "saga1abc".to_string(),
            state: serde_json::json!({"balance": 1000}),
            methods: vec![XbomMethod {
                name: "transfer".to_string(),
                params: vec![
                    XbomParam {
                        name: "to".to_string(),
                        param_type: "address".to_string(),
                        optional: false,
                    },
                    XbomParam {
                        name: "amount".to_string(),
                        param_type: "uint256".to_string(),
                        optional: false,
                    },
                ],
                visibility: MethodVisibility::Public,
            }],
            permissions: XbomPermissions {
                transferable: true,
                mutable: true,
                executable: true,
            },
        };

        // Test JSON serialization
        let serialized = XbomSerializer::serialize(&object).unwrap();
        assert!(!serialized.is_empty());

        let deserialized = XbomSerializer::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.id, object.id);
        assert_eq!(deserialized.class, object.class);

        // Test binary serialization
        let binary = XbomSerializer::serialize_binary(&object).unwrap();
        assert!(!binary.is_empty());
    }

    #[test]
    fn test_method_call_creation() {
        let call_bytes = XbomSerializer::create_method_call(
            "obj123",
            "transfer",
            vec![serde_json::json!("saga1recipient"), serde_json::json!(1000)],
        )
        .unwrap();

        assert!(!call_bytes.is_empty());

        // Verify it can be deserialized
        let call_json: serde_json::Value = serde_json::from_slice(&call_bytes).unwrap();
        assert_eq!(call_json["object_id"], "obj123");
        assert_eq!(call_json["method"], "transfer");
    }
}
