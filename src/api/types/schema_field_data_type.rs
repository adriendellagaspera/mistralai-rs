pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchemaFieldDataType {
    Int,
    Bool,
    String_,
    Embedding,
    Long,
    Float,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SchemaFieldDataType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Int => serializer.serialize_str("int"),
            Self::Bool => serializer.serialize_str("bool"),
            Self::String_ => serializer.serialize_str("string"),
            Self::Embedding => serializer.serialize_str("embedding"),
            Self::Long => serializer.serialize_str("long"),
            Self::Float => serializer.serialize_str("float"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SchemaFieldDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "int" => Ok(Self::Int),
            "bool" => Ok(Self::Bool),
            "string" => Ok(Self::String_),
            "embedding" => Ok(Self::Embedding),
            "long" => Ok(Self::Long),
            "float" => Ok(Self::Float),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SchemaFieldDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Bool => write!(f, "bool"),
            Self::String_ => write!(f, "string"),
            Self::Embedding => write!(f, "embedding"),
            Self::Long => write!(f, "long"),
            Self::Float => write!(f, "float"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
