use std::collections::{BTreeMap, HashMap};

use crate::avro_bytes::ser::pair::Pair;
use serde::{ser::SerializeSeq, Serializer};

pub(crate) struct PairWrapper<'a>(pub(crate) Vec<Pair<'a>>);

impl<'a, K: AsRef<[u8]>, V: AsRef<[u8]>> FromIterator<(&'a K, &'a V)> for PairWrapper<'a> {
    fn from_iter<T: IntoIterator<Item = (&'a K, &'a V)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(key, value)| Pair {
                    key: key.as_ref(),
                    value: value.as_ref(),
                })
                .collect(),
        )
    }
}

pub fn serialize_hashmap<S: Serializer, K: AsRef<[u8]>, V: AsRef<[u8]>>(
    v: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let pairs = v.iter().collect::<PairWrapper>().0;
    let mut seq = serializer.serialize_seq(Some(pairs.len()))?;
    for pair in pairs {
        seq.serialize_element(&pair)?;
    }
    seq.end()
}

pub fn serialize_option_hashmap<S: Serializer, K: AsRef<[u8]>, V: AsRef<[u8]>>(
    v: &Option<HashMap<K, V>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match v {
        None => serializer.serialize_none(),
        Some(map) => serializer.serialize_some(&map.iter().collect::<PairWrapper>().0),
    }
}

pub fn serialize_btreemap<S: Serializer, K: AsRef<[u8]>, V: AsRef<[u8]>>(
    v: &BTreeMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let pairs = v.iter().collect::<PairWrapper>().0;
    let mut seq = serializer.serialize_seq(Some(pairs.len()))?;
    for pair in pairs {
        seq.serialize_element(&pair)?;
    }
    seq.end()
}

pub fn serialize_option_btreemap<S: Serializer, K: AsRef<[u8]>, V: AsRef<[u8]>>(
    v: &Option<BTreeMap<K, V>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match v {
        None => serializer.serialize_none(),
        Some(map) => serializer.serialize_some(&map.iter().collect::<PairWrapper>().0),
    }
}
