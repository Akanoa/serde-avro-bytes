use serde::{ser::SerializeSeq, Serializer};

use crate::avro_bytes::ser::bytes::Bytes;

pub fn serialize_list_bytes<S: Serializer, T: AsRef<[u8]>>(
    v: &[T],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for x in v {
        seq.serialize_element(&Bytes(x.as_ref()))?
    }
    seq.end()
}

pub fn serialize_option_list_bytes<S: Serializer, T: AsRef<[u8]>>(
    v: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match v {
        None => serializer.serialize_none(),
        Some(v) => {
            serializer.serialize_some(&v.iter().map(AsRef::as_ref).map(Bytes).collect::<Vec<_>>())
        }
    }
}
