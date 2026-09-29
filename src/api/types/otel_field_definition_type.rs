pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OtelFieldDefinitionType {
    Enum,
    Text,
    Int,
    Float,
    Bool,
    Timestamp,
    Array,
    Map,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OtelFieldDefinitionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Enum => serializer.serialize_str("ENUM"),
            Self::Text => serializer.serialize_str("TEXT"),
            Self::Int => serializer.serialize_str("INT"),
            Self::Float => serializer.serialize_str("FLOAT"),
            Self::Bool => serializer.serialize_str("BOOL"),
            Self::Timestamp => serializer.serialize_str("TIMESTAMP"),
            Self::Array => serializer.serialize_str("ARRAY"),
            Self::Map => serializer.serialize_str("MAP"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OtelFieldDefinitionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ENUM" => Ok(Self::Enum),
            "TEXT" => Ok(Self::Text),
            "INT" => Ok(Self::Int),
            "FLOAT" => Ok(Self::Float),
            "BOOL" => Ok(Self::Bool),
            "TIMESTAMP" => Ok(Self::Timestamp),
            "ARRAY" => Ok(Self::Array),
            "MAP" => Ok(Self::Map),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OtelFieldDefinitionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Enum => write!(f, "ENUM"),
            Self::Text => write!(f, "TEXT"),
            Self::Int => write!(f, "INT"),
            Self::Float => write!(f, "FLOAT"),
            Self::Bool => write!(f, "BOOL"),
            Self::Timestamp => write!(f, "TIMESTAMP"),
            Self::Array => write!(f, "ARRAY"),
            Self::Map => write!(f, "MAP"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
