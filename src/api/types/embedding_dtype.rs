pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmbeddingDtype {
    Float,
    Int8,
    Uint8,
    Binary,
    Ubinary,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EmbeddingDtype {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Float => serializer.serialize_str("float"),
            Self::Int8 => serializer.serialize_str("int8"),
            Self::Uint8 => serializer.serialize_str("uint8"),
            Self::Binary => serializer.serialize_str("binary"),
            Self::Ubinary => serializer.serialize_str("ubinary"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EmbeddingDtype {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "float" => Ok(Self::Float),
            "int8" => Ok(Self::Int8),
            "uint8" => Ok(Self::Uint8),
            "binary" => Ok(Self::Binary),
            "ubinary" => Ok(Self::Ubinary),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EmbeddingDtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float => write!(f, "float"),
            Self::Int8 => write!(f, "int8"),
            Self::Uint8 => write!(f, "uint8"),
            Self::Binary => write!(f, "binary"),
            Self::Ubinary => write!(f, "ubinary"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
