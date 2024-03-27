use serde::de::{Error, SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt::Formatter;

#[allow(unused)]
pub fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = BytesVisitor;
    deserializer.deserialize_bytes(visitor)
}

#[allow(unused)]
pub fn deserialize_option_bytes<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = OptionBytesVisitor;
    deserializer.deserialize_option(visitor)
}

pub struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unable to decode bytes")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_vec())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut data = vec![];
        while let Some(x) = seq.next_element()? {
            data.push(x);
        }
        Ok(data)
    }
}

struct OptionBytesVisitor;

impl<'de> Visitor<'de> for OptionBytesVisitor {
    type Value = Option<Vec<u8>>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unable to decode option of bytes")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(deserialize_bytes(deserializer)?))
    }
}
