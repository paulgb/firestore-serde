use serde::ser;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum SerializationError {
    Message(String),
    OutsideIntRange(u64),
    Unrepresentable(String),
    NotAMap,
    NonStringKey,
}

impl Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Message(s) => writeln!(f, "{}", s),
            Self::OutsideIntRange(v) => writeln!(f, "Attempted to convert a u64 ({}) that falls outside of the i64 representable range.", v),
            Self::Unrepresentable(t) => writeln!(f, "Attempted to convert an unrepresentable type: {}", t),
            Self::NonStringKey => writeln!(f, "Attempted to use a non-string key in a map."),
            Self::NotAMap => writeln!(f, "Only types that convert to a map can be stored in a Document."),
        }
    }
}

impl std::error::Error for SerializationError {}

impl ser::Error for SerializationError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SerializationError::Message(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, SerializationError>;
