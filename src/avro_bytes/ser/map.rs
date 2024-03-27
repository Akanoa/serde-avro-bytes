use crate::avro_bytes::ser::pair::Pair;
use serde::ser::SerializeSeq;
use serde::Serializer;
use std::collections::{BTreeMap, HashMap};

pub fn serialize_hashmap<S>(v: &HashMap<Vec<u8>, Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let pairs: PairWrapper = v.into();
    let pairs = pairs.0;
    let mut seq = serializer.serialize_seq(Some(pairs.len()))?;
    for pair in pairs {
        seq.serialize_element(&pair)?;
    }
    seq.end()
}

pub fn serialize_option_hashmap<S>(
    v: &Option<HashMap<Vec<u8>, Vec<u8>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        None => serializer.serialize_none(),
        Some(map) => {
            let pairs: PairWrapper = map.into();
            serializer.serialize_some(&pairs.0)
        }
    }
}

pub fn serialize_btreemap<S>(
    v: &BTreeMap<Vec<u8>, Vec<u8>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let pairs: PairWrapper = v.into();
    let pairs = pairs.0;
    let mut seq = serializer.serialize_seq(Some(pairs.len()))?;
    for pair in pairs {
        seq.serialize_element(&pair)?;
    }
    seq.end()
}

pub fn serialize_option_btreemap<S>(
    v: &Option<BTreeMap<Vec<u8>, Vec<u8>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        None => serializer.serialize_none(),
        Some(map) => {
            let pairs: PairWrapper = map.into();
            serializer.serialize_some(&pairs.0)
        }
    }
}

struct PairWrapper<'a>(Vec<Pair<'a>>);

impl<'a> From<&'a HashMap<Vec<u8>, Vec<u8>>> for PairWrapper<'a> {
    fn from(value: &'a HashMap<Vec<u8>, Vec<u8>>) -> Self {
        let pairs = PairWrapper(
            value
                .iter()
                .map(|(key, value)| Pair { key, value })
                .collect(),
        );

        pairs
    }
}

impl<'a> From<&'a BTreeMap<Vec<u8>, Vec<u8>>> for PairWrapper<'a> {
    fn from(value: &'a BTreeMap<Vec<u8>, Vec<u8>>) -> Self {
        let pairs = PairWrapper(
            value
                .iter()
                .map(|(key, value)| Pair { key, value })
                .collect(),
        );

        pairs
    }
}
