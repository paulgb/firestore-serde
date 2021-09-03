use crate::firestore::Value;
use serde::de;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum DeserializationError {
    Message(String),
    WrongType(&'static str, Value),
    IntRange(&'static str, i64),
    MissingField(&'static str),
    Unrepresentable(&'static str),
}

impl Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializationError::Message(s) => writeln!(f, "{}", s),
            DeserializationError::MissingField(field) => {
                writeln!(f, "Expected field {} in map, but didn't find it.", field)
            }
            DeserializationError::WrongType(expected, got) => writeln!(
                f,
                "Tried to deserialize into {}, but got {:?}.",
                expected, got
            ),
            DeserializationError::IntRange(typ, val) => writeln!(
                f,
                "Tried to convert to {}, but value {} is out of range.",
                typ, val
            ),
            DeserializationError::Unrepresentable(typ) => {
                writeln!(f, "Tried to deserialize {}, which is unrepresentable.", typ)
            }
        }
    }
}

impl std::error::Error for DeserializationError {}

impl de::Error for DeserializationError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DeserializationError::Message(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, DeserializationError>;
