use anyhow::{Error, Result};
use serde::Serialize;
use serde_json::Map;
use std::fmt::Write;

/// Trait for objects that can be serialized into a `SerializedArchive`.
/// Implementors must define how they serialize their data.
pub trait DoSerialize {
    /// Serializes the object into the provided `SerializedArchive`.
    /// Returns `Ok(())` on success, or an `Error` on failure.
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), Error>;
}

/// Structure to hold serialized data in both binary and JSON formats.
/// Tracks depth and object state for proper JSON formatting.
#[derive(Debug, Clone, Default)]
pub struct SerializedArchive {
    /// Binary representation of the serialized data.
    pub data: Vec<u8>,
    /// JSON string representation of the serialized data.
    pub json_stream: String,
    /// Current nesting depth of objects/arrays.
    pub depth: usize,
    /// Indicates if an object has just begun, for comma placement.
    pub object_begin: bool,
}

impl SerializedArchive {
    /// Creates a new, empty `SerializedArchive` with default values.
    pub fn new() -> Self {
        SerializedArchive {
            data: Vec::new(),
            json_stream: String::new(),
            depth: 0,
            object_begin: false,
        }
    }

    /// Returns the serialized data as a byte slice for inspection.
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    /// Returns a copy of the serialized data as a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Serializes a vector of items that implement `DoSerialize`.
    /// Adds a tag and formats the vector as a JSON array.
    pub fn serialize_vector<T: DoSerialize>(&mut self, tag: &str, vec: &[T]) -> Result<(), Error> {
        self.json_stream.push_str(&format!("\"{tag}\": "));
        self.begin_array();
        for (i, item) in vec.iter().enumerate() {
            item.do_serialize(self)?;
            if i < vec.len() - 1 {
                self.delimit_array();
            }
        }
        self.end_array();
        Ok(())
    }

    /// Serializes a field with a given name and a serializable value.
    /// Formats the field as a JSON key-value pair.
    pub fn serialize_field<T: Serialize>(
        &mut self,
        field_name: &str,
        field_value: &T,
    ) -> Result<(), Error> {
        let mut map = Map::new();
        map.insert(field_name.to_string(), serde_json::to_value(field_value)?);
        let _json_str = serde_json::to_string(&map)?;
        self.json_stream.push_str(&format!(
            "\"{}\": {}",
            field_name,
            serde_json::to_string(field_value)?
        )); // Remove braces
        Ok(())
    }

    /// Serializes a byte slice as a key, encoding it as hex in the JSON stream.
    pub fn serialize_key(&mut self, key: &[u8]) -> Result<(), Error> {
        self.data.extend_from_slice(key);
        self.json_stream.push('\"');
        self.json_stream.push_str(&hex::encode(key));
        self.json_stream.push('\"');
        Ok(())
    }

    /// Incorporates raw bytes directly into the binary data and JSON stream.
    /// Bytes are encoded as hex in the JSON stream.
    pub fn serialize_directly(&mut self, blob: &[u8]) -> Result<(), Error> {
        self.data.extend_from_slice(blob);
        self.json_stream.push_str(&hex::encode(blob));
        Ok(())
    }

    /// Adds a JSON tag (key) to the serialized output with proper formatting.
    /// Handles comma placement based on object state.
    pub fn add_tag(&mut self, tag: &str) {
        if !self.object_begin {
            self.json_stream.push_str(", ");
        } else {
            self.object_begin = false;
        }
        self.json_stream.push('\"');
        self.json_stream.push_str(tag);
        self.json_stream.push_str("\": ");
    }

    /// Begins a new JSON object, increasing depth and setting object state.
    pub fn begin_object(&mut self) {
        self.json_stream.push('{');
        self.depth += 1;
        self.object_begin = true;
    }

    /// Ends the current JSON object, decreasing depth.
    pub fn end_object(&mut self) {
        self.json_stream.push('}');
        self.depth -= 1;
        self.object_begin = false;
    }

    /// Begins a new JSON array, increasing depth.
    pub fn begin_array(&mut self) {
        self.json_stream.push('[');
        self.depth += 1;
    }

    /// Begins a JSON array and encodes the size in binary data.
    pub fn begin_array_with_size(&mut self, size: usize) -> Result<(), Error> {
        self.begin_array();
        self.data.extend_from_slice(&size.to_le_bytes());
        Ok(())
    }

    /// Adds a comma delimiter between array elements.
    pub fn delimit_array(&mut self) {
        self.json_stream.push_str(", ");
    }

    /// Ends the current JSON array, decreasing depth.
    pub fn end_array(&mut self) {
        self.json_stream.push(']');
        self.depth -= 1;
    }

    /// Serializes an object implementing `DoSerialize` as a JSON object.
    pub fn serialize_object(&mut self, object: &dyn DoSerialize) -> Result<(), Error> {
        self.begin_object();
        object.do_serialize(self)?;
        self.end_object();
        Ok(())
    }
}

impl DoSerialize for usize {
    /// Serializes a `usize` value into the binary data and JSON stream.
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), Error> {
        serialized.data.extend_from_slice(&self.to_le_bytes());
        write!(serialized.json_stream, "{self}")?;
        Ok(())
    }
}
impl SerializedArchive {
    pub fn serialize_vector_variant<T, V>(
        &mut self,
        tag: &str,
        vec: &[T],
        _variant: V,
    ) -> Result<(), Error>
    where
        T: DoSerialize,
    {
        self.serialize_vector(tag, vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_archive() {
        let archive = SerializedArchive::new();
        assert_eq!(archive.data.len(), 0);
        assert_eq!(archive.json_stream, "");
        assert_eq!(archive.depth, 0);
        assert!(!archive.object_begin);
    }

    #[test]
    fn test_serialize_key() {
        let mut archive = SerializedArchive::new();
        let key = vec![0x1, 0x2, 0x3];
        archive.serialize_key(&key).unwrap();
        assert_eq!(archive.data, vec![0x1, 0x2, 0x3]);
        assert_eq!(archive.json_stream, "\"010203\"");
    }

    #[test]
    fn test_serialize_field() {
        let mut archive = SerializedArchive::new();
        archive.serialize_field("test", &42).unwrap();
        assert_eq!(archive.json_stream, "\"test\": 42");
    }

    #[test]
    fn test_begin_end_object() {
        let mut archive = SerializedArchive::new();
        archive.begin_object();
        assert_eq!(archive.depth, 1);
        assert_eq!(archive.json_stream, "{");
        archive.end_object();
        assert_eq!(archive.depth, 0);
        assert_eq!(archive.json_stream, "{}");
    }

    #[test]
    fn test_serialize_vector() {
        let mut archive = SerializedArchive::new();
        let vec = vec![1usize, 2usize, 3usize];
        archive.serialize_vector("numbers", &vec).unwrap();
        assert_eq!(archive.json_stream, "\"numbers\": [1, 2, 3]");
        assert_eq!(
            archive.data,
            vec![1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
