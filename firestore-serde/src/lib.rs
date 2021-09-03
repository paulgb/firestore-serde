pub use crate::deserialize::ValueDeserializer;
use crate::firestore::{value::ValueType, Document, MapValue, Value};
use crate::serialize::SerializationError;
pub use crate::serialize::ValueSerializer;
use serde::de::DeserializeOwned;
use serde::Serialize;

mod deserialize;
pub mod firestore;
mod serialize;

pub const TYPE: &str = "type";
pub const VALUE: &str = "value";
pub const VALUES: &str = "values";
pub const DATE_MAGIC: &str = "$TimestampValue";

#[cfg(all(feature = "google-firestore-v1", feature = "google-firestore-v1beta1"))]
compile_error!("If you enable the google-firestore-v1beta1 crate feature, you must disable the default feature google-firestore-v1 to avoid a conflict.");

pub fn to_grpc_value<T>(value: &T) -> crate::serialize::Result<Value>
where
    T: Serialize,
{
    let result = value.serialize(ValueSerializer)?;
    Ok(result)
}

pub fn from_grpc_value<T>(value: &Value) -> crate::deserialize::Result<T>
where
    T: DeserializeOwned,
{
    T::deserialize(&mut ValueDeserializer(value))
}

pub fn to_document<T>(value: &T) -> crate::serialize::Result<Document>
where
    T: Serialize,
{
    if let Value {
        value_type: Some(ValueType::MapValue(MapValue { fields })),
    } = to_grpc_value(value)?
    {
        Ok(Document {
            fields,
            ..Document::default()
        })
    } else {
        Err(SerializationError::NotAMap)
    }
}

pub fn from_document<T>(document: Document) -> crate::deserialize::Result<T>
where
    T: DeserializeOwned,
{
    let fields = document.fields;
    let value = Value {
        value_type: Some(ValueType::MapValue(MapValue { fields })),
    };

    from_grpc_value(&value)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::firestore::ArrayValue;
    use crate::serialize::SerializationError;
    use serde::Deserialize;
    use serde_bytes::{ByteBuf, Bytes};
    use std::{collections::HashMap, convert::TryFrom, fmt::Display};

    #[test]
    fn test_serialize_string() {
        let result = to_grpc_value(&"blah").unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::StringValue("blah".to_string()))
            },
            result
        );

        assert_eq!(
            "blah".to_string(),
            from_grpc_value::<String>(&result).unwrap()
        )
    }

    #[test]
    fn test_serialize_bytes() {
        // Per https://serde.rs/impl-serialize.html#other-special-cases,
        // serialize_bytes is not used directly. We could create a wrapper
        // type that uses it, though, so we've implemented it.
        let v = Bytes::new(b"blah");
        let result = to_grpc_value(&v).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::BytesValue(v.to_vec()))
            },
            result
        );

        assert_eq!(*v, from_grpc_value::<ByteBuf>(&result).unwrap());

        // Without the serde_bytes crate, we can still deserialize bytes
        // as a Vec<u8>.
        assert_eq!(
            v.iter().cloned().collect::<Vec<u8>>(),
            from_grpc_value::<Vec<u8>>(&result).unwrap()
        )
    }

    #[test]
    fn test_serialize_char() {
        let result = to_grpc_value(&'a').unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::StringValue("a".to_string()))
            },
            result
        );

        assert_eq!('a', from_grpc_value::<char>(&result).unwrap())
    }

    #[test]
    fn test_serialize_option() {
        let p: Option<u32> = None;
        let result = to_grpc_value(&p).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::NullValue(0))
            },
            result
        );

        assert_eq! {
            p,
            from_grpc_value::<Option<u32>>(&result).unwrap()
        }

        let result = to_grpc_value(&Some(100)).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::IntegerValue(100))
            },
            result
        );

        assert_eq! {
            Some(100),
            from_grpc_value::<Option<u32>>(&result).unwrap()
        }
    }

    fn test_serialize_int_type<'a, T>(num: T)
    where
        i64: TryFrom<T>,
        T: Serialize + Display + Copy + PartialEq + std::fmt::Debug + DeserializeOwned,
    {
        let result = to_grpc_value(&num).unwrap();

        let exp_val = if let Ok(val) = i64::try_from(num) {
            val
        } else {
            // Should only occur if we convert a u64 that is
            // greater than the i64 range.
            panic!("Could not convert {} to i64", num);
        };

        assert_eq!(
            Value {
                value_type: Some(ValueType::IntegerValue(exp_val))
            },
            result
        );

        assert_eq!(num, from_grpc_value(&result).unwrap());
    }

    #[test]
    fn test_serialize_int() {
        test_serialize_int_type(4u8);
        test_serialize_int_type(4u16);
        test_serialize_int_type(4u32);
        test_serialize_int_type(4u64);

        test_serialize_int_type(4i8);
        test_serialize_int_type(4i16);
        test_serialize_int_type(4i32);
        test_serialize_int_type(4i64);

        assert_eq!(
            SerializationError::OutsideIntRange(u64::MAX),
            to_grpc_value(&u64::MAX).unwrap_err()
        );
    }

    #[test]
    fn test_serialize_float() {
        let result = to_grpc_value(&99.9f32).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::DoubleValue(f64::from(99.9f32)))
            },
            result
        );

        assert_eq!(99.9f32, from_grpc_value(&result).unwrap());

        let result = to_grpc_value(&99.9f64).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::DoubleValue(99.9f64))
            },
            result
        );

        assert_eq!(99.9f64, from_grpc_value(&result).unwrap());
    }

    #[test]
    fn test_serialize_bool() {
        let result_true = to_grpc_value(&true).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::BooleanValue(true))
            },
            result_true
        );

        assert_eq!(true, from_grpc_value(&result_true).unwrap());

        let result_false = to_grpc_value(&false).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::BooleanValue(false))
            },
            result_false
        );

        assert_eq!(false, from_grpc_value(&result_false).unwrap());
    }

    #[derive(Serialize)]
    struct JustAUnitStruct;

    #[test]
    fn test_unrepresentable() {
        assert_eq!(
            SerializationError::Unrepresentable("()".to_string()),
            to_grpc_value(&()).unwrap_err()
        );

        assert_eq!(
            SerializationError::Unrepresentable("unit_struct".to_string()),
            to_grpc_value(&JustAUnitStruct).unwrap_err()
        );
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum JustAnEnum {
        TagUnitVariant,
        ANewtypeVariant(u32),
        ATupleVariant(u8, bool),
        ARecordVariant { an_int: u32, a_bool: bool },
    }

    #[test]
    fn test_serialize_enum_record() {
        let v = JustAnEnum::ARecordVariant {
            an_int: 4,
            a_bool: false,
        };
        let result = to_grpc_value(&v).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: vec![
                        (
                            TYPE.to_string(),
                            Value {
                                value_type: Some(ValueType::StringValue(
                                    "ARecordVariant".to_string()
                                ))
                            }
                        ),
                        (
                            VALUES.to_string(),
                            Value {
                                value_type: Some(ValueType::MapValue(MapValue {
                                    fields: vec![
                                        (
                                            "an_int".to_string(),
                                            Value {
                                                value_type: Some(ValueType::IntegerValue(4))
                                            }
                                        ),
                                        (
                                            "a_bool".to_string(),
                                            Value {
                                                value_type: Some(ValueType::BooleanValue(false))
                                            }
                                        ),
                                    ]
                                    .into_iter()
                                    .collect()
                                }))
                            }
                        )
                    ]
                    .into_iter()
                    .collect()
                }))
            },
            result
        );

        assert_eq!(v, from_grpc_value(&result).unwrap());
    }

    #[test]
    fn test_serialize_enum_unit() {
        let result = to_grpc_value(&JustAnEnum::TagUnitVariant).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::StringValue("TagUnitVariant".to_string()))
            },
            result
        );

        assert_eq!(
            JustAnEnum::TagUnitVariant,
            from_grpc_value(&result).unwrap()
        );
    }

    #[test]
    fn test_serialize_enum_newtype() {
        let v = JustAnEnum::ANewtypeVariant(55);
        let result = to_grpc_value(&v).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: vec![
                        (
                            "type".to_string(),
                            Value {
                                value_type: Some(ValueType::StringValue(
                                    "ANewtypeVariant".to_string()
                                ))
                            }
                        ),
                        (
                            "value".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(55))
                            }
                        ),
                    ]
                    .into_iter()
                    .collect()
                }))
            },
            result
        );

        assert_eq!(v, from_grpc_value(&result).unwrap());
    }

    #[test]
    fn test_serialize_enum_tuple() {
        let v = JustAnEnum::ATupleVariant(55, false);
        let result = to_grpc_value(&v).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: vec![
                        (
                            "type".to_string(),
                            Value {
                                value_type: Some(ValueType::StringValue(
                                    "ATupleVariant".to_string()
                                ))
                            }
                        ),
                        (
                            "values".to_string(),
                            Value {
                                value_type: Some(ValueType::ArrayValue(ArrayValue {
                                    values: vec![
                                        Value {
                                            value_type: Some(ValueType::IntegerValue(55))
                                        },
                                        Value {
                                            value_type: Some(ValueType::BooleanValue(false))
                                        },
                                    ]
                                }))
                            }
                        ),
                    ]
                    .into_iter()
                    .collect()
                }))
            },
            result
        );

        assert_eq!(v, from_grpc_value(&result).unwrap());
    }

    #[test]
    fn serialize_tuple() {
        let v = (3, false, "okay".to_string());
        let result = to_grpc_value(&v).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::IntegerValue(3))
                        },
                        Value {
                            value_type: Some(ValueType::BooleanValue(false))
                        },
                        Value {
                            value_type: Some(ValueType::StringValue("okay".to_string()))
                        },
                    ]
                }))
            },
            result
        );

        assert_eq!(v, from_grpc_value(&result).unwrap());
    }

    #[test]
    fn serialize_slice() {
        let v = [9, 5, 1];
        let result = to_grpc_value(&v).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::IntegerValue(9))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(5))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(1))
                        },
                    ]
                }))
            },
            result
        );

        assert_eq!(v, from_grpc_value::<[u32; 3]>(&result).unwrap());
    }

    #[test]
    fn serialize_vector() {
        let v = vec![9, 5, 1];
        let result = to_grpc_value(&v).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::IntegerValue(9))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(5))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(1))
                        },
                    ]
                }))
            },
            result
        );

        assert_eq!(v, from_grpc_value::<Vec<u32>>(&result).unwrap());
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct JustATupleStruct(u32, bool, String);

    #[test]
    fn serialize_tuple_struct() {
        let v = JustATupleStruct(3, false, "okay".to_string());
        let result = to_grpc_value(&v).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::IntegerValue(3))
                        },
                        Value {
                            value_type: Some(ValueType::BooleanValue(false))
                        },
                        Value {
                            value_type: Some(ValueType::StringValue("okay".to_string()))
                        },
                    ]
                }))
            },
            result
        );

        assert_eq!(v, from_grpc_value(&result).unwrap());
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct ARecordStruct {
        an_int_field: u32,
        a_string_field: String,
        a_vec_field: Vec<bool>,
    }

    #[test]
    fn serialize_record_struct() {
        let v = ARecordStruct {
            an_int_field: 8,
            a_string_field: "blah".to_string(),
            a_vec_field: vec![false, false, true],
        };
        let result = to_grpc_value(&v).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: vec![
                        (
                            "an_int_field".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(8))
                            }
                        ),
                        (
                            "a_string_field".to_string(),
                            Value {
                                value_type: Some(ValueType::StringValue("blah".to_string()))
                            }
                        ),
                        (
                            "a_vec_field".to_string(),
                            Value {
                                value_type: Some(ValueType::ArrayValue(ArrayValue {
                                    values: vec![
                                        Value {
                                            value_type: Some(ValueType::BooleanValue(false))
                                        },
                                        Value {
                                            value_type: Some(ValueType::BooleanValue(false))
                                        },
                                        Value {
                                            value_type: Some(ValueType::BooleanValue(true))
                                        },
                                    ]
                                }))
                            }
                        ),
                    ]
                    .into_iter()
                    .collect()
                }))
            },
            result
        );

        assert_eq!(v, from_grpc_value(&result).unwrap());
    }

    #[test]
    fn serialize_map() {
        let map: HashMap<String, u32> = vec![("foo".to_string(), 5), ("bar".to_string(), 50)]
            .into_iter()
            .collect();

        let result = to_grpc_value(&map).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: vec![
                        (
                            "foo".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(5))
                            }
                        ),
                        (
                            "bar".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(50))
                            }
                        ),
                    ]
                    .into_iter()
                    .collect()
                }))
            },
            result
        );

        assert_eq!(map, from_grpc_value(&result).unwrap());
    }

    #[derive(Serialize, PartialEq, Debug, Deserialize)]
    struct ANewtypeStruct(u32);

    #[test]
    fn serialize_newtype_struct() {
        let record = ANewtypeStruct(55);

        let result = to_grpc_value(&record).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::IntegerValue(55))
            },
            result
        );

        assert_eq!(record, from_grpc_value(&result).unwrap());
    }
}
