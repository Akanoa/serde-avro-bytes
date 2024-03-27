use std::any::Any;
use std::collections::{BTreeMap, HashMap};

use crate::avro_bytes::Pair;
use serde::ser::{Error, SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};

enum Map<'a> {
    HashMap(&'a HashMap<Vec<u8>, Vec<u8>>),
    BtreeMap(&'a BTreeMap<Vec<u8>, Vec<u8>>),
}

impl<'a> Map<'a> {
    fn get_pairs(&self) -> Vec<Pair<'a>> {
        let pairs = match self {
            Map::BtreeMap(map) => map.iter().fold(vec![], |mut acc, (k, v)| {
                acc.push(Pair { key: k, value: v });
                acc
            }),
            Map::HashMap(map) => map.iter().fold(vec![], |mut acc, (k, v)| {
                acc.push(Pair { key: k, value: v });
                acc
            }),
        };
        pairs
    }

    fn serialize_map<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pairs = self.get_pairs();
        let mut seq = serializer.serialize_seq(Some(pairs.len()))?;
        for pair in pairs {
            seq.serialize_element(&pair)?;
        }
        seq.end()
    }
}

#[allow(unused)]
pub fn serialize<S>(v: &dyn Any, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(bytes) = v.downcast_ref::<Vec<u8>>() {
        return serializer.serialize_bytes(bytes);
    }

    if let Some(bytes) = v.downcast_ref::<&[u8]>() {
        return serializer.serialize_bytes(bytes);
    }

    if let Some(bytes) = v.downcast_ref::<Option<Vec<u8>>>() {
        return serialize_option_bytes(bytes, serializer);
    }
    if let Some(bytes) = v.downcast_ref::<Vec<Vec<u8>>>() {
        return serialize_list_bytes(bytes, serializer);
    }

    if let Some(bytes) = v.downcast_ref::<Option<Vec<Vec<u8>>>>() {
        return serialize_option_list_bytes(bytes, serializer);
    }

    if let Some(map_bytes) = v.downcast_ref::<HashMap<Vec<u8>, Vec<u8>>>() {
        return Map::HashMap(map_bytes).serialize_map(serializer);
    }
    if let Some(maybe_map_bytes) = v.downcast_ref::<Option<HashMap<Vec<u8>, Vec<u8>>>>() {
        return serialize_option_map_bytes(
            &maybe_map_bytes
                .as_ref()
                .map(|hash_map| Map::HashMap(hash_map)),
            serializer,
        );
    }
    if let Some(btree_map) = v.downcast_ref::<BTreeMap<Vec<u8>, Vec<u8>>>() {
        return Map::BtreeMap(btree_map).serialize_map(serializer);
    }
    if let Some(maybe_map_bytes) = v.downcast_ref::<Option<BTreeMap<Vec<u8>, Vec<u8>>>>() {
        return serialize_option_map_bytes(
            &maybe_map_bytes
                .as_ref()
                .map(|btree_map| Map::BtreeMap(btree_map)),
            serializer,
        );
    }
    Err(S::Error::custom("Unsupported, supported types : Vec<u8>, Vec<Vec<u8>>, HashMap<Vec<u8>, Vec<u8>> and Option variation and BtreeMap"))
}

struct Bytes<'a>(&'a [u8]);

impl Serialize for Bytes<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.0)
    }
}

fn serialize_option_bytes<S>(v: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        None => serializer.serialize_none(),
        Some(bytes) => serializer.serialize_some(&Bytes(bytes)),
    }
}

fn serialize_list_bytes<S>(v: &Vec<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for x in v {
        seq.serialize_element(&Bytes(x))?
    }
    seq.end()
}

fn serialize_option_list_bytes<S>(
    v: &Option<Vec<Vec<u8>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        None => serializer.serialize_none(),
        Some(v) => {
            let list = v.iter().fold(vec![], |mut acc, x| {
                acc.push(Bytes(x));
                acc
            });
            serializer.serialize_some(&list)
        }
    }
}

fn serialize_option_map_bytes<S>(v: &Option<Map>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        None => serializer.serialize_none(),
        Some(map) => serializer.serialize_some(&map.get_pairs()),
    }
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
