use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;

use serde::{Deserialize, Serialize};

static SCHEMA: &str = r#"
{
  "name": "Record",
  "type": "record",
  "fields": [
    {
      "type": "bytes",
      "name": "key"
    },
    {
      "type": [
        "null",
        "bytes"
      ],
      "name": "key2"
    },
    {
      "type": "array",
      "items": {
        "type": "record",
        "name": "Pair1",
        "fields": [
          {
            "name": "key",
            "type": "bytes"
          },
          {
            "name": "value",
            "type": "bytes"
          }
        ]
      },
      "name": "key3"
    },
    {
      "type": [
        "null",
        {
          "type": "array",
          "items": {
            "type": "record",
            "name": "Pair",
            "fields": [
              {
                "name": "key",
                "type": "bytes"
              },
              {
                "name": "value",
                "type": "bytes"
              }
            ]
          }
        }
      ],
      "name": "key4"
    },
    {
      "type": "array",
      "items": "bytes",
      "name": "key5"
    },
    {
      "type": [
        "null",
        {
          "type": "array",
          "items": "bytes"
        }
      ],
      "name": "key6"
    }
  ]
}
"#;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Record {
    #[serde(with = "avro_bytes::bytes")]
    key: Vec<u8>,
    #[serde(with = "avro_bytes::bytes::option")]
    key2: Option<Vec<u8>>,
    #[serde(with = "avro_bytes::hashmap")]
    key3: HashMap<Vec<u8>, Vec<u8>>,
    #[serde(with = "avro_bytes::btreemap::option")]
    key4: Option<BTreeMap<Vec<u8>, Vec<u8>>>,
    #[serde(with = "avro_bytes::list")]
    key5: Vec<Vec<u8>>,
    #[serde(with = "avro_bytes::list::option")]
    key6: Option<Vec<Vec<u8>>>,
}

fn main() {
    env_logger::init();

    let map = BTreeMap::from([
        (vec![1, 5, 6], vec![7, 8, 9]),
        (vec![10, 11, 12], vec![13, 1, 48]),
    ]);

    let map2 = HashMap::from([
        (vec![1, 5, 6], vec![7, 8, 9]),
        (vec![10, 11, 13], vec![13, 1, 48]),
    ]);

    let v = vec![vec![1, 5, 6, 7], vec![4, 8, 2, 6]];

    let record = Record {
        key: vec![0, 1, 3],
        key2: Some(vec![4, 5, 6]),
        key3: map2,
        key4: Some(map),
        key5: v.clone(),
        key6: Some(v),
    };

    // encoding
    let schema = apache_avro::Schema::parse_str(&SCHEMA).unwrap();
    let avro_value = apache_avro::to_value(&record).unwrap();
    let encoded = apache_avro::to_avro_datum(&schema, avro_value).unwrap();

    // decoding
    let mut reader = Cursor::new(encoded);
    let decoding_data = apache_avro::from_avro_datum(&schema, &mut reader, Some(&schema));
    let result = apache_avro::from_value::<Record>(&decoding_data.unwrap()).unwrap();

    assert_eq!(result, record);
}

mod avro_bytes;
