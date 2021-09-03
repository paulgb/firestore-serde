use super::SerializationError;
use crate::serialize::Result;
use googapis::google::firestore::v1::{value::ValueType, Value};
use prost::Message;
use prost_types::Timestamp;
use serde::{Serialize, Serializer};

pub struct TimestampSerializer;

const PANIC_MESSAGE: &str = "TimestampSerializer should never be called with anything but bytes.";

impl Serializer for TimestampSerializer {
    type Ok = Value;

    type Error = SerializationError;

    type SerializeMap = serde::ser::Impossible<Value, SerializationError>;
    type SerializeSeq = serde::ser::Impossible<Value, SerializationError>;
    type SerializeStruct = serde::ser::Impossible<Value, SerializationError>;
    type SerializeStructVariant = serde::ser::Impossible<Value, SerializationError>;
    type SerializeTuple = serde::ser::Impossible<Value, SerializationError>;
    type SerializeTupleStruct = serde::ser::Impossible<Value, SerializationError>;
    type SerializeTupleVariant = serde::ser::Impossible<Value, SerializationError>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        let timestamp = Timestamp::decode(v).expect(
            "Should always be able to decode timestamp, since we encoded it immediately before.",
        );

        Ok(Value {
            value_type: Some(ValueType::TimestampValue(timestamp)),
        })
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_some<T: ?Sized>(self, _v: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        panic!("{}", PANIC_MESSAGE);
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        panic!("{}", PANIC_MESSAGE);
    }
}
