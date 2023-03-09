use crate::transaction::Variant;
use crate::{VarInt, VarIntEncoding};

#[derive(Debug, Clone, Default)]
pub struct SerializedArchive {
    pub data: Vec<u8>,
    pub json_stream: String,
    pub depth: usize,
    pub object_begin: bool,
}

impl SerializedArchive {
    /// Creates a new empty serialized blob object
    pub fn new() -> Self {
        SerializedArchive {
            ..Default::default()
        }
    }

    /// Returns the serialized data as a byte array
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    /// Returns the serialized data as a vector of bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Serializes a vector of containing items of type T which implement the
    /// DoSerialize trait
    pub fn serialize_vector<T>(&mut self, tag: &str, vec: &Vec<T>) -> Result<(), anyhow::Error>
    where
        T: DoSerialize,
    {
        self.add_tag(tag);
        self.begin_array_with_size(vec.len());
        for (i, item) in vec.iter().enumerate() {
            item.do_serialize(self)?;
            if i < vec.len() - 1 {
                self.delimit_array();
            }
        }
        self.end_array();
        Ok(())
    }

    /// Serializes a vector of containing items of type T, where T can be a
    /// member of an enum V The T type should implement the DoSerialize
    /// trait
    pub fn serialize_vector_variant<T, V>(
        &mut self,
        tag: &str,
        vec: &Vec<T>,
        variant: V,
    ) -> Result<(), anyhow::Error>
    where
        T: DoSerialize,
        V: Variant,
    {
        self.add_tag(tag);
        self.begin_array_with_size(vec.len());
        // Tag the variant type
        let variant_tag = variant.variant_tag();
        self.data.push(variant_tag);
        for (i, item) in vec.iter().enumerate() {
            item.do_serialize(self)?;
            if i < vec.len() - 1 {
                self.delimit_array();
            }
        }
        self.end_array();
        Ok(())
    }

    /// Serializes a field, implementation of  the FIELD(f) macro from Monero's
    /// codebase
    pub fn serialize_field<T>(
        &mut self,
        field_name: &str,
        field_value: &T,
    ) -> Result<(), anyhow::Error>
    where
        T: DoSerialize,
    {
        self.add_tag(field_name);
        field_value.do_serialize(self)?;
        Ok(())
    }

    /// Serializes bytes representing a key of some sort, the data from the key
    /// as added directly to the serialized data. The json stream is updated
    /// with the hex encoded version of the key encapsulated in quotes
    pub fn serialize_key(&mut self, key: &[u8]) -> Result<(), anyhow::Error> {
        self.data.extend_from_slice(&key);
        self.json_stream.push_str("\"");
        self.json_stream.push_str(&hex::encode(key.to_vec()));
        self.json_stream.push_str("\"");
        Ok(())
    }

    /// Incorporates bytes directly into the serialized archive
    /// Encodes the data to as hex and adds it to the json stream
    pub fn serialize_directly(&mut self, blob: &[u8]) -> Result<(), anyhow::Error> {
        self.data.extend_from_slice(blob);
        self.json_stream.push_str(&hex::encode(blob.to_vec()));
        Ok(())
    }

    /// Adds the tag to the serialized result, assumes no indent is needed
    pub fn add_tag(&mut self, tag: &str) -> () {
        if !self.object_begin {
            self.json_stream.push_str(", ");
        } else {
            self.object_begin = false;
        }
        self.json_stream.push_str("\"");
        self.json_stream.push_str(tag);
        self.json_stream.push_str("\": ");
    }

    pub fn begin_object(&mut self) -> () {
        self.json_stream.push_str("{");
        self.depth += 1;
        self.object_begin = true;
    }

    /// Currently assuming that make_intent is not set
    pub fn end_object(&mut self) -> () {
        self.json_stream.push_str("}");
        self.depth -= 1;
        self.object_begin = false;
    }

    /// Begins an array in the json stream and also adds the size of the array
    /// to the data stream
    pub fn begin_array_with_size(&mut self, size: usize) -> Result<(), anyhow::Error> {
        self.json_stream.push_str("[");
        self.depth += 1;
        size.do_serialize(self)?;
        Ok(())
    }

    /// Begins an array in the json stream, does not affect data stream
    pub fn begin_array(&mut self) -> () {
        self.json_stream.push_str("[");
        self.depth += 1;
    }

    /// Adds a delimiter for an array into the json stream
    pub fn delimit_array(&mut self) -> () {
        self.json_stream.push_str(", ");
    }

    /// Ends an array in the json stream
    pub fn end_array(&mut self) -> () {
        self.json_stream.push_str("]");
        self.depth -= 1;
    }

    /// Serialize an object, does not include the tagged name for the object
    /// The object can be anything that implements the DoSerialize trait
    pub fn serialize_object(&mut self, object: &dyn DoSerialize) -> Result<(), anyhow::Error> {
        self.begin_object();
        object.do_serialize(self)?;
        self.end_object();
        Ok(())
    }
}

pub trait DoSerialize {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error>;
}

impl DoSerialize for usize {
    fn do_serialize(&self, serialized: &mut SerializedArchive) -> Result<(), anyhow::Error> {
        serialized
            .data
            .extend(VarInt(*self as u64).encode_to_bytes());
        Ok(())
    }
}
