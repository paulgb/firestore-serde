use super::error::{Result, SerializationError};
use crate::firestore::{value::ValueType, ArrayValue, Value};
use crate::ValueSerializer;
use serde::{
    ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct},
    Serialize,
};

#[derive(Default)]
pub struct ArrayBuilder {
    values: Vec<Value>,
}

impl ArrayBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        ArrayBuilder {
            values: Vec::with_capacity(capacity),
        }
    }
}

impl<'a> SerializeTuple for ArrayBuilder {
    type Ok = Value;

    type Error = SerializationError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.values.push(value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Value {
            value_type: Some(ValueType::ArrayValue(ArrayValue {
                values: self.values,
            })),
        })
    }
}

impl<'a> SerializeTupleStruct for ArrayBuilder {
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
            value_type: Some(ValueType::ArrayValue(ArrayValue {
                values: self.values,
            })),
        })
    }
}

impl<'a> SerializeSeq for ArrayBuilder {
    type Ok = Value;

    type Error = SerializationError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.values.push(value.serialize(ValueSerializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Value {
            value_type: Some(ValueType::ArrayValue(ArrayValue {
                values: self.values,
            })),
        })
    }
}
