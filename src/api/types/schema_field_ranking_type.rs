pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchemaFieldRankingType {
    Count,
    Embedding,
    Timestamp,
    Text,
    String_,
    Bool,
    Int,
    Language,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SchemaFieldRankingType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Count => serializer.serialize_str("count"),
            Self::Embedding => serializer.serialize_str("embedding"),
            Self::Timestamp => serializer.serialize_str("timestamp"),
            Self::Text => serializer.serialize_str("text"),
            Self::String_ => serializer.serialize_str("string"),
            Self::Bool => serializer.serialize_str("bool"),
            Self::Int => serializer.serialize_str("int"),
            Self::Language => serializer.serialize_str("language"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SchemaFieldRankingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "count" => Ok(Self::Count),
            "embedding" => Ok(Self::Embedding),
            "timestamp" => Ok(Self::Timestamp),
            "text" => Ok(Self::Text),
            "string" => Ok(Self::String_),
            "bool" => Ok(Self::Bool),
            "int" => Ok(Self::Int),
            "language" => Ok(Self::Language),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SchemaFieldRankingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count => write!(f, "count"),
            Self::Embedding => write!(f, "embedding"),
            Self::Timestamp => write!(f, "timestamp"),
            Self::Text => write!(f, "text"),
            Self::String_ => write!(f, "string"),
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Language => write!(f, "language"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
