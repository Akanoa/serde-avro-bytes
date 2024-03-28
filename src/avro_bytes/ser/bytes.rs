use serde::{Serialize, Serializer};

pub struct Bytes<'a>(pub(crate) &'a [u8]);

impl Serialize for Bytes<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.0)
    }
}

pub fn serialize_bytes<S: Serializer>(
    v: impl AsRef<[u8]>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_bytes(v.as_ref())
}

pub fn serialize_option_bytes<S: Serializer, T: AsRef<[u8]>>(
    v: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match v {
        None => serializer.serialize_none(),
        Some(bytes) => serializer.serialize_some(&Bytes(bytes.as_ref())),
    }
}
