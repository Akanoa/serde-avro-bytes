# Serde Avro Bytes

[Avro](https://avro.apache.org/docs/1.11.1/specification/) is a binary encoding format which provides a "bytes" type optimized
to store `&[u8]` data like.

Unfortunately the [apache_avro](https://docs.rs/apache-avro/latest/apache_avro/) encodes `Vec<u8>` as an array of integers
thus the encoded data are twice bigger than using the `bytes`.

```rust
#[derive(Serialize)]
struct Record {
    data: Vec<u8>
}

fn playground() {
    let record = Record {
        data: vec![1,2]
    };
    // data field
    // => encoded as in int array : [4,1,1,1,2,0]
    // => encoded as bytes : [4,1,2]
}   
```

This crate provided a set of module to handle idiomatic Rust types and
encode its component as "bytes".

```rust
#[derive(Serialize, Deserialize)]
struct Record {
    #[serde(with = "serde_avro_bytes::bytes")]
    key: Vec<u8>,
    #[serde(with = "serde_avro_bytes::bytes::option")]
    key2: Option<Vec<u8>>,
    #[serde(with = "serde_avro_bytes::hashmap")]
    key3: HashMap<Vec<u8>, Vec<u8>>,
    #[serde(with = "serde_avro_bytes::btreemap::option")]
    key4: Option<BTreeMap<Vec<u8>, Vec<u8>>>,
    #[serde(with = "serde_avro_bytes::list")]
    key5: Vec<Vec<u8>>,
    #[serde(with = "serde_avro_bytes::list::option")]
    key6: Option<Vec<Vec<u8>>>,
}
```

## Features

* `bstr`: adds support for working with `BString`s which are convenient wrappers for partially valid UTF-8 bytes sequences provided by the [`bst`](https://github.com/BurntSushi/bstr) crate. See [`examples/bstr.rs`](./examples/bstr.rs).