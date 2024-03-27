use crate::avro_bytes::ser::bytes::Bytes;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub(crate) struct Pair<'a> {
    pub(crate) key: &'a Vec<u8>,
    pub(crate) value: &'a Vec<u8>,
}

impl Serialize for Pair<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut pair = serializer.serialize_struct("Pair", 2)?;
        pair.serialize_field("key", &Bytes(self.key))?;
        pair.serialize_field("value", &Bytes(self.value))?;
        pair.end()
    }
}
