use serde::{Serialize, Serializer};

pub fn serialize_bytes<S>(v: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bytes(&v)
}

pub fn serialize_option_bytes<S>(v: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        None => serializer.serialize_none(),
        Some(bytes) => serializer.serialize_some(&Bytes(bytes)),
    }
}

pub struct Bytes<'a>(pub(crate) &'a [u8]);

impl Serialize for Bytes<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.0)
    }
}
