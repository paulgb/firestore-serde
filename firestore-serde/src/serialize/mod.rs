pub use self::error::{Result, SerializationError};
use self::timestamp_serializer::TimestampSerializer;
use self::{
    array_builder::ArrayBuilder, kv_map_builder::KVMapBuilder, map_builder::MapBuilder,
    named_array_builder::NamedArrayBuilder, named_map_builder::NamedMapBuilder,
};
use crate::firestore::{value::ValueType, MapValue, Value};
use crate::{DATE_MAGIC, TYPE, VALUE};
use serde::{Serialize, Serializer};
use std::convert::TryFrom;

mod array_builder;
mod error;
mod kv_map_builder;
mod map_builder;
mod named_array_builder;
mod named_map_builder;
mod timestamp_serializer;

pub struct ValueSerializer;

impl Serializer for ValueSerializer {
    type Ok = Value;

    type Error = SerializationError;

    type SerializeMap = KVMapBuilder;
    type SerializeSeq = ArrayBuilder;
    type SerializeStruct = MapBuilder;
    type SerializeStructVariant = NamedMapBuilder;
    type SerializeTuple = ArrayBuilder;
    type SerializeTupleStruct = ArrayBuilder;
    type SerializeTupleVariant = NamedArrayBuilder;

    fn serialize_bool(self, v: bool) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::BooleanValue(v)),
        })
    }

    fn serialize_i8(self, v: i8) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(i64::from(v))),
        })
    }

    fn serialize_i16(self, v: i16) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(i64::from(v))),
        })
    }

    fn serialize_i32(self, v: i32) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(i64::from(v))),
        })
    }

    fn serialize_i64(self, v: i64) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(v)),
        })
    }

    fn serialize_u8(self, v: u8) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(i64::from(v))),
        })
    }

    fn serialize_u16(self, v: u16) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(i64::from(v))),
        })
    }

    fn serialize_u32(self, v: u32) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(i64::from(v))),
        })
    }

    fn serialize_u64(self, v: u64) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(
                i64::try_from(v).map_err(|_| SerializationError::OutsideIntRange(v))?,
            )),
        })
    }

    fn serialize_f32(self, v: f32) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::DoubleValue(f64::from(v))),
        })
    }

    fn serialize_f64(self, v: f64) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::DoubleValue(v)),
        })
    }

    fn serialize_char(self, v: char) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::StringValue(v.to_string())),
        })
    }

    fn serialize_str(self, v: &str) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::StringValue(v.to_string())),
        })
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::BytesValue(v.into())),
        })
    }

    fn serialize_none(self) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::NullValue(0)),
        })
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Value>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Value> {
        Err(SerializationError::Unrepresentable("()".to_string()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        Err(SerializationError::Unrepresentable(
            "unit_struct".to_string(),
        ))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        Ok(Value {
            value_type: Some(ValueType::StringValue(variant.to_string())),
        })
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Value>
    where
        T: Serialize,
    {
        if name == DATE_MAGIC {
            value.serialize(TimestampSerializer)
        } else {
            value.serialize(self)
        }
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value>
    where
        T: Serialize,
    {
        Ok(Value {
            value_type: Some(ValueType::MapValue(MapValue {
                fields: vec![
                    (TYPE.to_string(), variant.serialize(ValueSerializer)?),
                    (VALUE.to_string(), value.serialize(ValueSerializer)?),
                ]
                .into_iter()
                .collect(),
            })),
        })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<ArrayBuilder> {
        let builder = if let Some(len) = len {
            ArrayBuilder::with_capacity(len)
        } else {
            ArrayBuilder::default()
        };

        Ok(builder)
    }

    fn serialize_tuple(self, len: usize) -> Result<ArrayBuilder> {
        Ok(ArrayBuilder::with_capacity(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<ArrayBuilder> {
        Ok(ArrayBuilder::with_capacity(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<NamedArrayBuilder> {
        Ok(NamedArrayBuilder::with_capacity(variant.to_string(), len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<KVMapBuilder> {
        if let Some(len) = len {
            Ok(KVMapBuilder::with_capacity(len))
        } else {
            Ok(KVMapBuilder::default())
        }
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<MapBuilder> {
        Ok(MapBuilder::with_capacity(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<NamedMapBuilder> {
        Ok(NamedMapBuilder::with_capacity(variant.to_string(), len))
    }
}
