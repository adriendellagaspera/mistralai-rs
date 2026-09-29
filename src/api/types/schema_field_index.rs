pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchemaFieldIndex {
    Ann,
    Bm25,
    Attribute,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SchemaFieldIndex {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ann => serializer.serialize_str("ann"),
            Self::Bm25 => serializer.serialize_str("bm25"),
            Self::Attribute => serializer.serialize_str("attribute"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SchemaFieldIndex {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ann" => Ok(Self::Ann),
            "bm25" => Ok(Self::Bm25),
            "attribute" => Ok(Self::Attribute),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SchemaFieldIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ann => write!(f, "ann"),
            Self::Bm25 => write!(f, "bm25"),
            Self::Attribute => write!(f, "attribute"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
