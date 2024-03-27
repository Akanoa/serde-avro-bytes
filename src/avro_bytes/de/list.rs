use crate::avro_bytes::de::map::Bytes;
use serde::de::{Error, SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt::Formatter;

#[allow(unused)]
pub fn deserialize_list<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = ListVisitor;
    deserializer.deserialize_seq(visitor)
}

struct ListVisitor;

impl<'de> Visitor<'de> for ListVisitor {
    type Value = Vec<Vec<u8>>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unable to decode list of bytes")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut lists = vec![];
        while let Some(list) = seq.next_element::<Bytes>()? {
            lists.push(list.0);
        }
        Ok(lists)
    }
}

#[allow(unused)]
pub fn deserialize_option_list<'de, D>(deserializer: D) -> Result<Option<Vec<Vec<u8>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = OptionListVisitor;
    deserializer.deserialize_option(visitor)
}

struct OptionListVisitor;

impl<'de> Visitor<'de> for OptionListVisitor {
    type Value = Option<Vec<Vec<u8>>>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unable to decode option of list of bytes")
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
        Ok(Some(deserialize_list(deserializer)?))
    }
}
