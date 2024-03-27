use serde::Serializer;
use serde::ser::SerializeSeq;
use crate::avro_bytes::ser::bytes::Bytes;

pub fn serialize_list_bytes<S>(v: &Vec<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for x in v {
        seq.serialize_element(&Bytes(x))?
    }
    seq.end()
}

pub fn serialize_option_list_bytes<S>(
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
