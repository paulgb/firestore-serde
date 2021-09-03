use crate::firestore::{value::ValueType, ArrayValue, MapValue, Value};
use serde::{ser::SerializeTupleVariant, Serialize};

use crate::ValueSerializer;

use super::error::{Result, SerializationError};

use crate::{TYPE, VALUES};
pub struct NamedArrayBuilder {
    name: String,
    values: Vec<Value>,
}

impl NamedArrayBuilder {
    pub fn with_capacity(name: String, capacity: usize) -> Self {
        NamedArrayBuilder {
            name,
            values: Vec::with_capacity(capacity),
        }
    }
}

impl<'a> SerializeTupleVariant for NamedArrayBuilder {
    type Ok = Value;

    type Error = SerializationError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.values.push(value.serialize(ValueSerializer)?);

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
                            value_type: Some(ValueType::ArrayValue(ArrayValue {
                                values: self.values,
                            })),
                        },
                    ),
                ]
                .into_iter()
                .collect(),
            })),
        })
    }
}
