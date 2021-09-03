use super::error::{Result, SerializationError};
use crate::firestore::{value::ValueType, MapValue, Value};
use crate::ValueSerializer;
use serde::{ser::SerializeStruct, Serialize};
use std::collections::HashMap;

pub struct MapBuilder {
    fields: HashMap<String, Value>,
}

impl MapBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        MapBuilder {
            fields: HashMap::with_capacity(capacity),
        }
    }
}

impl<'a> SerializeStruct for MapBuilder {
    type Ok = Value;

    type Error = SerializationError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.fields
            .insert(key.to_string(), value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Value {
            value_type: Some(ValueType::MapValue(MapValue {
                fields: self.fields,
            })),
        })
    }

    fn skip_field(&mut self, key: &'static str) -> Result<()> {
        let _ = key;
        Ok(())
    }
}
