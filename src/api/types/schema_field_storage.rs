pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchemaFieldStorage {
    InMemory,
    OnDisk,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SchemaFieldStorage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InMemory => serializer.serialize_str("in_memory"),
            Self::OnDisk => serializer.serialize_str("on_disk"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SchemaFieldStorage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "in_memory" => Ok(Self::InMemory),
            "on_disk" => Ok(Self::OnDisk),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SchemaFieldStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InMemory => write!(f, "in_memory"),
            Self::OnDisk => write!(f, "on_disk"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
