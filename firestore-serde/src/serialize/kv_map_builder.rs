use super::error::{Result, SerializationError};
use crate::firestore::{value::ValueType, MapValue, Value};
use crate::ValueSerializer;
use serde::{ser::SerializeMap, Serialize};
use std::collections::HashMap;

#[derive(Default)]
pub struct KVMapBuilder {
    key: Option<String>,
    fields: HashMap<String, Value>,
}

impl KVMapBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        KVMapBuilder {
            key: None,
            fields: HashMap::with_capacity(capacity),
        }
    }
}

impl<'a> SerializeMap for KVMapBuilder {
    type Ok = Value;

    type Error = SerializationError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        let result = key.serialize(ValueSerializer)?;

        if let Value {
            value_type: Some(ValueType::StringValue(v)),
        } = result
        {
            self.key = Some(v);
        } else {
            return Err(SerializationError::NonStringKey);
        }

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let key = self
            .key
            .take()
            .expect("Should never attempt to serialize a value without having seen a key.");

        self.fields.insert(key, value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Value {
            value_type: Some(ValueType::MapValue(MapValue {
                fields: self.fields,
            })),
        })
    }
}
