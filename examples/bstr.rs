use std::{
    collections::{BTreeMap, HashMap},
    io::Cursor,
};

use apache_avro::Schema;
use bstr::BString;
use serde::{Deserialize, Serialize};

static RECORD: &str = r#"{
    "name": "Record",
    "type": "record",
    "fields": [
        {
            "name": "key",
            "type": "bytes"
        },
        {
            "name": "option",
            "type": [
                "null",
                "bytes"
            ]
        },
        {
            "name": "list",
            "type": "array",
            "items": "bytes"
        },
        {
            "name": "option_list",
            "type": [
                "null",
                {
                    "type": "array",
                    "items": "bytes"
                }
            ]
        },
        {
            "name": "hashmap",
            "type": "array",
            "items": {
                "name": "Pair",
                "type": "record",
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
        },
        {
            "name": "option_hashmap",
            "type": [
                "null",
                {
                    "type": "array",
                    "items": "Pair"
                }
            ]
        },
        {
            "name": "btreemap",
            "type": "array",
            "items": "Pair"
        },
        {
            "name": "option_btreemap",
            "type": [
                "null",
                {
                    "type": "array",
                    "items": "Pair"
                }
            ]
        }
    ]
}"#;

#[derive(Serialize, Default, Deserialize, PartialEq, Debug)]
pub struct Record {
    #[serde(with = "serde_avro_bytes::extra::bstr")]
    key: BString,
    #[serde(with = "serde_avro_bytes::extra::bstr::option")]
    option: Option<BString>,

    #[serde(with = "serde_avro_bytes::extra::bstr::list")]
    list: Vec<BString>,
    #[serde(with = "serde_avro_bytes::extra::bstr::list::option")]
    option_list: Option<Vec<BString>>,

    #[serde(with = "serde_avro_bytes::extra::bstr::hashmap")]
    hashmap: HashMap<BString, BString>,
    #[serde(with = "serde_avro_bytes::extra::bstr::hashmap::option")]
    option_hashmap: Option<HashMap<BString, BString>>,

    #[serde(with = "serde_avro_bytes::extra::bstr::btreemap")]
    btreemap: BTreeMap<BString, BString>,
    #[serde(with = "serde_avro_bytes::extra::bstr::btreemap::option")]
    option_btreemap: Option<BTreeMap<BString, BString>>,
}

fn avro_encode_decode(schema: &Schema, record: &Record) -> Record {
    let value = apache_avro::to_value(&record).expect("avro value");
    let encoded = apache_avro::to_avro_datum(&schema, value).expect("encoded value");
    let mut reader = Cursor::new(encoded);
    let value =
        apache_avro::from_avro_datum(&schema, &mut reader, Some(&schema)).expect("decoded record");
    apache_avro::from_value::<Record>(&value).expect("record")
}

static PARTIAL_UTF8: &[u8] = b"hello \xF4\x8F\xBF";

fn main() {
    let schema = apache_avro::Schema::parse_str(&RECORD).expect("valid avro schema");

    let record = Record::default();
    assert_eq!(record, avro_encode_decode(&schema, &record));

    let record = Record {
        key: BString::from(PARTIAL_UTF8),
        option: Some(BString::from(PARTIAL_UTF8)),
        list: vec![BString::from(PARTIAL_UTF8), BString::from(PARTIAL_UTF8)],
        option_list: Some(vec![
            BString::from(PARTIAL_UTF8),
            BString::from(PARTIAL_UTF8),
        ]),
        hashmap: HashMap::from([
            (BString::from(PARTIAL_UTF8), BString::from(PARTIAL_UTF8)),
            (BString::from(PARTIAL_UTF8), BString::from(PARTIAL_UTF8)),
        ]),
        option_hashmap: Some(HashMap::from([
            (BString::from(PARTIAL_UTF8), BString::from(PARTIAL_UTF8)),
            (BString::from(PARTIAL_UTF8), BString::from(PARTIAL_UTF8)),
        ])),
        btreemap: BTreeMap::from([
            (BString::from(PARTIAL_UTF8), BString::from(PARTIAL_UTF8)),
            (BString::from(PARTIAL_UTF8), BString::from(PARTIAL_UTF8)),
        ]),
        option_btreemap: Some(BTreeMap::from([
            (BString::from(PARTIAL_UTF8), BString::from(PARTIAL_UTF8)),
            (BString::from(PARTIAL_UTF8), BString::from(PARTIAL_UTF8)),
        ])),
    };
    assert_eq!(record, avro_encode_decode(&schema, &record));
}
