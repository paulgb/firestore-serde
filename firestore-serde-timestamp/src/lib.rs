pub mod timestamp {
    use chrono::{DateTime, TimeZone, Timelike, Utc};
    use prost::Message;
    use prost_types::Timestamp;
    use serde::{Deserialize, Deserializer, Serializer};
    use serde_bytes::ByteBuf;

    pub const DATE_MAGIC: &str = "$TimestampValue";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[allow(clippy::cast_possible_wrap)]
        let c = Timestamp {
            seconds: date.timestamp(),
            nanos: date.nanosecond() as i32,
        };

        let v = ByteBuf::from(c.encode_to_vec());

        serializer.serialize_newtype_struct(DATE_MAGIC, &v)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = ByteBuf::deserialize(deserializer)?;
        let timestamp = Timestamp::decode(buf.as_slice())
            .expect("Should always be able to decode timestamp because we just encoded it.");
        #[allow(clippy::cast_sign_loss)]
        let datetime = Utc.timestamp(timestamp.seconds, timestamp.nanos as u32);

        Ok(datetime)
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc, TimeZone};
    use firestore_serde::{from_grpc_value, to_grpc_value, ValueDeserializer, ValueSerializer};
    use googapis::google::firestore::v1::{value::ValueType, MapValue, Value};
    use prost_types::Timestamp;
    use serde::{Deserialize, Serialize};

    use crate::timestamp::{deserialize, serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct StructWithDate {
        #[serde(with = "crate::timestamp")]
        date: DateTime<Utc>,
    }

    #[test]
    fn test_serialize_date() {
        let date = Utc.timestamp(150, 200);

        let result = serialize(&date, ValueSerializer).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::TimestampValue(Timestamp {
                    seconds: 150,
                    nanos: 200
                }))
            },
            result
        );

        assert_eq!(date, deserialize(&mut ValueDeserializer(&result)).unwrap());
    }

    #[test]
    fn test_serialize_in_struct() {
        let st = StructWithDate {
            date: Utc.timestamp(150, 200),
        };

        let result = to_grpc_value(&st).unwrap();

        assert_eq!(
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: vec![(
                        "date".to_string(),
                        Value {
                            value_type: Some(ValueType::TimestampValue(Timestamp {
                                seconds: 150,
                                nanos: 200
                            }))
                        }
                    )]
                    .into_iter()
                    .collect()
                }))
            },
            result
        );

        assert_eq!(st, from_grpc_value(&result).unwrap());
    }
}
