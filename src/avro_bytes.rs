mod de;
mod ser;

#[derive(Debug)]
struct Pair<'a> {
    key: &'a Vec<u8>,
    value: &'a Vec<u8>,
}

#[allow(unused)]
pub use de::bytes::deserialize_bytes;
#[allow(unused)]
pub use de::bytes::deserialize_option_bytes;
#[allow(unused)]
pub use de::list::deserialize_list;
#[allow(unused)]
pub use de::list::deserialize_option_list;
#[allow(unused)]
pub use de::map::deserialize_btree_map;
#[allow(unused)]
pub use de::map::deserialize_hash_map;
#[allow(unused)]
pub use de::map::deserialize_option_btree_map;
#[allow(unused)]
pub use ser::serialize;
