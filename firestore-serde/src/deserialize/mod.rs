use crate::firestore::{value::ValueType, ArrayValue, MapValue, Value};
pub use error::{DeserializationError, Result};
use prost::Message;
use serde::{
    de::{EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess},
    Deserializer,
};
use std::convert::TryFrom;

use crate::{TYPE, VALUE, VALUES};

use self::{
    plain_byte_deserializer::PlainByteDeserializer,
    plain_string_deserializer::PlainStringDeserializer,
};

mod error;
mod plain_byte_deserializer;
mod plain_string_deserializer;

pub struct ValueDeserializer<'de>(pub &'de Value);

struct ArrayValueSeq<'de> {
    values: std::slice::Iter<'de, Value>,
}

impl<'de> ArrayValueSeq<'de> {
    pub fn new(values: std::slice::Iter<'de, Value>) -> Self {
        ArrayValueSeq { values }
    }
}

impl<'de> SeqAccess<'de> for ArrayValueSeq<'de> {
    type Error = DeserializationError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let Some(v) = self.values.next() {
            seed.deserialize(&mut ValueDeserializer(v)).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct BytesSeq<'de> {
    bytes: core::slice::Iter<'de, u8>,
}

impl<'de> BytesSeq<'de> {
    pub fn new(bytes: core::slice::Iter<'de, u8>) -> Self {
        BytesSeq { bytes }
    }
}

impl<'de> SeqAccess<'de> for BytesSeq<'de> {
    type Error = DeserializationError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let Some(v) = self.bytes.next() {
            seed.deserialize(PlainByteDeserializer(*v)).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct MapValueSeq<'de> {
    values: std::collections::hash_map::Iter<'de, String, Value>,
    next_value: Option<&'de Value>,
}

impl<'de> MapValueSeq<'de> {
    pub fn new(values: std::collections::hash_map::Iter<'de, String, Value>) -> Self {
        MapValueSeq {
            values,
            next_value: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapValueSeq<'de> {
    type Error = DeserializationError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if let Some((k, v)) = self.values.next() {
            self.next_value = Some(v);

            Ok(Some(seed.deserialize(PlainStringDeserializer(k))?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let value = self
            .next_value
            .take()
            .expect("Shouldn't visit value before key.");
        seed.deserialize(&mut ValueDeserializer(value))
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut ValueDeserializer<'de> {
    type Error = DeserializationError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeserializationError::Unrepresentable("any"))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::BooleanValue(v)),
        } = self.0
        {
            visitor.visit_bool(*v)
        } else {
            Err(DeserializationError::WrongType("bool", self.0.clone()))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::IntegerValue(v)),
        } = self.0
        {
            visitor
                .visit_i8(i8::try_from(*v).map_err(|_| DeserializationError::IntRange("i8", *v))?)
        } else {
            Err(DeserializationError::WrongType("i8", self.0.clone()))
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::IntegerValue(v)),
        } = self.0
        {
            visitor.visit_i16(
                i16::try_from(*v).map_err(|_| DeserializationError::IntRange("i16", *v))?,
            )
        } else {
            Err(DeserializationError::WrongType("i16", self.0.clone()))
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::IntegerValue(v)),
        } = self.0
        {
            visitor.visit_i32(
                i32::try_from(*v).map_err(|_| DeserializationError::IntRange("i32", *v))?,
            )
        } else {
            Err(DeserializationError::WrongType("i32", self.0.clone()))
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::IntegerValue(v)),
        } = self.0
        {
            visitor.visit_i64(*v)
        } else {
            Err(DeserializationError::WrongType("i64", self.0.clone()))
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::IntegerValue(v)),
        } = self.0
        {
            visitor
                .visit_u8(u8::try_from(*v).map_err(|_| DeserializationError::IntRange("u8", *v))?)
        } else {
            Err(DeserializationError::WrongType("i8", self.0.clone()))
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::IntegerValue(v)),
        } = self.0
        {
            visitor.visit_u16(
                u16::try_from(*v).map_err(|_| DeserializationError::IntRange("u16", *v))?,
            )
        } else {
            Err(DeserializationError::WrongType("u16", self.0.clone()))
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::IntegerValue(v)),
        } = self.0
        {
            visitor.visit_u32(
                u32::try_from(*v).map_err(|_| DeserializationError::IntRange("u32", *v))?,
            )
        } else {
            Err(DeserializationError::WrongType("u32", self.0.clone()))
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::IntegerValue(v)),
        } = self.0
        {
            visitor.visit_u64(
                u64::try_from(*v).map_err(|_| DeserializationError::IntRange("u64", *v))?,
            )
        } else {
            Err(DeserializationError::WrongType("u64", self.0.clone()))
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::DoubleValue(v)),
        } = self.0
        {
            #[allow(clippy::cast_possible_truncation)]
            visitor.visit_f32(*v as f32)
        } else {
            Err(DeserializationError::WrongType("f32", self.0.clone()))
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::DoubleValue(v)),
        } = self.0
        {
            visitor.visit_f64(*v)
        } else {
            Err(DeserializationError::WrongType("f64", self.0.clone()))
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::StringValue(v)),
        } = self.0
        {
            if v.len() == 1 {
                visitor.visit_char(
                    v.chars()
                        .next()
                        .expect("Already checked that string has exactly one char."),
                )
            } else {
                Err(DeserializationError::WrongType("char", self.0.clone()))
            }
        } else {
            Err(DeserializationError::WrongType("char", self.0.clone()))
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::StringValue(v)),
        } = self.0
        {
            visitor.visit_str(v)
        } else {
            Err(DeserializationError::WrongType("str", self.0.clone()))
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::StringValue(v)),
        } = self.0
        {
            visitor.visit_string(v.clone())
        } else {
            Err(DeserializationError::WrongType("string", self.0.clone()))
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::BytesValue(bytes)),
        } = self.0
        {
            visitor.visit_bytes(bytes)
        } else {
            Err(DeserializationError::WrongType("bytes", self.0.clone()))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::BytesValue(bytes)),
        } = self.0
        {
            visitor.visit_byte_buf(bytes.clone())
        } else if let Value {
            value_type: Some(ValueType::TimestampValue(timestamp)),
        } = self.0
        {
            let bytes = timestamp.encode_to_vec();
            visitor.visit_byte_buf(bytes)
        } else {
            Err(DeserializationError::WrongType("byte_buf", self.0.clone()))
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::NullValue(_)),
        } = self.0
        {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeserializationError::Unrepresentable("unit"))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeserializationError::Unrepresentable("unit_struct"))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::ArrayValue(ArrayValue { values })),
        } = self.0
        {
            visitor.visit_seq(ArrayValueSeq::new(values.iter()))
        } else if let Value {
            value_type: Some(ValueType::BytesValue(bytes)),
        } = self.0
        {
            visitor.visit_seq(BytesSeq::new(bytes.iter()))
        } else {
            Err(DeserializationError::WrongType("seq", self.0.clone()))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::ArrayValue(ArrayValue { values })),
        } = self.0
        {
            visitor.visit_seq(ArrayValueSeq::new(values.iter()))
        } else {
            Err(DeserializationError::WrongType("tuple", self.0.clone()))
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Value {
            value_type: Some(ValueType::MapValue(MapValue { fields })),
        } = self.0
        {
            visitor.visit_map(MapValueSeq::new(fields.iter()))
        } else {
            Err(DeserializationError::WrongType("map", self.0.clone()))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match &self.0.value_type {
            Some(ValueType::StringValue(v)) => visitor.visit_enum(v.clone().into_deserializer()),
            Some(ValueType::MapValue(MapValue { fields })) => {
                let mut typ: Option<&String> = None;
                let mut value: Option<&Value> = None;

                for (k, v) in fields {
                    if k == TYPE {
                        if let Value {
                            value_type: Some(ValueType::StringValue(v)),
                        } = v
                        {
                            typ = Some(v);
                        } else {
                            return Err(DeserializationError::WrongType("string", v.clone()));
                        }
                    } else if k == VALUE || k == VALUES {
                        value = Some(v);
                    }
                }

                let typ = if let Some(typ) = typ {
                    typ
                } else {
                    return Err(DeserializationError::MissingField(TYPE));
                };

                if let Some(value) = value {
                    visitor.visit_enum(Enum::new(typ, value))
                } else {
                    Err(DeserializationError::MissingField(VALUE))
                }
            }
            _ => Err(DeserializationError::WrongType("enum", self.0.clone())),
        }
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeserializationError::Unrepresentable("identifier"))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct Enum<'de> {
    typ: &'de str,
    value: &'de Value,
}

impl<'de> Enum<'de> {
    pub fn new(typ: &'de str, value: &'de Value) -> Self {
        Enum { typ, value }
    }
}

impl<'de> EnumAccess<'de> for Enum<'de> {
    type Error = DeserializationError;

    type Variant = Enum<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let val = seed.deserialize(PlainStringDeserializer(self.typ))?;

        Ok((val, self))
    }
}

impl<'de> VariantAccess<'de> for Enum<'de> {
    type Error = DeserializationError;

    fn unit_variant(self) -> Result<()> {
        panic!("Unit variant was already handled.")
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut ValueDeserializer(self.value))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        ValueDeserializer(self.value).deserialize_seq(visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        ValueDeserializer(self.value).deserialize_map(visitor)
    }
}
