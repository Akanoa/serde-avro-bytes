use core::fmt;
use std::collections::{BTreeMap, HashMap};

use bstr::BString;
use serde::{
    de::{Error, SeqAccess, Visitor},
    Deserializer,
};

#[allow(unused)]
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<BString, D::Error> {
    struct BStringVisitor;

    impl<'de> Visitor<'de> for BStringVisitor {
        type Value = BString;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a byte string")
        }

        #[inline]
        fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<BString, V::Error> {
            let capacity = seq.size_hint().unwrap_or_default();
            let mut bytes = Vec::with_capacity(capacity);
            while let Some(v) = seq.next_element()? {
                bytes.push(v);
            }
            Ok(BString::from(bytes))
        }

        #[inline]
        fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<BString, E> {
            Ok(BString::from(value))
        }

        #[inline]
        fn visit_byte_buf<E: Error>(self, value: Vec<u8>) -> Result<BString, E> {
            Ok(BString::from(value))
        }

        #[inline]
        fn visit_str<E: Error>(self, value: &str) -> Result<BString, E> {
            Ok(BString::from(value))
        }

        #[inline]
        fn visit_string<E: Error>(self, value: String) -> Result<BString, E> {
            Ok(BString::from(value))
        }
    }

    deserializer.deserialize_byte_buf(BStringVisitor)
}

#[allow(unused)]
pub fn deserialize_option<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<BString>, D::Error> {
    struct OptionBStringVisitor;

    impl<'de> Visitor<'de> for OptionBStringVisitor {
        type Value = Option<BString>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an optional byte string")
        }

        #[inline]
        fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            Ok(Some(deserialize(deserializer)?))
        }
    }

    deserializer.deserialize_option(OptionBStringVisitor)
}

#[allow(unused)]
pub fn deserialize_list<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<BString>, D::Error> {
    struct VecBStringVisitor;

    impl<'de> Visitor<'de> for VecBStringVisitor {
        type Value = Vec<BString>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a collection of byte strings")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            use crate::de::map::Bytes;

            let capacity = seq.size_hint().unwrap_or_default();
            let mut items = Vec::with_capacity(capacity);
            while let Some(bytes) = seq.next_element::<Bytes>()? {
                items.push(BString::from(bytes.0));
            }
            Ok(items)
        }
    }

    deserializer.deserialize_seq(VecBStringVisitor)
}

#[allow(unused)]
pub fn deserialize_option_list<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<BString>>, D::Error> {
    struct OptionVecBStringVisitor;

    impl<'de> Visitor<'de> for OptionVecBStringVisitor {
        type Value = Option<Vec<BString>>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "an optional collection of byte strings")
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            Ok(Some(deserialize_list(deserializer)?))
        }

        #[inline]
        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_option(OptionVecBStringVisitor)
}

#[allow(unused)]
pub fn deserialize_hashmap<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<HashMap<BString, BString>, D::Error> {
    struct HashMapBStringVisitor;

    impl<'de> Visitor<'de> for HashMapBStringVisitor {
        type Value = HashMap<BString, BString>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a map of byte strings")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            use crate::de::map::Pair;

            let capacity = seq.size_hint().unwrap_or_default();
            let mut items = HashMap::with_capacity(capacity);
            while let Some(Pair { key, value }) = seq.next_element::<Pair>()? {
                if let Some(duplicated) = items.insert(BString::new(key.0), BString::new(value.0)) {
                    return Err(serde::de::Error::custom(format!(
                        "unexpected duplicate key: `{duplicated}`"
                    )));
                }
            }
            Ok(items)
        }
    }

    deserializer.deserialize_seq(HashMapBStringVisitor)
}

#[allow(unused)]
pub fn deserialize_option_hashmap<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<HashMap<BString, BString>>, D::Error> {
    struct OptionHashMapBStringVisitor;

    impl<'de> Visitor<'de> for OptionHashMapBStringVisitor {
        type Value = Option<HashMap<BString, BString>>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "an optional map of byte strings")
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            Ok(Some(deserialize_hashmap(deserializer)?))
        }

        #[inline]
        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_option(OptionHashMapBStringVisitor)
}

#[allow(unused)]
pub fn deserialize_btreemap<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<BTreeMap<BString, BString>, D::Error> {
    struct BTreeMapBStringVisitor;

    impl<'de> Visitor<'de> for BTreeMapBStringVisitor {
        type Value = BTreeMap<BString, BString>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a map of byte strings")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            use crate::de::map::Pair;

            let mut items = BTreeMap::new();
            while let Some(Pair { key, value }) = seq.next_element::<Pair>()? {
                if let Some(duplicated) = items.insert(BString::new(key.0), BString::new(value.0)) {
                    return Err(serde::de::Error::custom(format!(
                        "unexpected duplicate key: `{duplicated}`"
                    )));
                }
            }
            Ok(items)
        }
    }

    deserializer.deserialize_seq(BTreeMapBStringVisitor)
}

#[allow(unused)]
pub fn deserialize_option_btreemap<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<BTreeMap<BString, BString>>, D::Error> {
    struct OptionBTreeMapBStringVisitor;

    impl<'de> Visitor<'de> for OptionBTreeMapBStringVisitor {
        type Value = Option<BTreeMap<BString, BString>>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "an optional map of byte strings")
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            Ok(Some(deserialize_btreemap(deserializer)?))
        }

        #[inline]
        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_option(OptionBTreeMapBStringVisitor)
}
