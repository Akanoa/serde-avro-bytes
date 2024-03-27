mod de;
mod ser;

pub mod bytes {
    pub use super::*;

    #[allow(unused)]
    pub use de::bytes::deserialize_bytes as deserialize;
    #[allow(unused)]
    pub use ser::bytes::serialize_bytes as serialize;

    pub mod option {
        pub use super::*;
        #[allow(unused)]
        pub use de::bytes::deserialize_option_bytes as deserialize;
        #[allow(unused)]
        pub use ser::bytes::serialize_option_bytes as serialize;
    }
}

pub mod hashmap {
    pub use super::*;
    #[allow(unused)]
    pub use de::map::deserialize_hashmap as deserialize;
    #[allow(unused)]
    pub use ser::map::serialize_hashmap as serialize;

    pub mod option {
        pub use super::*;
        #[allow(unused)]
        pub use de::map::deserialize_option_hashmap as deserialize;
        #[allow(unused)]
        pub use ser::map::serialize_option_hashmap as serialize;
    }
}

pub mod btreemap {
    pub use super::*;
    #[allow(unused)]
    pub use de::map::deserialize_btreemap as deserialize;
    #[allow(unused)]
    pub use ser::map::serialize_btreemap as serialize;

    pub mod option {
        pub use super::*;
        #[allow(unused)]
        pub use de::map::deserialize_option_btreemap as deserialize;
        #[allow(unused)]
        pub use ser::map::serialize_option_btreemap as serialize;
    }
}

pub mod list {
    pub use super::*;
    #[allow(unused)]
    pub use de::list::deserialize_list as deserialize;
    #[allow(unused)]
    pub use ser::list::serialize_list_bytes as serialize;

    pub mod option {
        pub use super::*;
        #[allow(unused)]
        pub use de::list::deserialize_option_list as deserialize;
        #[allow(unused)]
        pub use ser::list::serialize_option_list_bytes as serialize;
    }
}
