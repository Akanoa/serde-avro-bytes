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
pub use de::map::deserialize_btreemap;
#[allow(unused)]
pub use de::map::deserialize_hashmap;
#[allow(unused)]
pub use de::map::deserialize_option_btreemap;
#[allow(unused)]
pub use ser::serialize;

pub mod bytes {
    use crate::avro_bytes::de;
    use crate::avro_bytes::ser;
    #[allow(unused)]
    pub use de::bytes::deserialize_bytes as deserialize;
    #[allow(unused)]
    pub use ser::serialize_bytes as serialize;

    pub mod option {
        #[allow(unused)]
        pub use crate::avro_bytes::de::bytes::deserialize_option_bytes as deserialize;
        #[allow(unused)]
        pub use crate::avro_bytes::ser::serialize_option_bytes as serialize;
    }
}

pub mod hashmap {
    use crate::avro_bytes::de;
    use crate::avro_bytes::ser;
    #[allow(unused)]
    pub use de::map::deserialize_hashmap as deserialize;
    #[allow(unused)]
    pub use ser::serialize_hashmap as serialize;

    pub mod option {
        #[allow(unused)]
        pub use crate::avro_bytes::de::map::deserialize_option_hashmap as deserialize;
        #[allow(unused)]
        pub use crate::avro_bytes::ser::serialize_option_hashmap as serialize;
    }
}

pub mod btreemap {
    use crate::avro_bytes::de;
    use crate::avro_bytes::ser;
    #[allow(unused)]
    pub use de::map::deserialize_btreemap as deserialize;
    #[allow(unused)]
    pub use ser::serialize_btreemap as serialize;

    pub mod option {
        #[allow(unused)]
        pub use crate::avro_bytes::de::map::deserialize_option_btreemap as deserialize;
        #[allow(unused)]
        pub use crate::avro_bytes::ser::serialize_option_btreemap as serialize;
    }
}

pub mod list {
    use crate::avro_bytes::de;
    use crate::avro_bytes::ser;
    #[allow(unused)]
    pub use de::list::deserialize_list as deserialize;
    #[allow(unused)]
    pub use ser::serialize_list_bytes as serialize;

    pub mod option {
        #[allow(unused)]
        pub use crate::avro_bytes::de::list::deserialize_option_list as deserialize;
        #[allow(unused)]
        pub use crate::avro_bytes::ser::serialize_option_list_bytes as serialize;
    }
}
