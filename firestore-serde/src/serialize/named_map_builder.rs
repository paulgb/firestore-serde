use super::error::{Result, SerializationError};
use crate::firestore::{value::ValueType, MapValue, Value};
use crate::ValueSerializer;
use crate::{TYPE, VALUES};
use serde::{ser::SerializeStructVariant, Serialize};
use std::collections::HashMap;

pub struct NamedMapBuilder {
    name: String,
    fields: HashMap<String, Value>,
}

impl NamedMapBuilder {
    pub fn with_capacity(name: String, capacity: usize) -> Self {
        NamedMapBuilder {
            name,
            fields: HashMap::with_capacity(capacity),
        }
    }
}

impl<'a> SerializeStructVariant for NamedMapBuilder {
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
                fields: vec![
                    (TYPE.to_string(), self.name.serialize(ValueSerializer)?),
                    (
                        VALUES.to_string(),
                        Value {
                            value_type: Some(ValueType::MapValue(MapValue {
                                fields: self.fields,
                            })),
                        },
                    ),
                ]
                .into_iter()
                .collect(),
            })),
        })
    }

    fn skip_field(&mut self, key: &'static str) -> Result<()> {
        let _ = key;
        Ok(())
    }
}
