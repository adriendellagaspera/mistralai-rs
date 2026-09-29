pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionConfigType {
    Mcp,
    Turbine,
    Eolienne,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectionConfigType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mcp => serializer.serialize_str("mcp"),
            Self::Turbine => serializer.serialize_str("turbine"),
            Self::Eolienne => serializer.serialize_str("eolienne"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectionConfigType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mcp" => Ok(Self::Mcp),
            "turbine" => Ok(Self::Turbine),
            "eolienne" => Ok(Self::Eolienne),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectionConfigType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mcp => write!(f, "mcp"),
            Self::Turbine => write!(f, "turbine"),
            Self::Eolienne => write!(f, "eolienne"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
